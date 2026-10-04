<p align="center">
  <a href="https://github.com/daynin/monk">
    <img src="./logo.svg" height="200px"/>
  </a>
</p>

<h2 align="center">
    Monk is a simple Git hooks manager
</h2>

<p align="center">
  <a href="https://www.bestpractices.dev/en/projects/10442">
    <img alt="OpenSSF Best Practices" src="https://www.bestpractices.dev/projects/6505/badge">
  </a>
  <a href="https://github.com/daynin/monk/blob/master/LICENSE">
    <img alt="License" src="https://img.shields.io/badge/license-MIT-blue.svg">
  </a>
  <a href="https://github.com/daynin/monk/issues">
    <img alt="GitHub Issues" src="https://img.shields.io/github/issues/daynin/monk.svg">
  </a>
  <a href="https://crates.io/crates/monk">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/monk.svg">
  </a>
  <a href="https://crates.io/crates/monk">
    <img alt="Downloads" src="https://img.shields.io/crates/d/monk">
  </a>
</p>

### Monk's features:

- 🦀 **Easily set up in your Rust project.** No need to install additional package managers.
- ⚙️ **Works with custom `build.rs` files.** Automate the hooks installation process.
- 💻 **Run your hooks via CLI.** Test your hooks without triggering them via Git.

> Keep calm, monk will protect your repo!

### Installation

You can install it using `cargo`:

```sh
cargo install monk
```

#### Or

You can add it as a build dependency:

```sh
cargo add --build monk
```

Then create a `build.rs` file:

```rust
pub fn main() {
    monk::init();
}
```

In this case, `monk` will be installed automatically and will initialize all hooks from `monk.toml`
.
This is the most convenient option for Rust projects, as it doesn't require contributors to install `monk` manually.


### Usage

Create a configuration file named `monk.toml` in your project root:

```toml
[pre-commit.commands.fmt]
run = "cargo fmt -- --check"

[pre-commit.commands.clippy]
run = "cargo clippy -- -D warnings"

[pre-push.commands.test]
run = "cargo test"
```

Then install the hooks:

```sh
monk install
```

If you added monk as a build dependency with `build.rs` (see above), hooks are installed automatically when you build your project.

---

### Documentation

#### Named Commands

Each command has a name and a `run` field:

```toml
[pre-commit.commands.fmt]
run = "cargo fmt -- --check"

[pre-commit.commands.clippy]
run = "cargo clippy -- -D warnings"

[pre-commit.commands.test]
run = "cargo test"
```

Commands run in the order they are defined. If any command fails, execution stops and the hook fails.

<details>
<summary>Legacy format (backward compatible)</summary>

A plain array of strings still works:

```toml
[pre-commit]
commands = ["cargo fmt -- --check", "cargo clippy -- -D warnings"]
```

Commands are auto-named `cmd1`, `cmd2`, etc. The named format is recommended for new configs.

</details>

#### File Placeholders

Use placeholders to pass file lists to your tools:

| Placeholder | Expands to |
|---|---|
| `{staged_files}` | Files staged for commit (`git diff --cached`) |
| `{push_files}` | Files changed between local and remote |
| `{all_files}` | All tracked files in the repository |

```toml
[pre-commit.commands.lint]
run = "eslint {staged_files}"

[pre-commit.commands.fmt]
run = "prettier --write {staged_files}"

[pre-push.commands.test]
run = "cargo test {push_files}"
```

When a placeholder expands to an empty file list, the command is automatically skipped. If the expanded command exceeds the OS argument length limit, it is automatically split into batches.

#### Glob Filtering

Use `glob` and `exclude` to filter which files a command applies to:

```toml
[pre-commit.commands.lint-js]
run = "eslint {staged_files}"
glob = "*.{js,ts}"
exclude = "*.min.js"

[pre-commit.commands.lint-rs]
run = "cargo clippy"
glob = "*.rs"

[pre-commit.commands.fmt]
run = "prettier --write {staged_files}"
glob = ["*.js", "*.ts", "*.css"]
```

Both `glob` and `exclude` accept a single pattern or a list of patterns. Patterns without a `/` match files in any directory (e.g., `*.rs` matches `src/main.rs`).

When `glob` is set but no files match, the command is skipped. When `glob` is used with a placeholder like `{staged_files}`, only matching files are passed to the command.

When `glob` is set without a file placeholder, monk checks staged files against the pattern and skips the command if none match.

#### Path-Based Configuration

For monorepos with multiple modules or mixed technologies:

```toml
[pre-commit.paths."frontend/"]
working_directory = "frontend"

[pre-commit.paths."frontend/".commands.lint]
run = "npm run lint"

[pre-commit.paths."frontend/".commands.test]
run = "npm test"

[pre-commit.paths."backend/"]
working_directory = "backend"

[pre-commit.paths."backend/".commands.fmt]
run = "cargo fmt -- --check"

[pre-commit.paths."backend/".commands.clippy]
run = "cargo clippy -- -D warnings"
```

When using `monk run --changed-only` (the default for installed hooks), only hooks whose path prefix matches the changed files will run.

#### Working Directory

Set `working_directory` at the hook level or the command level. Command-level overrides hook-level:

```toml
[pre-commit.commands.frontend-lint]
run = "npm run lint"
working_directory = "frontend"

[pre-commit.commands.backend-test]
run = "cargo test"
working_directory = "backend"
```

Or at the hook level for all commands:

```toml
[pre-commit]
working_directory = "backend"

[pre-commit.commands.fmt]
run = "cargo fmt -- --check"

[pre-commit.commands.test]
run = "cargo test"
```

#### Parallel Execution

Run all commands in a hook concurrently with `parallel: true`:

```toml
[pre-commit]
parallel = true

[pre-commit.commands.fmt]
run = "cargo fmt -- --check"

[pre-commit.commands.clippy]
run = "cargo clippy -- -D warnings"

[pre-commit.commands.test]
run = "cargo test"
```

All commands run simultaneously and their output is buffered. A summary with pass/fail status and timing is printed after all commands finish. If any command fails, the hook fails.

#### Piped Execution

Run commands sequentially in priority order with `piped: true`:

```toml
[pre-commit]
piped = true

[pre-commit.commands.install]
run = "npm install"
priority = 1

[pre-commit.commands.lint]
run = "eslint ."
priority = 2

[pre-commit.commands.test]
run = "npm test"
priority = 3
```

Commands are sorted by `priority` (lower number runs first). Commands without `priority` run after prioritized ones, in their original definition order. If any command fails, execution stops and the hook fails.

Add `follow: true` to continue running all commands even when one fails:

```toml
[post-merge]
piped = true
follow = true

[post-merge.commands.bundle]
run = "bundle install"
priority = 1

[post-merge.commands.migrate]
run = "bundle exec rails db:migrate"
priority = 2
```

With `follow: true`, all commands run regardless of failures and a summary with pass/fail status is printed at the end. If any command failed, the hook fails.

When both `piped` and `parallel` are set, `piped` takes precedence.

#### Skip Conditions

Skip hooks or individual commands based on git state, branch, or shell conditions:

```toml
[pre-commit]
skip = ["merge", "rebase"]

[pre-commit.commands.fmt]
run = "cargo fmt -- --check"

[pre-commit.commands.slow-test]
run = "cargo test --all"
skip = [{ run = 'test -n "$CI"' }]

[pre-push.commands.deploy]
run = "./deploy.sh"
skip = [{ ref = "main" }]

[pre-push.commands.test]
run = "cargo test"
skip = [{ ref = "release/*" }]
```

| Condition | Skips when |
|---|---|
| `merge` | A merge is in progress (`.git/MERGE_HEAD` exists) |
| `rebase` | A rebase is in progress |
| `ref: <pattern>` | Current branch matches the pattern (supports globs like `release/*`) |
| `run: <command>` | Shell command exits with code 0 |

Skip accepts a single condition or a list. If any condition matches, the hook or command is skipped.

To disable all hooks globally, set the environment variable `MONK=0`.

#### Required Hooks and Commands

Mark a hook or command as `required` to make it ignore `MONK=0` and its own `skip` conditions:

```toml
[pre-push.commands.test]
run = "cargo test"
required = true
```

A `required` command still runs when `MONK=0` is set, and when the global disable is active, only `required` hooks and commands run — everything else is skipped. Setting `required: true` on a hook itself makes the whole hook (including its own `skip` conditions) immune to `MONK=0`.

This is a client-side convenience, not a security boundary: anyone with shell access to the repository can still bypass it by editing the config, unsetting `MONK`, or running `git commit --no-verify` (which skips the installed hook script entirely, a Git behavior monk cannot override). Enforce genuinely non-bypassable rules server-side, e.g. via required CI checks or branch protection.

#### Environment Variables

Set environment variables for specific commands using the `env` key:

```toml
[pre-commit.commands.lint]
run = "eslint {staged_files}"

[pre-commit.commands.lint.env]
NODE_ENV = "production"
FORCE_COLOR = "1"

[pre-commit.commands.test]
run = "cargo test"

[pre-commit.commands.test.env]
RUST_LOG = "debug"
```

Environment variables are added to the command's process environment (they augment the inherited environment, not replace it). Each command can have its own set of environment variables.

#### RC Files

Use the top-level `rc` key to source a shell script before every command:

```toml
rc = ".monkrc"

[pre-commit.commands.lint]
run = "eslint ."

[pre-commit.commands.test]
run = "npm test"
```

The RC file is sourced via `. <path> && <command>` (POSIX-compatible dot-source). This is useful for shell-managed toolchains like nvm, rbenv, or pyenv that require shell initialization before tools are available.

The `rc` path is relative to the project root. RC also applies to `skip: run:` shell conditions.

#### Local Config Overrides

Create a `monk-local.toml` file (add it to `.gitignore`) to override or extend your project's `monk.toml` without affecting teammates:

```toml
[pre-commit]
parallel = true

[pre-commit.commands.clippy]
run = "cargo clippy"

[pre-commit.commands.mycheck]
run = "./my-local-check.sh"

[pre-push]
skip = [{ ref = "main" }]
```

Merge rules:
- **Hooks**: merged by name. New hooks are added, existing hooks are deep-merged.
- **Commands**: merged by name. A local command with the same name fully replaces the base command. New commands are added.
- **Scalar fields** (`parallel`, `working_directory`): local value overrides base.
- **Skip conditions**: local replaces base (not concatenated).
- **Different hook variants** (Simple vs PathBased): local replaces base entirely.

If `monk-local.toml` does not exist, `monk.toml` is used as-is.

#### CLI

```sh
monk install       # Install hooks defined in monk.toml
monk run <hook>    # Run a hook manually (e.g., monk run pre-commit)
monk uninstall     # Remove hooks and restore backups
```

`monk` automatically backs up existing hooks before installing. Running `monk uninstall` restores the original hooks.
