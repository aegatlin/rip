fn main() {
    let app = App {
        namespaces: vec![ts(), ex()],
    };

    let cmd = clap::command!()
        .subcommand_required(true)
        .subcommand(ts().to_clap_subcommand())
        .subcommand(ex().to_clap_subcommand());

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

fn ex() -> Namespace {
    Namespace {
        key: "ex",
        description: "elixir",
        tasks: vec![],
    }
}

fn ts() -> Namespace {
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

struct Namespace {
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
struct Task {
    key: &'static str,
    actions: Vec<Action>,
}

impl Task {
    fn run(&self) {
        self.actions.iter().for_each(|action| action.run())
    }

    fn to_clap_subcommand(&self) -> clap::Command {
        clap::Command::new(self.key)
    }
}

#[derive(Clone)]
enum Action {
    Task(Task),
    Command(Vec<&'static str>),
}

impl Action {
    fn run(&self) {
        match self {
            Action::Task(task) => task.run(),
            Action::Command(command) => {
                let cmd = command.first().expect("command should have command name");
                let args = command[1..].to_vec();
                std::process::Command::new(cmd)
                    .args(args)
                    .output()
                    .expect("command should be a valid command and not error");
            }
        }
    }
}
