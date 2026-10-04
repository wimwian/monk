use monk::*;

#[test]
fn test_config_parallel_parses() {
    let toml = r#"
[pre-commit]
parallel = true

[pre-commit.commands.fmt]
run = "cargo fmt -- --check"

[pre-commit.commands.clippy]
run = "cargo clippy -- -D warnings"

[pre-commit.commands.test]
run = "cargo test"
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert_eq!(hooks.len(), 1);
    assert!(hooks[0].parallel);
    assert_eq!(hooks[0].commands.len(), 3);
}

#[test]
fn test_config_parallel_backward_compat() {
    let toml = r#"
[pre-commit.commands.fmt]
run = "cargo fmt -- --check"
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert!(!hooks[0].parallel);
}

#[test]
fn test_config_parallel_legacy_format() {
    let toml = r#"
[pre-commit]
parallel = true
commands = ["cargo fmt", "cargo clippy"]
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert!(hooks[0].parallel);
    assert_eq!(hooks[0].commands.len(), 2);
}

#[test]
fn test_config_parallel_path_based() {
    let toml = r#"
[pre-commit.paths."frontend/"]
parallel = true

[pre-commit.paths."frontend/".commands.lint]
run = "npm run lint"

[pre-commit.paths."frontend/".commands.test]
run = "npm test"

[pre-commit.paths."backend/".commands.fmt]
run = "cargo fmt -- --check"
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert_eq!(hooks.len(), 2);
    assert!(hooks[0].parallel);
    assert!(!hooks[1].parallel);
}

#[test]
fn test_config_parallel_with_glob() {
    let toml = r#"
[pre-commit]
parallel = true

[pre-commit.commands.lint-js]
run = "eslint {staged_files}"
glob = "*.{js,ts}"

[pre-commit.commands.lint-rs]
run = "cargo clippy"
glob = "*.rs"
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert!(hooks[0].parallel);

    let lint_js = hooks[0].commands.get("lint-js").unwrap();
    assert_eq!(lint_js.glob, vec!["*.{js,ts}"]);

    let lint_rs = hooks[0].commands.get("lint-rs").unwrap();
    assert_eq!(lint_rs.glob, vec!["*.rs"]);
}
