use std::{
    env::{current_dir, var_os},
    path::PathBuf,
    process,
};

use clap::Command;

fn main() {
    let workspace_dir = var_os("CARGO_WORKSPACE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| current_dir().unwrap());

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("backend", _)) => {
            process::Command::new("cargo")
                .arg("run")
                .arg("-p")
                .arg("backend")
                .current_dir(&workspace_dir)
                .stdout(process::Stdio::inherit())
                .stderr(process::Stdio::inherit())
                .status()
                .expect("running backend");
        }
        Some(("frontend", _)) => {
            process::Command::new("pnpm")
                .args(["run", "dev"])
                .current_dir(workspace_dir.join("web"))
                .stdout(process::Stdio::inherit())
                .stderr(process::Stdio::inherit())
                .status()
                .expect("Running Frontend dev mode");
        }
        Some(("entity", submatches)) => match submatches.subcommand() {
            Some(("generate", _)) => {
                process::Command::new("sea-orm-cli")
                    .arg("generate")
                    .arg("entity")
                    .arg("-o")
                    .arg("crates/backend/src/db/entity/")
                    .arg("--with-serde")
                    .arg("both")
                    .current_dir(&workspace_dir)
                    .stdout(process::Stdio::inherit())
                    .stderr(process::Stdio::inherit())
                    .status()
                    .expect("running entity generate");
            }
            Some(("clean", _)) => {
                let dir = workspace_dir.join("crates/backend/src/db/entity");
                let files = dir.read_dir().expect("Failed to read entity directory");
                for file in files {
                    let file = file.expect("failed to get file path");
                    if file.file_name() == "lib.rs" {
                        continue;
                    }
                    let file_path = file.path();
                    match std::fs::remove_file(&file_path) {
                        Ok(_) => println!("Removed file {}", file_path.display()),
                        Err(_) => println!("Failed to remove file {}", file_path.display()),
                    }
                }
            }
            _ => {
                panic!(
                    "Unknown command: entity {:?}",
                    submatches.subcommand().map(|c| c.0)
                )
            }
        },
        _ => {
            panic!("Unknown command: {:?}", matches.subcommand().map(|c| c.0))
        }
    }
}

fn cli() -> Command {
    Command::new("xtask")
        .about("docusphere useful commands")
        .subcommand_required(true)
        .subcommand(Command::new("backend"))
        .subcommand(Command::new("frontend"))
        .subcommand(Command::new("start"))
        .subcommand(
            Command::new("entity")
                .subcommand_required(true)
                .subcommand(Command::new("generate"))
                .subcommand(Command::new("clean")),
        )
}
