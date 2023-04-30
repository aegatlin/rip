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
            docker(),
        ],
    }
}

/// An alternative strategy is writing to ~/.gitconfig, which is what the
/// following cli commands do. I think it is an inferior strategy because these
/// cli commands handle formatting, duplication, etc., for me.
fn git() -> Task {
    Task {
        key: "git",
        actions: vec![
            brew_install("git"),
            brew_install("lazygit"),
            brew_install("gh"),
            git_config_global("pull.rebase", "true"),
            git_config_global("user.name", "Austin Gatlin"),
            git_config_global("user.email", "austin@gatlin.io"),
            git_config_global("alias.co", "checkout"),
            git_config_global("alias.br", "branch"),
            git_config_global("alias.st", "status"),
            git_config_global("alias.lol", "log --oneline"),
            git_config_global("alias.c", "commit"),
            git_config_global("alias.cane", "commit --amend --no-edit"),
            git_config_global("alias.cm", "commit -m"),
            git_config_global("alias.ap", "add -p"),
        ],
    }
}

fn brew() -> Task {
    let content = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";

    Task {
        key: "brew",
        actions: vec![Action::CopyToClipboard(s!(content))],
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
            brew_install("tree"),
            brew_install_cask("wireshark"),
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
            brew_install("asdf"),
            asdf_plugin_add("nodejs"),
            asdf_plugin_add("erlang"),
            asdf_plugin_add("elixir"),
        ],
    }
}

fn docker() -> Task {
    Task {
        key: "docker",
        actions: vec![brew_install_cask("docker"), brew_install("lazydocker")],
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
            zshrc_unique_line("alias g='git'"),
            zshrc_unique_line("alias gg='lazygit'"),
            zshrc_unique_line("alias ls='ls -G'"),
            zshrc_unique_line("alias ll='ls -al'"),
            zshrc_unique_line("alias ..='cd ..'"),
            zshrc_unique_line("alias nr='npm run'"),
            zshrc_unique_line("alias imps='iex -S mix phx.server'"),
        ],
    }
}

/// Note: a new-line is appended automatically
fn zshrc_unique_line(line: &str) -> Action {
    Action::UniqueAppendLineToFile {
        line: format!("{line}\n"),
        file_path: zshrc_path(),
    }
}

fn zshrc_path() -> PathBuf {
    let mut path = UserDirs::new().unwrap().home_dir().to_path_buf();
    path.push(".zshrc");
    path
}

fn starship() -> Action {
    let zshrc_mod = Action::UniqueAppendLineToFile {
        line: s!("eval \"$(starship init zsh)\""),
        file_path: zshrc_path(),
    };

    Action::Task(Task {
        key: "starship",
        actions: vec![brew_install("starship"), zshrc_mod],
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

fn brew_install_cask(arg: &str) -> Action {
    Action::Command(vec![s!("brew"), s!("install"), s!("--cask"), s!(arg)])
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
