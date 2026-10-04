use monk::*;

#[test]
fn test_skip_single_merge() {
    let toml = r#"
[pre-commit]
skip = ["merge"]

[pre-commit.commands.fmt]
run = "cargo fmt -- --check"
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert_eq!(hooks[0].skip, vec![SkipCondition::Merge]);
}

#[test]
fn test_skip_single_rebase() {
    let toml = r#"
[pre-commit]
skip = ["rebase"]

[pre-commit.commands.fmt]
run = "cargo fmt -- --check"
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert_eq!(hooks[0].skip, vec![SkipCondition::Rebase]);
}

#[test]
fn test_skip_list_merge_and_rebase() {
    let toml = r#"
[pre-commit]
skip = ["merge", "rebase"]

[pre-commit.commands.fmt]
run = "cargo fmt -- --check"
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert_eq!(
        hooks[0].skip,
        vec![SkipCondition::Merge, SkipCondition::Rebase]
    );
}

#[test]
fn test_skip_ref_exact_branch() {
    let toml = r#"
[pre-push.commands.deploy]
run = "./deploy.sh"
skip = [{ ref = "main" }]
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-push");
    let deploy = hooks[0].commands.get("deploy").unwrap();
    assert_eq!(deploy.skip, vec![SkipCondition::Ref("main".to_string())]);
}

#[test]
fn test_skip_ref_glob_pattern() {
    let toml = r#"
[pre-push.commands.test]
run = "cargo test"
skip = [{ ref = "release/*" }]
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-push");
    let test_cmd = hooks[0].commands.get("test").unwrap();
    assert_eq!(
        test_cmd.skip,
        vec![SkipCondition::Ref("release/*".to_string())]
    );
}

#[test]
fn test_skip_run_shell_condition() {
    let toml = r#"
[pre-commit.commands.slow-test]
run = "cargo test --all"
skip = [{ run = 'test -n "$CI"' }]
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    let cmd = hooks[0].commands.get("slow-test").unwrap();
    assert_eq!(
        cmd.skip,
        vec![SkipCondition::Run("test -n \"$CI\"".to_string())]
    );
}

#[test]
fn test_skip_mixed_conditions() {
    let toml = r#"
[pre-commit]
skip = ["merge", "rebase", { ref = "main" }]

[pre-commit.commands.fmt]
run = "cargo fmt -- --check"
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert_eq!(
        hooks[0].skip,
        vec![
            SkipCondition::Merge,
            SkipCondition::Rebase,
            SkipCondition::Ref("main".to_string()),
        ]
    );
}

#[test]
fn test_skip_defaults_to_empty() {
    let toml = r#"
[pre-commit.commands.fmt]
run = "cargo fmt -- --check"
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert!(hooks[0].skip.is_empty());
    assert!(hooks[0].commands.get("fmt").unwrap().skip.is_empty());
}

#[test]
fn test_skip_on_both_hook_and_command() {
    let toml = r#"
[pre-commit]
skip = ["merge"]

[pre-commit.commands.fmt]
run = "cargo fmt -- --check"
skip = [{ ref = "main" }]

[pre-commit.commands.clippy]
run = "cargo clippy"
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert_eq!(hooks[0].skip, vec![SkipCondition::Merge]);
    assert_eq!(
        hooks[0].commands.get("fmt").unwrap().skip,
        vec![SkipCondition::Ref("main".to_string())]
    );
    assert!(hooks[0].commands.get("clippy").unwrap().skip.is_empty());
}

#[test]
fn test_skip_with_path_based_config() {
    let toml = r#"
[pre-commit.paths."frontend/"]
skip = ["merge"]

[pre-commit.paths."frontend/".commands.lint]
run = "npm run lint"

[pre-commit.paths."backend/".commands.fmt]
run = "cargo fmt -- --check"
skip = [{ ref = "release/*" }]
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert_eq!(hooks[0].skip, vec![SkipCondition::Merge]);
    assert!(hooks[1].skip.is_empty());
    assert_eq!(
        hooks[1].commands.get("fmt").unwrap().skip,
        vec![SkipCondition::Ref("release/*".to_string())]
    );
}

#[test]
fn test_skip_with_parallel() {
    let toml = r#"
[pre-commit]
parallel = true
skip = ["rebase"]

[pre-commit.commands.fmt]
run = "cargo fmt -- --check"

[pre-commit.commands.clippy]
run = "cargo clippy"
"#;
    let config: Config = toml::from_str(toml).unwrap();
    let hooks = find_all_path_configs(&config, "pre-commit");
    assert!(hooks[0].parallel);
    assert_eq!(hooks[0].skip, vec![SkipCondition::Rebase]);
}
