use clap::Command;
use crate::commands::install;
use crate::commands::delete;

pub fn commands() {
    let matches = Command::new("apms")
        .about("APMS - The Azine Package Management System")
        .subcommand(
            Command::new("install")
                .about("Install a package with APMS")
                .arg(clap::Arg::new("package")
                    .help("The package to install")
                    .required(true))
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a package with APMS")
                .arg(clap::Arg::new("package")
                    .help("The package to delete")
                    .required(true))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            install::install(package);
        }
        Some(("delete", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            delete::delete(package);
        }
        _ => {
            println!("[ERROR] No subcommand provided.");
        }
    }
}