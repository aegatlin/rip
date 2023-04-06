use std::{io::Write, path::PathBuf, process::Command};

pub fn prettier(dir: PathBuf) {
    npm_command(&dir, &["install", "--save-dev", "prettier"]);
    npm_command(&dir, &["pkg", "set", "scripts.format=prettier --write src"]);
}

fn npm_command(dir: &PathBuf, args: &[&str]) {
    let cmd = Command::new("npm")
        .current_dir(dir.canonicalize().unwrap())
        .args(args)
        .output()
        .unwrap();

    std::io::stdout().write_all(&cmd.stdout).unwrap();
    std::io::stderr().write_all(&cmd.stderr).unwrap();
}

#[cfg(test)]
mod test {
    use assert_fs::fixture::TempDir;
    use assert_fs::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::fs;

    use crate::prettier;

    fn dir_with_empty_package_json() -> TempDir {
        let dir = TempDir::new().unwrap();
        dir.child("package.json").write_str("{}").unwrap();
        dir
    }

    fn parse_package_json(dir: &TempDir) -> PackageJson {
        let package_json = dir.child("package.json");
        let json_string = fs::read_to_string(package_json.canonicalize().unwrap()).unwrap();
        let json_str = json_string.as_str();
        let json: PackageJson = serde_json::from_str(json_str).unwrap();
        json
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct PackageJson {
        scripts: Option<HashMap<String, String>>,
        dev_dependencies: HashMap<String, String>,
    }

    #[test]
    fn prettier_dev_dep() {
        let dir = dir_with_empty_package_json();

        prettier(dir.to_path_buf());

        let json = parse_package_json(&dir);
        assert!(json.dev_dependencies.contains_key("prettier"));
    }

    #[test]
    fn prettier_format_script() {
        let dir = dir_with_empty_package_json();

        prettier(dir.to_path_buf());

        let json = parse_package_json(&dir);
        assert!(json
            .scripts
            .unwrap()
            .get("format")
            .unwrap()
            .eq("prettier --write src"));
    }
}
