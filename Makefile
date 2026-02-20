# forge-obsidian — build, test, lint, install, verify

SKILLS   = ObsidianCLI ObsidianConventions ObsidianBase ObsidianTemplates VaultOperations WikiLink ProjectConventions Draft Promote ObsidianREST ObsidianActions
SKILL_SRC = skills
LIB_DIR  = $(or $(FORGE_LIB),lib)

# Fallbacks when common.mk is not yet available (uninitialized submodule)
INSTALL_SKILLS  ?= $(LIB_DIR)/bin/install-skills
VALIDATE_MODULE ?= $(LIB_DIR)/bin/validate-module

.PHONY: help build test lint check check-cli clean install verify init

help:
	@echo "forge-obsidian targets:"
	@echo "  make build     Compile Rust binaries"
	@echo "  make test      Run all tests + module validation"
	@echo "  make lint      Clippy + fmt + shellcheck + semgrep"
	@echo "  make check     Verify module structure and dependencies"
	@echo "  make install   Install skills for all providers (SCOPE=workspace|user|all)"
	@echo "  make verify    Verify the full installation"
	@echo "  make clean     Remove build artifacts and installed skills"

init:
	@if [ ! -f $(LIB_DIR)/Cargo.toml ]; then \
	  echo "Initializing forge-lib submodule..."; \
	  git submodule update --init $(LIB_DIR); \
	fi

ifneq ($(wildcard $(LIB_DIR)/mk/common.mk),)
  include $(LIB_DIR)/mk/common.mk
  include $(LIB_DIR)/mk/skills/install.mk
  include $(LIB_DIR)/mk/skills/verify.mk
endif

build:
	cargo build --release

test: $(VALIDATE_MODULE)
	cargo test
	@if [ -f tests/test.sh ]; then bash tests/test.sh; fi
	@$(VALIDATE_MODULE) $(CURDIR)

lint:
	cargo fmt --check
	cargo clippy -- -D warnings
	@if find . -name '*.sh' -not -path '*/target/*' -not -path '*/lib/*' | grep -q .; then \
	  find . -name '*.sh' -not -path '*/target/*' -not -path '*/lib/*' | xargs shellcheck -S warning 2>/dev/null || true; \
	fi
	@if command -v semgrep >/dev/null 2>&1; then semgrep scan --config=p/owasp-top-ten --metrics=off --quiet . 2>/dev/null || true; fi

check-cli:
	@if command -v obsidian >/dev/null 2>&1; then \
	  echo "  ok obsidian CLI"; \
	elif [ -x "/Applications/Obsidian.app/Contents/MacOS/Obsidian" ]; then \
	  echo "  ok obsidian CLI (macOS app bundle, not in PATH)"; \
	else \
	  echo "  -- obsidian CLI (optional, requires Obsidian 1.12+)"; \
	fi

check: check-cli
	@test -f module.yaml && echo "  ok module.yaml" || echo "  MISSING module.yaml"
	@test -f Cargo.toml && echo "  ok Cargo.toml" || echo "  MISSING Cargo.toml"
	@test -d hooks && echo "  ok hooks/" || echo "  MISSING hooks/"
	@test -d skills/ObsidianCLI && echo "  ok skills/ObsidianCLI/" || echo "  MISSING skills/ObsidianCLI/"
	@test -x "$(INSTALL_SKILLS)" && echo "  ok install-skills" || echo "  MISSING install-skills (run: make -C $(LIB_DIR) build)"
	@test -x "$(VALIDATE_MODULE)" && echo "  ok validate-module" || echo "  MISSING validate-module (run: make -C $(LIB_DIR) build)"

install: install-skills
	@echo "Installation complete. Restart your session or reload skills."

clean: clean-skills
	cargo clean

verify: verify-skills
