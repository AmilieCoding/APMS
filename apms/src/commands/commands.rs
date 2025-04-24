use clap::Command;
use crate::commands::install;

pub fn commands() {
    let matches = Command::new("apms")
        .about("Azine's Package Manager")
        .subcommand(
            Command::new("install")
                .about("Install a package with APMS")
                .arg(clap::Arg::new("package")
                    .help("The package to install")
                    .required(true))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            install::install(package);
        }
        _ => {
            println!("[ERROR] No subcommand provided.");
        }
    }
}