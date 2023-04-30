use crate::{Action, Namespace, Task};

pub fn namespace() -> Namespace {
    Namespace {
        key: "ts",
        description: "tasks for preparing typescript projects",
        tasks: vec![vite(), next(), prettier()],
    }
}

fn next() -> Task {
    Task {
        key: "next",
        actions: vec![
            nodejs_lts(),
            Action::Command(vec![s!("npx"), s!("create-next-app@latest"), s!(".")]),
        ],
    }
}

fn vite() -> Task {
    Task {
        key: "vite",
        actions: vec![
            nodejs_lts(),
            Action::Command(vec![s!("npm"), s!("create"), s!("vite@latest"), s!(".")]),
            npm_install_save_dev("vitest"),
            npm_pkg_set("scripts.test=vitest"),
        ],
    }
}

fn prettier() -> Task {
    Task {
        key: "prettier",
        actions: vec![
            npm_install_save_dev("prettier"),
            npm_install_save_dev("prettier-plugin-tailwindcss"),
            npm_pkg_set("scripts.format=prettier --write src"),
        ],
    }
}

fn npm_install_save_dev(arg: &str) -> Action {
    Action::Command(vec![s!("npm"), s!("install"), s!("--save-dev"), s!(arg)])
}

fn npm_pkg_set(arg: &str) -> Action {
    Action::Command(vec![s!("npm"), s!("pkg"), s!("set"), s!(arg)])
}

fn nodejs_lts() -> Action {
    Action::Task(Task {
        key: "nodejs_lts",
        actions: vec![asdf_install("nodejs", "lts"), asdf_local("nodejs", "lts")],
    })
}

fn asdf_install(lang: &str, version: &str) -> Action {
    Action::Command(vec![s!("asdf"), s!("install"), s!(lang), s!(version)])
}

fn asdf_local(lang: &str, version: &str) -> Action {
    Action::Command(vec![s!("asdf"), s!("local"), s!(lang), s!(version)])
}
