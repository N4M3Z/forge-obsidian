#[cfg(test)]
mod tests;

use crate::note::NoteContext;
use std::path::Path;

// ─── Values ─────────────────────────────────────────────────

/// Runtime value produced by expression evaluation.
#[derive(Debug, Clone)]
pub enum Val {
    Str(String),
    Bool(bool),
    List(Vec<String>),
    Null,
}

impl Val {
    pub fn as_str(&self) -> &str {
        match self {
            Val::Str(s) => s,
            Val::Bool(true) => "true",
            Val::Bool(false) => "false",
            Val::Null | Val::List(_) => "",
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Val::Bool(b) => *b,
            Val::Str(s) => !s.is_empty(),
            Val::List(l) => !l.is_empty(),
            Val::Null => false,
        }
    }

    fn to_string_val(&self) -> String {
        match self {
            Val::Str(s) => s.clone(),
            Val::Bool(b) => b.to_string(),
            Val::Null => String::new(),
            Val::List(l) => l.join(", "),
        }
    }
}

// ─── Tokens ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),
    Str(String),
    Int(i64),
    Dot,
    LParen,
    RParen,
    Comma,
    Not,
    Eq,
    Neq,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' => i += 1,
            '.' => {
                tokens.push(Token::Dot);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            '!' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                tokens.push(Token::Neq);
                i += 2;
            }
            '!' => {
                tokens.push(Token::Not);
                i += 1;
            }
            '=' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                tokens.push(Token::Eq);
                i += 2;
            }
            '"' | '\'' => {
                let quote = chars[i];
                i += 1;
                let start = i;
                while i < chars.len() && chars[i] != quote {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::Str(s));
                if i < chars.len() {
                    i += 1; // closing quote
                }
            }
            c if c.is_ascii_digit() || (c == '-' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit()) => {
                let start = i;
                if c == '-' {
                    i += 1;
                }
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::Int(s.parse().unwrap_or(0)));
            }
            c if c.is_alphanumeric() || c == '_' || c == '/' || c == '#' => {
                let start = i;
                while i < chars.len()
                    && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '/' || chars[i] == '#')
                {
                    i += 1;
                }
                let s: String = chars[start..i].iter().collect();
                tokens.push(Token::Ident(s));
            }
            _ => i += 1, // skip unknown
        }
    }

    tokens
}

// ─── AST ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Expr {
    /// String or number literal
    Literal(Val),
    /// Property chain: file.name, property.tags, this.file.name
    Property(Vec<String>),
    /// Method call: expr.method(args...)
    MethodCall {
        receiver: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    /// Function call: contains(a, b), file.hasTag("t1", "t2")
    FuncCall {
        name: String,
        receiver: Option<Box<Expr>>,
        args: Vec<Expr>,
    },
    /// Binary comparison: a != b, a == b
    BinOp {
        op: BinOperator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Prefix negation: !expr
    Not(Box<Expr>),
}

#[derive(Debug, Clone, Copy)]
enum BinOperator {
    Eq,
    Neq,
}

// ─── Parser ─────────────────────────────────────────────────

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Parse a full expression.
    fn parse_expr(&mut self) -> Option<Expr> {
        let left = self.parse_unary()?;

        // Check for binary operators
        match self.peek() {
            Some(Token::Neq) => {
                self.advance();
                let right = self.parse_unary()?;
                Some(Expr::BinOp {
                    op: BinOperator::Neq,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            Some(Token::Eq) => {
                self.advance();
                let right = self.parse_unary()?;
                Some(Expr::BinOp {
                    op: BinOperator::Eq,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            _ => Some(left),
        }
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        if self.peek() == Some(&Token::Not) {
            self.advance();
            let inner = self.parse_primary()?;
            return Some(Expr::Not(Box::new(inner)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        let base = match self.peek()? {
            Token::Str(_) => {
                let Token::Str(s) = self.advance()? else {
                    unreachable!()
                };
                Expr::Literal(Val::Str(s))
            }
            Token::Int(_) => {
                let Token::Int(n) = self.advance()? else {
                    unreachable!()
                };
                Expr::Literal(Val::Str(n.to_string()))
            }
            Token::Ident(_) => {
                let Token::Ident(name) = self.advance()? else {
                    unreachable!()
                };

                // Check if this is a standalone function call: name(args)
                if self.peek() == Some(&Token::LParen) {
                    self.advance(); // consume (
                    let args = self.parse_arg_list();
                    self.expect(&Token::RParen);
                    Expr::FuncCall {
                        name,
                        receiver: None,
                        args,
                    }
                } else {
                    // Start of a property chain: collect dots
                    let mut chain = vec![name];
                    while self.peek() == Some(&Token::Dot) {
                        self.advance(); // consume .
                        if let Some(Token::Ident(_)) = self.peek() {
                            let Token::Ident(part) = self.advance().unwrap() else {
                                unreachable!()
                            };
                            chain.push(part);
                        } else {
                            break;
                        }

                        // Check if last part is a function call
                        if self.peek() == Some(&Token::LParen) {
                            let method = chain.pop().unwrap();
                            self.advance(); // consume (
                            let args = self.parse_arg_list();
                            self.expect(&Token::RParen);

                            let receiver = Expr::Property(chain.clone());

                            // Return a method call or function call with receiver
                            let result = if is_builtin_method(&method) {
                                Expr::MethodCall {
                                    receiver: Box::new(receiver),
                                    method,
                                    args,
                                }
                            } else {
                                Expr::FuncCall {
                                    name: method,
                                    receiver: Some(Box::new(receiver)),
                                    args,
                                }
                            };

                            // Allow further chaining: .method().method()
                            return Some(self.parse_chain(result));
                        }
                    }
                    Expr::Property(chain)
                }
            }
            _ => return None,
        };

        // Check for chained method calls on the base
        Some(self.parse_chain(base))
    }

    /// Parse chained method calls after a base expression.
    fn parse_chain(&mut self, mut base: Expr) -> Expr {
        while self.peek() == Some(&Token::Dot) {
            self.advance(); // consume .
            if let Some(Token::Ident(_)) = self.peek() {
                let Token::Ident(part) = self.advance().unwrap() else {
                    unreachable!()
                };
                if self.peek() == Some(&Token::LParen) {
                    self.advance(); // consume (
                    let args = self.parse_arg_list();
                    self.expect(&Token::RParen);

                    base = if is_builtin_method(&part) {
                        Expr::MethodCall {
                            receiver: Box::new(base),
                            method: part,
                            args,
                        }
                    } else {
                        Expr::FuncCall {
                            name: part,
                            receiver: Some(Box::new(base)),
                            args,
                        }
                    };
                } else {
                    // Property access on an expression — convert to method call on toString
                    // e.g., `this.file` after a method call — rebuild as property
                    // This is an edge case; for now, just ignore stray dots
                }
            }
        }
        base
    }

    fn parse_arg_list(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();
        if self.peek() == Some(&Token::RParen) {
            return args;
        }
        if let Some(arg) = self.parse_expr() {
            args.push(arg);
        }
        while self.peek() == Some(&Token::Comma) {
            self.advance();
            if let Some(arg) = self.parse_expr() {
                args.push(arg);
            }
        }
        args
    }
}

fn is_builtin_method(name: &str) -> bool {
    matches!(
        name,
        "toString" | "startsWith" | "endsWith" | "contains" | "slice"
    )
}

// ─── Evaluation ─────────────────────────────────────────────

/// Context for `this.*` references — the .base file itself.
pub struct ThisContext {
    pub name: String,
    pub folder: String,
    pub rel_path: String,
    pub properties: std::collections::HashMap<String, serde_yaml::Value>,
}

impl ThisContext {
    /// Create a `ThisContext` from a .base file path relative to vault root.
    pub fn from_base_path(vault_root: &Path, base_path: &Path) -> Self {
        let rel = base_path
            .strip_prefix(vault_root)
            .unwrap_or(base_path);
        let name = base_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let folder = rel
            .parent()
            .unwrap_or(Path::new(""))
            .to_string_lossy()
            .into_owned();
        let rel_path = rel.to_string_lossy().into_owned();

        Self {
            name,
            folder,
            rel_path,
            properties: std::collections::HashMap::new(),
        }
    }
}

/// Evaluate a filter expression string against a note.
pub fn eval_filter(expr_str: &str, note: &NoteContext, this_ctx: &ThisContext) -> bool {
    let tokens = tokenize(expr_str);
    let mut parser = Parser::new(tokens);
    if let Some(expr) = parser.parse_expr() {
        eval_expr(&expr, note, this_ctx).as_bool()
    } else {
        eprintln!("warning: could not parse expression: {expr_str}");
        true // permissive: include note if we can't parse the filter
    }
}

fn eval_expr(expr: &Expr, note: &NoteContext, this_ctx: &ThisContext) -> Val {
    match expr {
        Expr::Literal(v) => v.clone(),

        Expr::Property(chain) => resolve_property(chain, note, this_ctx),

        Expr::Not(inner) => Val::Bool(!eval_expr(inner, note, this_ctx).as_bool()),

        Expr::BinOp { op, left, right } => {
            let l = eval_expr(left, note, this_ctx);
            let r = eval_expr(right, note, this_ctx);
            let result = match op {
                BinOperator::Eq => l.to_string_val() == r.to_string_val(),
                BinOperator::Neq => l.to_string_val() != r.to_string_val(),
            };
            Val::Bool(result)
        }

        Expr::MethodCall {
            receiver,
            method,
            args,
        } => {
            let recv = eval_expr(receiver, note, this_ctx);
            eval_method(&recv, method, args, note, this_ctx)
        }

        Expr::FuncCall {
            name,
            receiver,
            args,
        } => eval_func(name, receiver.as_deref(), args, note, this_ctx),
    }
}

fn resolve_property(chain: &[String], note: &NoteContext, this_ctx: &ThisContext) -> Val {
    if chain.is_empty() {
        return Val::Null;
    }

    // Handle `this.*` references
    if chain[0] == "this" {
        return resolve_this(&chain[1..], this_ctx);
    }

    // Handle `file.*` properties
    if chain[0] == "file" {
        return resolve_file_prop(&chain[1..], note);
    }

    // Handle `property.*` — explicit frontmatter access
    if chain[0] == "property" && chain.len() >= 2 {
        return resolve_frontmatter(&chain[1..], note);
    }

    // Handle `note.*` — alias for property access
    if chain[0] == "note" && chain.len() >= 2 {
        return resolve_frontmatter(&chain[1..], note);
    }

    // Bare property name — check frontmatter first, then fall back
    if chain.len() == 1 {
        let key = &chain[0];
        if let Some(val) = note.get_property(key) {
            return yaml_to_val(val);
        }
    }

    // Dotted bare property: created.toString() etc. — treat first part as property
    if let Some(val) = note.get_property(&chain[0]) {
        return yaml_to_val(val);
    }

    Val::Null
}

fn resolve_this(chain: &[String], this_ctx: &ThisContext) -> Val {
    if chain.is_empty() {
        return Val::Str(this_ctx.name.clone());
    }

    if chain[0] == "file" {
        return match chain.get(1).map(String::as_str) {
            Some("fullname") => Val::Str(this_ctx.rel_path.clone()),
            Some("folder") => Val::Str(this_ctx.folder.clone()),
            _ => Val::Str(this_ctx.name.clone()), // "name" and fallback
        };
    }

    // Bare property on this — check this_ctx.properties
    if chain.len() == 1 {
        if let Some(val) = this_ctx.properties.get(&chain[0]) {
            return yaml_to_val(val);
        }
    }

    Val::Null
}

fn resolve_file_prop(chain: &[String], note: &NoteContext) -> Val {
    if chain.is_empty() {
        return Val::Str(note.rel_path.clone());
    }
    match chain[0].as_str() {
        "name" => Val::Str(note.name.clone()),
        "path" | "fullname" => Val::Str(note.rel_path.clone()),
        "ext" => Val::Str(note.ext.clone()),
        "folder" => Val::Str(note.folder.clone()),
        "tags" => Val::List(note.tags.clone()),
        "links" => Val::List(note.links.clone()),
        _ => Val::Null,
    }
}

fn resolve_frontmatter(chain: &[String], note: &NoteContext) -> Val {
    if chain.is_empty() {
        return Val::Null;
    }
    // Support dotted keys: property.item.owned → frontmatter key "item.owned" or nested
    let key = chain.join(".");
    if let Some(val) = note.get_property(&key) {
        return yaml_to_val(val);
    }
    // Try first key, then traverse
    if let Some(val) = note.get_property(&chain[0]) {
        if chain.len() == 1 {
            return yaml_to_val(val);
        }
        // Nested access: property.item.owned → properties["item"]["owned"]
        let mut current = val.clone();
        for part in &chain[1..] {
            match current.get(part.as_str()) {
                Some(v) => current = v.clone(),
                None => return Val::Null,
            }
        }
        return yaml_to_val(&current);
    }
    Val::Null
}

fn eval_method(
    recv: &Val,
    method: &str,
    args: &[Expr],
    note: &NoteContext,
    this_ctx: &ThisContext,
) -> Val {
    match method {
        "toString" => Val::Str(recv.to_string_val()),

        "startsWith" => {
            let s = recv.to_string_val();
            let prefix = args
                .first()
                .map(|a| eval_expr(a, note, this_ctx).to_string_val())
                .unwrap_or_default();
            Val::Bool(s.starts_with(&prefix))
        }

        "endsWith" => {
            let s = recv.to_string_val();
            let suffix = args
                .first()
                .map(|a| eval_expr(a, note, this_ctx).to_string_val())
                .unwrap_or_default();
            Val::Bool(s.ends_with(&suffix))
        }

        "contains" => {
            match recv {
                Val::List(list) => {
                    let target = args
                        .first()
                        .map(|a| eval_expr(a, note, this_ctx).to_string_val())
                        .unwrap_or_default();
                    Val::Bool(list.iter().any(|item| item == &target))
                }
                Val::Str(s) => {
                    let needle = args
                        .first()
                        .map(|a| eval_expr(a, note, this_ctx).to_string_val())
                        .unwrap_or_default();
                    Val::Bool(s.contains(&needle))
                }
                _ => Val::Bool(false),
            }
        }

        "slice" => {
            let s = recv.to_string_val();
            let start = args
                .first()
                .map(|a| eval_expr(a, note, this_ctx).to_string_val())
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0);
            let end = args
                .get(1)
                .map(|a| eval_expr(a, note, this_ctx).to_string_val())
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(s.len());
            let start = start.min(s.len());
            let end = end.min(s.len());
            Val::Str(s[start..end].to_owned())
        }

        _ => {
            eprintln!("warning: unknown method: {method}");
            Val::Null
        }
    }
}

fn eval_func(
    name: &str,
    receiver: Option<&Expr>,
    args: &[Expr],
    note: &NoteContext,
    this_ctx: &ThisContext,
) -> Val {
    match name {
        // Global contains(collection, value)
        "contains" if receiver.is_none() => {
            let coll = args
                .first()
                .map_or(Val::Null, |a| eval_expr(a, note, this_ctx));
            let target = args
                .get(1)
                .map(|a| eval_expr(a, note, this_ctx).to_string_val())
                .unwrap_or_default();
            match coll {
                Val::List(list) => Val::Bool(list.iter().any(|item| item.contains(&target))),
                Val::Str(s) => Val::Bool(s.contains(&target)),
                _ => Val::Bool(false),
            }
        }

        // file.hasTag("tag1", "tag2", ...) — true if any match
        "hasTag" => {
            for arg in args {
                let tag = eval_expr(arg, note, this_ctx).to_string_val();
                if note.has_tag(&tag) {
                    return Val::Bool(true);
                }
            }
            Val::Bool(false)
        }

        // file.hasLink(ref) — true if note links to ref
        "hasLink" => {
            let target = args
                .first()
                .map(|a| {
                    let v = eval_expr(a, note, this_ctx);
                    v.to_string_val()
                })
                .unwrap_or_default();
            // If target is a this.file reference, use the name
            Val::Bool(note.has_link(&target))
        }

        // file.links.contains(ref)
        "contains" if receiver.is_some() => {
            let recv = eval_expr(receiver.unwrap(), note, this_ctx);
            let target = args
                .first()
                .map(|a| eval_expr(a, note, this_ctx).to_string_val())
                .unwrap_or_default();
            match recv {
                Val::List(list) => Val::Bool(list.contains(&target)),
                Val::Str(s) => Val::Bool(s.contains(&target)),
                _ => Val::Bool(false),
            }
        }

        _ => {
            eprintln!("warning: unknown function: {name}");
            Val::Null
        }
    }
}

fn yaml_to_val(val: &serde_yaml::Value) -> Val {
    match val {
        serde_yaml::Value::String(s) => Val::Str(s.clone()),
        serde_yaml::Value::Bool(b) => Val::Bool(*b),
        serde_yaml::Value::Number(n) => Val::Str(n.to_string()),
        serde_yaml::Value::Sequence(seq) => {
            let items: Vec<String> = seq
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
            Val::List(items)
        }
        serde_yaml::Value::Null => Val::Null,
        _ => Val::Str(serde_yaml::to_string(val).unwrap_or_default().trim().to_owned()),
    }
}
