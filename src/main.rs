use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn main() {
    let app = App {
        namespaces: vec![ts::namespace(), mac::namespace()],
    };

    let cmd = clap::command!()
        .subcommand_required(true)
        .subcommand(ts::namespace().to_clap_subcommand())
        .subcommand(mac::namespace().to_clap_subcommand());

    match cmd.get_matches().subcommand() {
        Some((namespace_name, arg_matches)) => {
            let namespace = app.find_namespace(namespace_name);
            namespace.run(arg_matches);
        }
        None => unreachable!(),
    };
}

struct App {
    namespaces: Vec<Namespace>,
}

impl App {
    fn find_namespace(&self, namespace_name: &str) -> &Namespace {
        self.namespaces
            .iter()
            .find(|n| n.key == namespace_name)
            .expect("namespace should exist due to clap construction")
    }
}

mod mac {
    use directories::UserDirs;

    use crate::{Action, Namespace, Task};

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
            ],
        }
    }

    fn brew() -> Task {
        Task {
            key: "brew",
            actions: vec![Action::Command(vec![
                "/bin/bash",
                "-c",
                "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)",
            ])],
        }
    }

    fn comms() -> Task {
        Task {
            key: "comms",
            actions: vec![
                Action::Command(brew_install("slack")),
                Action::Command(brew_install("discord")),
                Action::Command(brew_install("signal")),
                Action::Command(brew_install("telegram")),
                Action::Command(brew_install("whatsapp")),
                Action::Command(brew_install("zoom")),
            ],
        }
    }

    fn tech() -> Task {
        Task {
            key: "tech",
            actions: vec![
                Action::Task(starship()),
                Action::Task(font_fira_code_nerd_font()),
                Action::Command(brew_install("visual-studio-code")),
                Action::Command(brew_install("lazygit")),
                Action::Command(brew_install("lazydocker")),
                Action::Command(brew_install("asdf")),
                Action::Command(brew_install("git")),
                Action::Command(brew_install("gh")),
                Action::Command(brew_install("tree")),
                Action::Command(brew_install("docker")),
                Action::Command(brew_install("wireshark")),
                Action::Command(brew_install("redis")),
            ],
        }
    }

    fn neovim() -> Task {
        Task {
            key: "neovim",
            actions: vec![
                Action::Command(brew_install("iterm2")),
                Action::Command(brew_install("neovim")),
                Action::Command(brew_install("lazygit")),
                Action::Command(brew_install("bottom")),
                Action::Command(brew_install("ripgrep")),
                Action::Command(cargo_install("tree-sitter-cli")),
            ],
        }
    }

    fn asdf() -> Task {
        Task {
            key: "asdf",
            actions: vec![
                Action::Command(asdf_plugin_add("nodejs")),
                Action::Command(asdf_plugin_add("erlang")),
                Action::Command(asdf_plugin_add("elixir")),
            ],
        }
    }

    fn browsers() -> Task {
        Task {
            key: "browsers",
            actions: vec![
                Action::Command(brew_install("google-chrome")),
                Action::Command(brew_install("chromium")),
                Action::Command(brew_install("firefox")),
            ],
        }
    }

    fn apps() -> Task {
        Task {
            key: "apps",
            actions: vec![
                Action::Command(brew_install("spotify")),
                Action::Command(brew_install("obsidian")),
                Action::Command(brew_install("bitwarden")),
            ],
        }
    }

    fn starship() -> Task {
        let mut path = UserDirs::new().unwrap().home_dir().to_path_buf();
        path.push(".zshrc");

        let x = "\neval \"$(starship init zsh)\"\n".to_owned();
        Task {
            key: "starship",
            actions: vec![
                Action::Command(brew_install("starship")),
                Action::AppendToFile {
                    content: x,
                    file_path: path,
                },
            ],
        }
    }

    fn font_fira_code_nerd_font() -> Task {
        Task {
            key: "font_fira_code_nerd_font",
            actions: vec![
                Action::Command(vec!["brew", "tap", "homebrew/cask-fonts"]),
                Action::Command(brew_install("font-fira-code-nerd-font")),
            ],
        }
    }

    fn asdf_plugin_add(arg: &str) -> Vec<&str> {
        vec!["asdf", "plugin", "add", arg]
    }

    fn cargo_install(arg: &str) -> Vec<&str> {
        vec!["cargo", "install", arg]
    }

    fn brew_install(arg: &str) -> Vec<&str> {
        vec!["brew", "install", arg]
    }
}

mod ts {
    use crate::{Action, Namespace, Task};

    pub fn namespace() -> Namespace {
        Namespace {
            key: "ts",
            description: "typescript",
            tasks: vec![prettier(), skooh()],
        }
    }

    fn prettier() -> Task {
        Task {
            key: "prettier",
            actions: vec![
                Action::Command(npm_install_save_dev("prettier")),
                Action::Command(npm_pkg_set("scripts.format=prettier --write src")),
            ],
        }
    }

    fn skooh() -> Task {
        Task {
            key: "skooh",
            actions: vec![
                Action::Command(npm_install_save_dev("skooh")),
                Action::Command(npm_pkg_set("hooks.pre-commit=npm run format")),
                Action::Command(npm_pkg_set("hooks.pre-push=npm run test")),
            ],
        }
    }

    fn npm_install_save_dev(arg: &str) -> Vec<&str> {
        vec!["npm", "install", "--save-dev", arg]
    }

    fn npm_pkg_set(arg: &str) -> Vec<&str> {
        vec!["npm", "pkg", "set", arg]
    }
}

pub struct Namespace {
    key: &'static str,
    description: &'static str,
    tasks: Vec<Task>,
}

impl Namespace {
    fn find_task(&self, task_name: &str) -> &Task {
        self.tasks
            .iter()
            .find(|t| t.key == task_name)
            .expect("task should exist because of clap setup")
    }

    fn to_clap_subcommand(&self) -> clap::Command {
        let subs: Vec<clap::Command> = self.tasks.iter().map(|t| t.to_clap_subcommand()).collect();
        clap::Command::new(self.key)
            .subcommand_required(true)
            .subcommands(subs)
            .about(self.description)
    }

    fn run(&self, arg_matches: &clap::ArgMatches) {
        match arg_matches.subcommand() {
            Some((task_name, _arg_matches)) => {
                let task = self.find_task(task_name);
                task.run();
            }
            None => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct Task {
    key: &'static str,
    actions: Vec<Action>,
}

impl Task {
    fn run(&self) {
        self.actions.iter().for_each(|action| action.run())
    }

    fn to_clap_subcommand(&self) -> clap::Command {
        clap::Command::new(self.key).about(self.about())
    }

    fn about(&self) -> String {
        let init = format!("`{}` will run the following commands:\n", self.key);
        let mut about = self.actions.iter().fold(init, |mut acc, e| {
            let s = format!("\n{}", e.about());
            acc.push_str(&s);
            acc
        });
        about.push_str("\n");
        about
    }
}

#[derive(Clone)]
enum Action {
    Task(Task),
    Command(Vec<&'static str>),
    /// ## Notes
    ///
    /// Whitespace is **not** implicitly applied to lines. Add it as desired to
    /// `content`.
    AppendToFile {
        content: String,
        file_path: PathBuf,
    },
}

impl Action {
    fn run(&self) {
        match self {
            Action::Task(task) => task.run(),
            Action::Command(command) => {
                let cmd = command.first().expect("command should have command name");
                let args = command[1..].to_vec();
                let mut x = std::process::Command::new(cmd)
                    .args(args)
                    .spawn()
                    .expect("command should be a valid command and not error");

                let y = x.wait().expect("child process should complete");
                println!("process exited with exit status {y}");
            }
            Action::AppendToFile { content, file_path } => {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path)
                    .unwrap();
                file.write_all(content.as_bytes()).unwrap();
            }
        }
    }

    fn about(&self) -> String {
        match self {
            Action::Task(t) => {
                let mut s = format!("for sub-task {}:\n\n", t.key);
                t.actions
                    .iter()
                    .for_each(|a| s.push_str(format!("  {}\n", a.about()).as_str()));
                s.push_str("\n");
                s
            }
            Action::Command(v) => v.join(" "),
            Action::AppendToFile { content, file_path } => {
                let n = content.trim();
                let p = file_path.to_str().unwrap();
                format!("append to {p} : {n}")
            }
        }
    }
}
