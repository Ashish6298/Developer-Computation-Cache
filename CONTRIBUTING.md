# Contributing to Developer Computation Cache (`dcc`)

Thank you for your interest in contributing to **Developer Computation Cache (`dcc`)**! We welcome contributions from systems engineers, tool builders, and researchers advancing high-performance, deterministic computation caching for polyglot developer workflows.

---

## 🧭 Table of Contents

- [Code of Conduct](#-code-of-conduct)
- [Architecture & Core Guarantees](#-architecture--core-guarantees)
- [Workspace Organization](#-workspace-organization)
- [Development Setup](#-development-setup)
- [Running Tests & Verification](#-running-tests--verification)
- [Contribution Workflow](#-contribution-workflow)
- [Commit Conventions](#-commit-conventions)
- [Engineering Guidelines](#-engineering-guidelines)
- [Security & Sensitive Data](#-security--sensitive-data)

---

## 📜 Code of Conduct

All contributors and maintainers are expected to uphold an open, welcoming, and inclusive community. Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## 🏗️ Architecture & Core Guarantees

Developer Computation Cache is built upon uncompromising systems engineering guarantees:

1. **Strict Correctness Guarantee**: A cache hit must **never** alter the semantic outcome of a computation. If safety cannot be mathematically and cryptographically proven, `dcc` strictly re-executes.
2. **Content-Addressed Immutability**: All storage keys and CAS objects are cryptographically addressed using streaming SHA-256 digests over raw bytes—never unstable file modification timestamps (`mtime`).
3. **Zero External Telemetry**: 100% local-first and offline. No network requests, telemetry, or remote services are required for core operations.
4. **Symlink Hardening & Path Confinement**: Outputs are confined strictly to the declared workspace boundaries. Directory traversal attempts (`../`) and symlink escapes are rigorously quarantined.
5. **Self-Healing & Quarantine**: Corrupted cache objects are automatically detected via checksum validation and safely quarantined (`*.corrupted`), seamlessly falling back to a clean re-execution.

---

## 📦 Workspace Organization

The repository is structured as a modular 6-crate Rust workspace:

| Crate | Path | Purpose |
| :--- | :--- | :--- |
| `dcc-core` | `crates/cache-core` | Domain types, canonical serialization, digest calculations, and cache key derivation. |
| `dcc-storage` | `crates/cache-storage` | Content-Addressed Storage (CAS), atomic operations, locking, eviction policies (LRU, LFU, FIFO), and disk budgeting. |
| `dcc-runner` | `crates/cache-runner` | Execution engine, process supervisor, single-flight locking, output restoration, and miss diagnostics. |
| `dcc-cli` | `crates/cache-cli` | User-facing command-line binary (`dcc`) built with `clap`. |
| `dcc-integrations` | `crates/cache-integrations` | Polyglot build tool adapters, code generator interfaces, and benchmark suite. |
| `dcc-test-utils` | `crates/cache-test-utils` | Shared synthetic workspaces, failure injectors, and concurrency fixtures. |

---

## 💻 Development Setup

### Prerequisites

- **Rust 1.77+** (Stable toolchain recommended)
- **Cargo** (Included with Rust toolchain)
- **Git 2.20+**

### Local Setup Steps

```bash
# 1. Fork and clone the repository
git clone https://github.com/<your-username>/Reforge.git
cd Reforge

# 2. Check toolchain requirements
rustc --version
cargo --version

# 3. Build the entire workspace
cargo build --workspace
```

---

## 🧪 Running Tests & Verification

Before opening a pull request, ensure all test suites, linters, and formatting checks pass across all workspace crates:

### 1. Format Check
```bash
cargo fmt --all -- --check
```
To automatically apply formatting:
```bash
cargo fmt --all
```

### 2. Lint Check (Clippy)
```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### 3. Full Workspace Test Suite
```bash
cargo test --workspace --verbose
```

### 4. Specialized Test Suites
```bash
# Concurrency & Process Locking Suite
cargo test -p dcc-runner --test concurrency_tests

# Security & Path Traversal Confinement Suite
cargo test -p dcc-runner --test security_tests

# Failure Injection & Resilience Suite
cargo test -p dcc-runner --test failure_injection_tests
```

### 5. Developer Integration & Examples Suite
```bash
cargo run -p dcc-integrations --example cached_codegen
cargo run -p dcc-integrations --example cached_analysis
cargo run -p dcc-integrations --example cached_transform
cargo run -p dcc-integrations --example cached_rust_build
cargo run -p dcc-integrations --example build_demonstration
cargo run -p dcc-integrations --example build_benchmark
```

---

## 🔄 Contribution Workflow

1. **Create an Issue**: For non-trivial features or cache-invalidation alterations, open an [issue](https://github.com/Ashish6298/Reforge/issues) first to discuss architecture and design trade-offs.
2. **Branch From `main`**:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Implement Cleanly**: Keep commits focused on a single change with accompanying unit/integration tests.
4. **Validate Locally**: Run the test and lint commands described above.
5. **Open a Pull Request**: Submit your pull request against the `main` branch with clear description and reproduction/test evidence.

---

## 📝 Commit Conventions

We follow clear, conventional commit messages:

- `feat(core): add streaming digest normalization for symlinked inputs`
- `fix(storage): enforce atomic rename during CAS object ingestion`
- `perf(runner): optimize concurrent lock acquisition under high contention`
- `test(security): add test verifying rejection of path traversal sequences`
- `docs(readme): clarify polyglot build tool integration contracts`

---

## 📐 Engineering Guidelines

- **Zero Unsound Code**: Avoid `unsafe` blocks unless absolutely required for platform-specific OS interactions, and document all safety invariants with `// SAFETY:` comments.
- **Cross-Platform Compatibility**: Code must run natively on Windows, Linux, and macOS without assuming a POSIX shell. Use `dcc_core::PathUtils` for path manipulations and avoid platform-specific shell idioms in tests (prefer `printf` over `echo -n`).
- **Atomic Persistence**: Never overwrite files in-place without write-to-temp-then-atomic-rename semantics (`.tmp` -> final).
- **Public API Documentation**: All public structs, enums, and methods must have descriptive Rustdoc comments explaining invariants and failure conditions.

---

## 🛡️ Security & Sensitive Data

- Never commit secrets, certificates, credentials, or production cache files to version control.
- If you discover a security vulnerability (such as a path traversal flaw or sandbox escape), please report it responsibly by contacting the maintainer directly at [ashishgoswami6298@gmail.com](mailto:ashishgoswami6298@gmail.com) or via GitHub profile [@Ashish6298](https://github.com/Ashish6298) rather than opening a public issue.
