use clap::Command;
use rip::prettier;

fn main() {
    let rip = Command::new("rip")
        .about("rip runs a variety of actions")
        .subcommand_required(true)
        .subcommand(
            Command::new("js").about("Javascript rips").subcommand(Command::new("prettier"))
        ).subcommand(
            Command::new("ts").about("Typescript rips")
            .long_about("Most of the js subcommands should also work here. This is only for actions that exclusively interact with TypeScript")
        );

    let matches = rip.get_matches();
    let dir = std::env::current_dir().unwrap();

    match matches.subcommand() {
        Some(("js", sub_matches)) => match sub_matches.subcommand() {
            Some(("prettier", _sub_matches)) => prettier(dir),
            _ => unreachable!(),
        },
        Some(("ts", _sub_matches)) => {
            println!("Im in js")
        }
        _ => unreachable!(),
    }
}
