use std::{
    env,
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use copypasta::ClipboardProvider;

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

mod mac;

#[macro_export]
macro_rules! s {
    ($s:expr) => {
        String::from($s)
    };
}

mod ts;

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
            .about(&self.description)
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
    Command(Vec<String>),
    /// Append a line to a file **only if** the file does not already contain
    /// that line.
    ///
    /// A new line is appended to the end of `line`
    UniqueAppendLineToFile {
        line: String,
        file_path: PathBuf,
    },
    CopyToClipboard(String),
    ChangeDir(PathBuf),
}

impl Action {
    fn run(&self) {
        match self {
            Action::Task(task) => task.run(),
            Action::Command(command) => {
                let cmd = command.first().expect("command should have command name");
                let args = command[1..].to_vec();
                let mut child = std::process::Command::new(cmd)
                    .args(args)
                    .spawn()
                    .expect("command should be a valid command and not error");

                let child_exit_status = child.wait().expect("child process should complete");
                println!("rip log: child process exit status: {child_exit_status}");
            }
            Action::UniqueAppendLineToFile { line, file_path } => {
                append_line_to_file_unless_present(line, file_path)
            }
            Action::CopyToClipboard(s) => {
                let mut ctx = copypasta::ClipboardContext::new().unwrap();
                ctx.set_contents(s!(s)).unwrap();
                println!("rip log: copied to clipboard: {s}\n");
            }
            Action::ChangeDir(path) => {
                env::set_current_dir(path).unwrap();
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
            Action::UniqueAppendLineToFile { line, file_path } => {
                let n = line.trim();
                let p = file_path.to_str().unwrap();
                format!("append to {p} unless present : {n}")
            }
            Action::CopyToClipboard(s) => {
                format!("copy to clipboard : {s}")
            }
            Action::ChangeDir(path) => {
                format!("change dir to : {}", path.to_str().unwrap())
            }
        }
    }
}

fn append_line_to_file_unless_present(line: &str, file_path: &Path) {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(file_path)
        .unwrap();
    let mut contents = String::from("");

    file.read_to_string(&mut contents).unwrap();

    if !contents.contains(line) {
        file.write_all(format!("{line}\n").as_bytes()).unwrap();
    };
}

#[test]
fn test_append_line_to_file_unless_present() {
    use assert_fs::prelude::*;
    use assert_fs::NamedTempFile;
    use predicates::prelude::*;
    use std::fs;

    let file = NamedTempFile::new("zshrc").unwrap();
    fs::write(file.path(), "uno\ndos\n").unwrap();

    append_line_to_file_unless_present("dos", file.path());
    file.assert(predicate::str::contains("dos\n"));
    file.assert(predicate::str::contains("dos\ndos").not());

    append_line_to_file_unless_present("tres", file.path());
    file.assert(predicate::str::contains("tres\n"));
    file.assert(predicate::str::contains("tres\ntres").not());

    append_line_to_file_unless_present("tres", file.path());
    file.assert(predicate::str::contains("tres\n"));
    file.assert(predicate::str::contains("tres\ntres").not());
}
