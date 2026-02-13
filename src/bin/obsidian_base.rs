use forge_obsidian::base::{self, FilterEntry, FilterNode};
use forge_obsidian::eval::{self, ThisContext};
use forge_obsidian::note::{self, NoteContext};
use forge_obsidian::vault;
use serde_json::json;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

struct Args {
    base_path: PathBuf,
    view_filter: Option<String>,
    paths_only: bool,
}

fn parse_args() -> Result<Args, ExitCode> {
    let args: Vec<String> = env::args().collect();
    let mut base_path = None;
    let mut view_filter = None;
    let mut paths_only = false;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--view" => {
                i += 1;
                view_filter = args.get(i).cloned();
            }
            "--paths" => paths_only = true,
            "--help" | "-h" => {
                print_usage();
                return Err(ExitCode::SUCCESS);
            }
            arg if !arg.starts_with('-') => base_path = Some(arg.to_owned()),
            other => {
                eprintln!("Unknown option: {other}");
                print_usage();
                return Err(ExitCode::from(1));
            }
        }
        i += 1;
    }

    let Some(path_str) = base_path else {
        eprintln!("Error: no .base file specified");
        print_usage();
        return Err(ExitCode::from(1));
    };

    let base_path = std::fs::canonicalize(&path_str).map_err(|e| {
        eprintln!("Cannot resolve {path_str}: {e}");
        ExitCode::from(1)
    })?;

    Ok(Args {
        base_path,
        view_filter,
        paths_only,
    })
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(code) => return code,
    };

    let Some(vault_root) = vault::find_vault(args.base_path.to_str().unwrap_or("")) else {
        eprintln!("Cannot find vault root (no .obsidian/ directory in parent chain)");
        return ExitCode::from(1);
    };

    let spec = match base::parse_file(&args.base_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(1);
        }
    };

    let notes = note::walk_vault(&vault_root);
    let this_ctx = ThisContext::from_base_path(&vault_root, &args.base_path);

    // Pre-filter with top-level filters
    let notes: Vec<&NoteContext> = notes
        .iter()
        .filter(|n| match &spec.filters {
            Some(f) => matches_filter(f, n, &this_ctx),
            None => true,
        })
        .collect();

    // Process each view
    for view in &spec.views {
        if let Some(ref name) = args.view_filter {
            if !view.name.eq_ignore_ascii_case(name) {
                continue;
            }
        }

        let mut matched: Vec<&NoteContext> = notes
            .iter()
            .filter(|n| match &view.filters {
                Some(f) => matches_filter(f, n, &this_ctx),
                None => true,
            })
            .copied()
            .collect();

        // Apply sort
        for sort_spec in view.sort.iter().rev() {
            let prop = &sort_spec.property;
            let desc = sort_spec.direction == base::SortDirection::Desc;
            matched.sort_by(|a, b| {
                let cmp = get_sort_key(a, prop).cmp(&get_sort_key(b, prop));
                if desc { cmp.reverse() } else { cmp }
            });
        }

        for matched_note in &matched {
            if args.paths_only {
                println!("{}", matched_note.rel_path);
            } else {
                print_jsonl(view, matched_note);
            }
        }
    }

    ExitCode::SUCCESS
}

fn print_jsonl(view: &base::ViewSpec, ctx: &NoteContext) {
    let mut obj = serde_json::Map::new();
    obj.insert("view".into(), json!(view.name));
    obj.insert("file".into(), json!(ctx.rel_path));
    obj.insert("name".into(), json!(ctx.name));

    for col in &view.order {
        let key = col
            .strip_prefix("file.")
            .or_else(|| col.strip_prefix("property."))
            .or_else(|| col.strip_prefix("note."))
            .unwrap_or(col);
        if key == "name" || obj.contains_key(key) {
            continue;
        }
        let val = get_display_value(ctx, col);
        if !val.is_null() {
            obj.insert(key.to_owned(), val);
        }
    }

    println!("{}", serde_json::to_string(&obj).unwrap_or_default());
}

fn matches_filter(filter: &FilterNode, ctx: &NoteContext, this_ctx: &ThisContext) -> bool {
    match filter {
        FilterNode::And(entries) => entries.iter().all(|e| matches_entry(e, ctx, this_ctx)),
        FilterNode::Or(entries) => entries.iter().any(|e| matches_entry(e, ctx, this_ctx)),
    }
}

fn matches_entry(entry: &FilterEntry, ctx: &NoteContext, this_ctx: &ThisContext) -> bool {
    match entry {
        FilterEntry::Expr(s) => eval::eval_filter(s, ctx, this_ctx),
        FilterEntry::Nested(filter) => matches_filter(filter, ctx, this_ctx),
    }
}

fn get_sort_key(ctx: &NoteContext, prop: &str) -> String {
    match prop {
        "file.name" => ctx.name.clone(),
        "file.path" => ctx.rel_path.clone(),
        "file.tags" | "property.tags" => ctx.tags.join(", "),
        p => {
            let key = p
                .strip_prefix("property.")
                .or_else(|| p.strip_prefix("note."))
                .unwrap_or(p);
            ctx.get_property(key)
                .map(|v| match v {
                    serde_yaml::Value::String(s) => s.clone(),
                    other => serde_yaml::to_string(other).unwrap_or_default(),
                })
                .unwrap_or_default()
        }
    }
}

fn get_display_value(ctx: &NoteContext, col: &str) -> serde_json::Value {
    match col {
        "file.name" => json!(ctx.name),
        "file.path" | "file.fullname" => json!(ctx.rel_path),
        "file.ext" => json!(ctx.ext),
        "file.folder" => json!(ctx.folder),
        "file.tags" | "tags" => json!(ctx.tags),
        "file.links" => json!(ctx.links),
        c => {
            let key = c
                .strip_prefix("property.")
                .or_else(|| c.strip_prefix("note."))
                .unwrap_or(c);
            match ctx.get_property(key) {
                Some(serde_yaml::Value::String(s)) => json!(s),
                Some(serde_yaml::Value::Bool(b)) => json!(b),
                Some(serde_yaml::Value::Number(n)) => {
                    if let Some(i) = n.as_i64() {
                        json!(i)
                    } else if let Some(f) = n.as_f64() {
                        json!(f)
                    } else {
                        json!(n.to_string())
                    }
                }
                Some(serde_yaml::Value::Sequence(seq)) => {
                    let items: Vec<&str> = seq.iter().filter_map(|v| v.as_str()).collect();
                    json!(items)
                }
                Some(serde_yaml::Value::Null) | None => serde_json::Value::Null,
                Some(other) => json!(serde_yaml::to_string(other).unwrap_or_default().trim()),
            }
        }
    }
}

fn print_usage() {
    eprintln!("Usage: obsidian-base <file.base> [--view <name>] [--paths]");
    eprintln!();
    eprintln!("Resolve an Obsidian Base file against its vault.");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --view <name>  Only resolve the named view");
    eprintln!("  --paths        Output file paths only (one per line)");
    eprintln!("  -h, --help     Show this help");
}
