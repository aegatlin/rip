use std::path::PathBuf;

use directories::UserDirs;

use crate::{s, Action, Namespace, Task};

pub fn namespace() -> Namespace {
    Namespace {
        key: "mac",
        description: "actions to setup a macos device",
        tasks: vec![
            brew(),
            comms(),
            tech(),
            neovim(),
            asdf(),
            browsers(),
            apps(),
            git(),
            zsh(),
        ],
    }
}

fn git() -> Task {
    Task {
        key: "git",
        actions: vec![
            git_config_global("alias.co", "checkout"),
            git_config_global("alias.br", "branch"),
            git_config_global("alias.st", "status"),
            git_config_global("alias.lol", "log --oneline"),
            git_config_global("alias.c", "commit"),
            git_config_global("alias.cane", "commit --amend --no-edit"),
            git_config_global("alias.cm", "commit -m"),
            git_config_global("alias.ap", "add -p"),
            git_config_global("pull.rebase", "true"),
        ],
    }
}

fn brew() -> Task {
    Task {
        key: "brew",
        actions: vec![Action::Command(vec![
            s!("/bin/bash"),
            s!("-c"),
            s!("$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"),
        ])],
    }
}

fn comms() -> Task {
    Task {
        key: "comms",
        actions: vec![
            brew_install("slack"),
            brew_install("discord"),
            brew_install("signal"),
            brew_install("telegram"),
            brew_install("whatsapp"),
            brew_install("zoom"),
        ],
    }
}

fn tech() -> Task {
    Task {
        key: "tech",
        actions: vec![
            starship(),
            font_fira_code_nerd_font(),
            brew_install("visual-studio-code"),
            brew_install("lazygit"),
            brew_install("lazydocker"),
            brew_install("asdf"),
            brew_install("git"),
            brew_install("gh"),
            brew_install("tree"),
            brew_install("docker"),
            brew_install("wireshark"),
            brew_install("redis"),
        ],
    }
}

fn neovim() -> Task {
    Task {
        key: "neovim",
        actions: vec![
            brew_install("iterm2"),
            brew_install("neovim"),
            brew_install("lazygit"),
            brew_install("bottom"),
            brew_install("ripgrep"),
            cargo_install("tree-sitter-cli"),
        ],
    }
}

fn asdf() -> Task {
    Task {
        key: "asdf",
        actions: vec![
            asdf_plugin_add("nodejs"),
            asdf_plugin_add("erlang"),
            asdf_plugin_add("elixir"),
        ],
    }
}

fn browsers() -> Task {
    Task {
        key: "browsers",
        actions: vec![
            brew_install("google-chrome"),
            brew_install("chromium"),
            brew_install("firefox"),
        ],
    }
}

fn apps() -> Task {
    Task {
        key: "apps",
        actions: vec![
            brew_install("spotify"),
            brew_install("obsidian"),
            brew_install("bitwarden"),
        ],
    }
}

fn zsh() -> Task {
    Task {
        key: "zsh",
        actions: vec![
            zshrc_append_line("alias g='git'"),
            zshrc_append_line("alias ls='ls -G'"),
            zshrc_append_line("alias ll='ls -al'"),
            zshrc_append_line("alias ..='cd ..'"),
            zshrc_append_line("alias nr='npm run'"),
            zshrc_append_line("alias imps='iex -S mix phx.server'"),
        ],
    }
}

/// ## Notes
///
/// Because this is a per-line append, whitespace is provided. One new line will
/// be appended at the end of the line.
fn zshrc_append_line(line: &str) -> Action {
    Action::AppendToFile {
        content: format!("{line}\n"),
        file_path: zshrc_path(),
    }
}

fn zshrc_path() -> PathBuf {
    let mut path = UserDirs::new().unwrap().home_dir().to_path_buf();
    path.push(".zshrc");
    path
}

fn starship() -> Action {
    Action::Task(Task {
        key: "starship",
        actions: vec![
            brew_install("starship"),
            Action::AppendToFile {
                content: s!("\neval \"$(starship init zsh)\"\n"),
                file_path: zshrc_path(),
            },
        ],
    })
}

fn font_fira_code_nerd_font() -> Action {
    Action::Task(Task {
        key: "font_fira_code_nerd_font",
        actions: vec![
            Action::Command(vec![s!("brew"), s!("tap"), s!("homebrew/cask-fonts")]),
            brew_install("font-fira-code-nerd-font"),
        ],
    })
}

fn asdf_plugin_add(arg: &str) -> Action {
    Action::Command(vec![s!("asdf"), s!("plugin"), s!("add"), s!(arg)])
}

fn brew_install(arg: &str) -> Action {
    Action::Command(vec![s!("brew"), s!("install"), s!(arg)])
}

fn git_config_global(keypath: &str, value: &str) -> Action {
    Action::Command(vec![
        s!("git"),
        s!("config"),
        s!("--global"),
        s!(keypath),
        s!(value),
    ])
}

fn cargo_install(arg: &str) -> Action {
    Action::Command(vec![s!("cargo"), s!("install"), s!(arg)])
}
