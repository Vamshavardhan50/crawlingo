# 07_CONTRIBUTING_GUIDE.md

Welcome to Crawlingo! We welcome contributions to make this web data engine more reliable, modular, and performant.

---

## 1. Code of Conduct
By participating in this project, you agree to abide by basic professional standards: respect all contributors, maintain constructive discussions, and focus on evidence-based engineering critiques.

---

## 2. Development Workflow

### Step 1: Branch Naming Conventions
Always create a feature branch off of the `main` branch. Use the following prefix guidelines:
- `feature/` for new capabilities (e.g. `feature/browser-fetcher`)
- `bugfix/` for bug repairs (e.g. `bugfix/connection-leak`)
- `docs/` for documentation updates (e.g. `docs/api-overhaul`)
- `refactor/` for code restructuring (e.g. `refactor/fetcher-trait`)

### Step 2: Coding Standards
- **Rust:** Run `cargo fmt` and `cargo clippy --all-targets` before committing. Ensure there are no compiler warnings.
- **Python:** Format with `black` or `ruff` and verify type assertions.
- **Node.js:** Ensure ESLint checks pass (`npm run lint`).

---

## 3. Pull Request Guidelines

Before submitting a Pull Request:
1. **Sync with Main:** Rebase your branch against the latest commits on the upstream `main` branch to avoid conflicts.
2. **Write Tests:** Every new feature must include corresponding integration tests. Fixes must include regression tests.
3. **Run Checks Locally:** Verify all core and SDK test suites pass locally.
4. **Draft Detailed PR Descriptions:** List the specific files modified, explain why the changes were made, and outline your manual verification results.

---

## 4. Commit Message Formats

We recommend using Semantic Commits:
- `feat(core): add browser rendering strategy`
- `fix(engine): resolve sled db lock error`
- `docs(api): update page markdown methods`
- `refactor(ffi): consolidate python wrappers`
