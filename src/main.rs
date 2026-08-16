//use base64::{Engine as _, engine::general_purpose};
use clap::{Parser};
use std::fs;
//use std::io::{Write};
//use std::os::unix::fs::PermissionsExt;
//use std::process;
//use crossterm::event::{read, Event};

use std::path::Path;

//enums
mod enums;
use crate::enums::commands::Command;
use crate::enums::commands::ShowTarget;

mod func;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}



fn main() {
    let args = Args::parse();
    let my_secret_path = Path::new("keys/sec.key");
    let my_public_path = Path::new("keys/pub.key");
    let their_public_path = Path::new("keys/their_pub.key");

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

            let result =
                func::cryptography(encryptors.as_path(), force, quiet, target, true, false);

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

        Command::Show { target } => {
           match target {
            ShowTarget::Keys => {
                let files = vec![my_secret_path,my_public_path,their_public_path];
                for file in files {
                    match fs::read_to_string(file) {
                        Ok(content) => {
                            println!("{}: {}", file.to_string_lossy(), content);
                        },
                        Err(e) => {
                            println!("{}:[ERROR] failed to read ({})", file.to_string_lossy(),{e})
                        }
                    }
                }
            },
            ShowTarget::PubKey => {
                match fs::read_to_string(my_public_path) {
                    Ok(content) => {
                        println!("{}", content);
                    },
                    Err(e) => {
                        println!("[ERROR] failed to read public key.\n{}", {e});
                    }
                }
            },
            ShowTarget::SecKey => {
                match fs::read_to_string(my_secret_path) {
                    Ok(content) => {
                        println!("{}", content);
                    },
                    Err(e) => {
                        println!("[ERROR] failed to read public key.\n{}", {e});
                    }
                }
            },
           }
        }
    }
}
