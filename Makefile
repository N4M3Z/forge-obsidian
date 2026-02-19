# forge-obsidian — build, test, lint, install, verify

.PHONY: help build test lint check check-cli clean install install-skills install-skills-claude install-skills-gemini install-skills-codex install-skills-opencode verify verify-skills verify-skills-claude verify-skills-gemini verify-skills-codex verify-skills-opencode lib-init

# Variables
SKILLS = ObsidianCLI ObsidianConventions ObsidianBase ObsidianTemplates VaultOperations WikiLink ProjectConventions Draft Promote ObsidianREST ObsidianActions
SKILL_SRC = skills
LIB_DIR = $(or $(FORGE_LIB),lib)
SCOPE ?= workspace
CLAUDE_SKILLS_DST ?= $(if $(filter workspace,$(SCOPE)),$(CURDIR)/.claude/skills,$(HOME)/.claude/skills)
CODEX_SKILLS_DST ?= $(if $(filter workspace,$(SCOPE)),$(CURDIR)/.codex/skills,$(HOME)/.codex/skills)
OPENCODE_SKILLS_DST ?= $(if $(filter workspace,$(SCOPE)),$(CURDIR)/.opencode/skills,$(HOME)/.opencode/skills)

# Rust binaries from forge-lib submodule
INSTALL_SKILLS := $(LIB_DIR)/bin/install-skills
VALIDATE_MODULE := $(LIB_DIR)/bin/validate-module

help:
	@echo "forge-obsidian targets:"
	@echo "  make build     Compile Rust binaries"
	@echo "  make test      Run all tests + module validation"
	@echo "  make lint      Clippy + fmt + shellcheck + semgrep"
	@echo "  make check     Verify module structure and dependencies"
	@echo "  make install   Install skills for all providers (SCOPE=workspace|user|all)"
	@echo "  make verify    Verify the full installation"
	@echo "  make clean     Remove build artifacts and installed skills"

build:
	cargo build --release

lib-init:
	@if [ ! -f $(LIB_DIR)/Cargo.toml ]; then \
	  echo "Initializing forge-lib submodule..."; \
	  git submodule update --init $(LIB_DIR); \
	fi

$(INSTALL_SKILLS) $(VALIDATE_MODULE): lib-init
	@$(MAKE) -C $(LIB_DIR) build

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

install-skills: install-skills-claude install-skills-gemini install-skills-codex install-skills-opencode

install-skills-claude: $(INSTALL_SKILLS)
	@if [ "$(SCOPE)" = "all" ]; then \
	  $(INSTALL_SKILLS) $(SKILL_SRC) --provider claude --scope "$(SCOPE)" --dst "$(CURDIR)/.claude/skills"; \
	  $(INSTALL_SKILLS) $(SKILL_SRC) --provider claude --scope "$(SCOPE)" --dst "$(HOME)/.claude/skills"; \
	elif [ "$(SCOPE)" = "workspace" ]; then \
	  $(INSTALL_SKILLS) $(SKILL_SRC) --provider claude --scope "$(SCOPE)" --dst "$(CURDIR)/.claude/skills"; \
	elif [ "$(SCOPE)" = "user" ]; then \
	  $(INSTALL_SKILLS) $(SKILL_SRC) --provider claude --scope "$(SCOPE)" --dst "$(HOME)/.claude/skills"; \
	else \
	  echo "Error: Invalid SCOPE '$(SCOPE)'. Use workspace, user, or all."; \
	  exit 1; \
	fi

install-skills-gemini: $(INSTALL_SKILLS)
	@if command -v gemini >/dev/null 2>&1; then \
	  $(INSTALL_SKILLS) $(SKILL_SRC) --provider gemini --scope "$(SCOPE)"; \
	else \
	  echo "  skip gemini skill install (gemini CLI not installed)"; \
	fi

install-skills-codex: $(INSTALL_SKILLS)
	@if [ "$(SCOPE)" = "all" ]; then \
	  $(INSTALL_SKILLS) $(SKILL_SRC) --provider codex --scope "$(SCOPE)" --dst "$(CURDIR)/.codex/skills"; \
	  $(INSTALL_SKILLS) $(SKILL_SRC) --provider codex --scope "$(SCOPE)" --dst "$(HOME)/.codex/skills"; \
	elif [ "$(SCOPE)" = "workspace" ]; then \
	  $(INSTALL_SKILLS) $(SKILL_SRC) --provider codex --scope "$(SCOPE)" --dst "$(CURDIR)/.codex/skills"; \
	elif [ "$(SCOPE)" = "user" ]; then \
	  $(INSTALL_SKILLS) $(SKILL_SRC) --provider codex --scope "$(SCOPE)" --dst "$(HOME)/.codex/skills"; \
	else \
	  echo "Error: Invalid SCOPE '$(SCOPE)'. Use workspace, user, or all."; \
	  exit 1; \
	fi

install-skills-opencode:
	@if [ "$(SCOPE)" = "all" ]; then \
	  dsts="$(CURDIR)/.opencode/skills $(HOME)/.opencode/skills"; \
	elif [ "$(SCOPE)" = "workspace" ]; then \
	  dsts="$(CURDIR)/.opencode/skills"; \
	elif [ "$(SCOPE)" = "user" ]; then \
	  dsts="$(HOME)/.opencode/skills"; \
	else \
	  echo "Error: Invalid SCOPE '$(SCOPE)'. Use workspace, user, or all."; \
	  exit 1; \
	fi; \
	for dst in $$dsts; do \
	  mkdir -p "$$dst"; \
	  for skill_dir in $(SKILL_SRC)/*/; do \
	    skill=$$(basename "$$skill_dir"); \
	    kebab=$$(echo "$$skill" | sed 's/\([A-Z]\)/-\1/g' | sed 's/^-//' | tr '[:upper:]' '[:lower:]'); \
	    mkdir -p "$$dst/$$kebab"; \
	    command cp "$$skill_dir"SKILL.md "$$dst/$$kebab/SKILL.md" 2>/dev/null || true; \
	    command cp "$$skill_dir"SKILL.yaml "$$dst/$$kebab/SKILL.yaml" 2>/dev/null || true; \
	  done; \
	  echo "  installed $$(ls -d "$$dst"/*/ 2>/dev/null | wc -l | tr -d ' ') skills to $$dst"; \
	done

clean:
	cargo clean
	@for dir in .claude/skills .gemini/skills .codex/skills .opencode/skills; do \
	  for s in $(SKILLS); do \
	    command rm -rf "$$dir/$$s" 2>/dev/null || true; \
	  done; \
	done
	@echo "Cleaned build artifacts and installed skills."

verify: verify-skills

verify-skills: verify-skills-claude verify-skills-gemini verify-skills-codex verify-skills-opencode

verify-skills-claude:
	@missing=0; \
	echo "Verifying Claude skills in $(CLAUDE_SKILLS_DST)..."; \
	for s in $(SKILLS); do \
	  if test -f "$(CLAUDE_SKILLS_DST)/$$s/SKILL.md"; then \
	    echo "  ok $$s"; \
	  else \
	    echo "  missing $$s"; \
	    missing=1; \
	  fi; \
	done; \
	test $$missing -eq 0

verify-skills-gemini:
	@if command -v gemini >/dev/null 2>&1; then \
	  echo "Verifying Gemini skills via CLI..."; \
	  out_file=$$(mktemp); \
	  if gemini skills list > "$$out_file" 2>&1; then \
	    grep -E "ObsidianCLI|ObsidianBase|WikiLink" "$$out_file" || true; \
	  else \
	    echo "  skip gemini skill verification (non-interactive or unauthenticated)"; \
	  fi; \
	  command rm -f "$$out_file"; \
	else \
	  echo "  skip gemini skill verification (gemini CLI not installed)"; \
	fi

verify-skills-codex:
	@missing=0; \
	echo "Verifying Codex skills in $(CODEX_SKILLS_DST)..."; \
	for s in $(SKILLS); do \
	  if test -f "$(CODEX_SKILLS_DST)/$$s/SKILL.md"; then \
	    echo "  ok $$s"; \
	  else \
	    echo "  missing $$s"; \
	    missing=1; \
	  fi; \
	done; \
	test $$missing -eq 0

verify-skills-opencode:
	@missing=0; \
	echo "Verifying OpenCode skills in $(OPENCODE_SKILLS_DST)..."; \
	for skill_dir in $(SKILL_SRC)/*/; do \
	  skill=$$(basename "$$skill_dir"); \
	  kebab=$$(echo "$$skill" | sed 's/\([A-Z]\)/-\1/g' | sed 's/^-//' | tr '[:upper:]' '[:lower:]'); \
	  if test -f "$(OPENCODE_SKILLS_DST)/$$kebab/SKILL.md"; then \
	    echo "  ok $$kebab"; \
	  else \
	    echo "  missing $$kebab"; \
	    missing=1; \
	  fi; \
	done; \
	test $$missing -eq 0
