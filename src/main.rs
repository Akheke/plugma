//use base64::{Engine as _, engine::general_purpose};
use clap::Parser;
use std::fs;
//use std::io::{Write};
//use std::os::unix::fs::PermissionsExt;
//use std::process;
//use crossterm::event::{read, Event};

use std::path::Path;
//use std::path::PathBuf;


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
    //self installer
    let exe = std::env::current_exe().expect(
        "[ERROR]\ncould not get path\nI recommend downloading the release version from GitHub.\n\
            Link: https://github.com/Akheke/plugma\n");
    let real_exe = std::fs::canonicalize(exe).expect(
        "[ERROR]\ncould not canonicalize path\nI recommend downloading the release version from GitHub.\n\
            Link: https://github.com/Akheke/plugma\n");
    let base_dir = real_exe.parent().unwrap();
    let plugma_dir = base_dir.join("plugma");

    println!("base_dir = {:?}", base_dir);


    if !plugma_dir.exists() {


        println!("\n==============================\n\
        Did you install this tool using `cargo install`?\n\
        Don’t worry—unlike versions prior to 0.1.3, this version is designed to work properly! (Probably...)\n\
        If anyone installed a version earlier than 0.1.3 using `cargo install`, I’m sorry!!\n\
        From the developer, Akheke\n\
        \n==============================\n"
    );


        if let Err(e) = fs::create_dir(&plugma_dir) {

            eprintln!(
            "[ERROR]\n\
            Failed to set up the environment required for plugma to run.\n\
            I recommend downloading the release version from GitHub.\n\
            Link: https://github.com/Akheke/plugma\n\
            Reason for the error:\n\
            {}",e
            )
        }

        let plugma_key_dir = plugma_dir.join("keys");

        if let Err(e) = fs::create_dir(plugma_key_dir) {

            eprintln!(
                "[WARNING]\n\
                It looks like an error occurred during the initial setup when you first launched the app.\n\
                It might resolve itself automatically, so try typing “plugma key”\n\
                (Oh! If you've saved your key, make sure to back it up somewhere else first).\n\
                If that doesn't work, I recommend downloading the binary from this link!\n\
                https://github.com/Akheke/plugma\n\
                Reason for the error:\n\
            {}",e
            )
        }

        let plugma_plugin_dir = plugma_dir.join("plugin");

        if let Err(e) = fs::create_dir(&plugma_plugin_dir) {

            eprintln!(
                "[ERROR]\n\
                The environment setup failed during the initial startup.\n\
                We recommend downloading the binary from the link below.\n\
                https://github.com/Akheke/plugma\n\
                Reason for the error:\n\
            {}",e
            )
        }

        let plugma_order_file = plugma_plugin_dir.join("order.order");

        #[cfg(target_os = "windows")]
        const DEFAULT_NAME: &str = "plugma_default.exe";

        #[cfg(not(target_os = "windows"))]
        const DEFAULT_NAME: &str = "plugma_default";

        let plugma_order_content = String::from(DEFAULT_NAME);

        if let Err(e) = fs::write(plugma_order_file, plugma_order_content) {

            eprintln!(
                "[ERROR]\n\
                The environment setup failed during the initial startup.\n\
                We recommend downloading the binary from the link below.\n\
                https://github.com/Akheke/plugma\n\
                Reason for the error:\n\
            {}",e
            )
        }
    
        //move plugma_default.exe
        let plugma_default_path = base_dir.join(DEFAULT_NAME);
        let dest = plugma_plugin_dir.join(DEFAULT_NAME);
        fs::rename(plugma_default_path, dest).expect("[ERROR]\n\
                The environment setup failed during the initial startup.\n\
                We recommend downloading the binary from the link below.\n\
                https://github.com/Akheke/plugma\n");
    }

    let args = Args::parse();
    let my_secret_path = Path::new("plugma/keys/sec.key");
    let my_public_path = Path::new("plugma/keys/pub.key");
    let their_public_path = Path::new("plugma/keys/their_pub.key");

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

        Command::Show { target } => match target {
            ShowTarget::Keys => {
                let files = vec![my_secret_path, my_public_path, their_public_path];
                for file in files {
                    match fs::read_to_string(file) {
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
                    match fs::read_to_string(file) {
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
        },
    }
}
