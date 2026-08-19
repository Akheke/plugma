//use base64::{Engine as _, engine::general_purpose};
use clap::Parser;
use std::env;
use std::fs;
//use std::io::{Write};
//use std::os::unix::fs::PermissionsExt;
//use std::process;
//use crossterm::event::{read, Event};

//use std::path::Path;
use std::path::PathBuf;

//enums
mod enums;
use crate::enums::commands::Command;
use crate::enums::commands::ShowTarget;
use crate::func::find_order_files;

mod func;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    // self installer
    let plugma_dir = dirs_next::config_dir()
        .expect("[ERROR]\nCould not find config directory\n")
        .join("plugma")
        .join("plugma_data");

    if !plugma_dir.exists() {
        println!(
            "\n==============================\n\
            First-time setup detected.\n\
            plugma_data directory will be created in your system config directory.\n\
            This location is stable and works regardless of how you installed plugma.\n\
            From the developer, Akheke\n\
            ===============================\n"
        );

        if let Err(e) = fs::create_dir_all(&plugma_dir) {
            eprint!(
                "[ERROR]\n\
                Failed to create plugma_data directory.\n\
                Reason:\n{}",
                e
            );
        }

        //keys
        let plugma_key_dir = plugma_dir.join("keys");
        if let Err(e) = fs::create_dir_all(&plugma_key_dir) {
            eprintln!(
                "[ERROR]\n\
                Failed to create plugin directory.\n\
                Reason:\n{}",
                e
            );
        }

        //plugin
        let plugma_plugin_dir = plugma_dir.join("plugin");
        if let Err(e) = fs::create_dir_all(&plugma_plugin_dir) {
            eprintln!(
                "[ERROR]\n\
                Failed to create plugin directory.\n\
                Reason:\n{}",
                e
            );
        }

        //find default plugin
        let mut plugins: Vec<PathBuf> = Vec::new();
        let path_var = match env::var("PATH") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[ERROR]\nFailed to get PATH: {}", e);
                String::new()
            }
        };

        for dir in env::split_paths(&path_var) {
            if let Ok(entries) = fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    let path = entry.path();

                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with("plugma-default") {
                            plugins.push(path);
                        }
                    }
                }
            }
        }
        if plugins.is_empty() {
            let current_dir = std::env::current_dir().expect("[ERROR]\nCould not get current dir");

            if let Ok(entries) = fs::read_dir(&current_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.starts_with("plugma-default") {
                            plugins.push(path);
                        }
                    }
                }
            }
        }

        if plugins.is_empty() {
            eprintln!("Failed to get default plugin")
        }

        // default.order
        let plugma_order_file = plugma_plugin_dir.join("default.order");
        let order_content = plugins
            .iter()
            .map(|p| p.to_string_lossy())
            .collect::<Vec<_>>()
            .join(";");

        /*
        #[cfg(target_os = "windows")]
        const DEFAULT_NAME: &str = "plugma_default.exe";

        #[cfg(not(target_os = "windows"))]
        const DEFAULT_NAME: &str = "plugma_default";

         */
        if let Err(e) = fs::write(&plugma_order_file, order_content) {
            eprintln!(
                "[ERROR]\n\
            Failed to write order.order.\n\
            Reason:\n{}",
                e
            );
        }
    }
    let args = Args::parse();
    let my_secret_path = plugma_dir.join("keys/sec.key");
    let my_public_path = plugma_dir.join("keys/pub.key");
    let their_public_path = plugma_dir.join("keys/their_pub.key");

    let _ = their_public_path;

    match args.command {
        Command::Key { force, quiet } => {
            func::create_key(force, quiet, &my_secret_path, &my_public_path);
            println!("Key generation completed.");
        }

        Command::Register { force, quiet } => {
            let _ = force;

            // 対話型で公開鍵を入力
            let their_pub = func::read_user_input("please enter public key(Base64): ");

            if their_pub.is_empty() {
                println!("public key is empty.please try again.");
                return;
            }

            func::register_their_public(&their_pub, &their_public_path, force, quiet);
        }

        Command::Encrypt {
            output,
            output_path,
            encryptors,
            target_path,
            target,
            force,
            quiet,
        } => {
            let target = func::get_target(&target_path, &target.unwrap_or_default(), quiet);

            let plugin = func::plugin_to_path(
                &plugma_dir.join("plugin"),
                &encryptors.to_string_lossy().into_owned(),
                &String::from("order"),
            );
            let result = func::cryptography(&plugin, force, quiet, target, true, false);

            func::handle_output(output, output_path, result, quiet, force);
        }

        Command::Decode {
            output,
            output_path,
            encryptors,
            target_path,
            target,
            force,
            quiet,
        } => {
            let target = func::get_target(&target_path, &target.unwrap_or_default(), quiet);

            let result =
                func::cryptography(encryptors.as_path(), force, quiet, target, false, true);

            func::handle_output(output, output_path, result, quiet, force);
        }

        Command::Show { target } => match target {
            ShowTarget::Keys => {
                let files = vec![my_secret_path, my_public_path, their_public_path];
                for file in files {
                    match fs::read_to_string(&file) {
                        Ok(content) => {
                            println!("{}: {}", file.to_string_lossy(), content);
                        }
                        Err(e) => {
                            println!("{}:[ERROR] failed to read ({})", file.to_string_lossy(), {
                                e
                            })
                        }
                    }
                }
            }
            ShowTarget::MyPubKey => match fs::read_to_string(my_public_path) {
                Ok(content) => {
                    println!("{}", content);
                }
                Err(e) => {
                    println!("[ERROR] failed to read your public key.\n{}", { e });
                }
            },
            ShowTarget::TheirPubKey => match fs::read_to_string(their_public_path) {
                Ok(content) => {
                    println!("{}", content);
                }
                Err(e) => {
                    println!("[ERROR] failed to read their public key.\n{}", { e });
                }
            },
            ShowTarget::SecKey => match fs::read_to_string(my_secret_path) {
                Ok(content) => {
                    println!("{}", content);
                }
                Err(e) => {
                    println!("[ERROR] failed to read public key.\n{}", { e });
                }
            },
            ShowTarget::PublicKey => {
                let files = vec![my_public_path, their_public_path];
                for file in files {
                    match fs::read_to_string(&file) {
                        Ok(content) => {
                            println!("{}: {}", file.to_string_lossy(), content);
                        }
                        Err(e) => {
                            println!("{}:[ERROR] failed to read ({})", file.to_string_lossy(), {
                                e
                            })
                        }
                    }
                }
            }
            ShowTarget::Plugin => {
                let plugin_dir = plugma_dir.join("plugin");
                let plugins = match find_order_files(&plugin_dir.as_path(), &String::from("order"))
                {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!(
                            "[ERROR]\nThe process was terminated due to an error.\n{}",
                            e
                        );
                        std::process::exit(1);
                    }
                };
                let output = plugins
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                println!("{}", output);
            }
        },
    }
}
