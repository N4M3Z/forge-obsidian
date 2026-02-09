# Contributing to forge-obsidian

## Code style

- All shell scripts start with `set -euo pipefail`
- Use `command rm`, `command cp`, `command mv` — never bare (macOS aliases add `-i`)
- Use `builtin cd` — `cd` may be intercepted by shell plugins
- Pass [shellcheck](https://www.shellcheck.net/) with no warnings
- Use `if/then/fi` instead of `&&` chains under `set -e` (see [learning](https://github.com/N4M3Z/forge-obsidian/wiki))

## Testing

Run tests before committing:

```bash
bash tests/test.sh
```

Tests use a simple `assert_eq`/`assert_contains` harness with temp directory isolation. Add tests for new functionality.

## Linting

If shellcheck is installed, the git pre-commit hook runs it automatically on staged `.sh` files.

Setup:

```bash
git config core.hooksPath .githooks
```

## Pull requests

1. Create a feature branch
2. Make changes
3. Run `bash tests/test.sh` — all tests pass
4. Run `shellcheck hooks/*.sh tests/*.sh` — clean
5. Open a PR with a clear description
