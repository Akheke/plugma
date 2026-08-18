// src/commands/command.rs
use clap::{Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::enums::output::Output;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// make secret key and public key
    Key {
        #[arg(short, long)]
        force: bool,

        #[arg(short, long)]
        quiet: bool,
    },

    /// register the other person's public key
    Register {
        #[arg(short, long)]
        force: bool,

        #[arg(short, long)]
        quiet: bool,
    },

    /// encrypt data
    Encrypt {
        #[arg(short, long, value_enum)]
        output: Output,

        #[arg(long = "output-path", alias = "op", requires = "output")]
        output_path: Option<PathBuf>,

        #[arg(short = 'E', long)]
        encryptors: PathBuf,

        #[arg(long, alias = "tp", conflicts_with = "target")]
        target_path: Option<PathBuf>,

        #[arg(short, long, conflicts_with = "target_path")]
        target: Option<String>,

        #[arg(short, long)]
        force: bool,

        #[arg(short, long)]
        quiet: bool,
    },

    /// decode data
    Decode {
        #[arg(short, long, value_enum)]
        output: Output,

        #[arg(long = "output-path", alias = "op", requires = "output")]
        output_path: Option<PathBuf>,

        #[arg(short = 'E', long)]
        encryptors: PathBuf,

        #[arg(long, alias = "tp", conflicts_with = "target")]
        target_path: Option<PathBuf>,

        #[arg(short, long, conflicts_with = "target_path")]
        target: Option<String>,

        #[arg(short, long)]
        force: bool,

        #[arg(short, long)]
        quiet: bool,
    },

    /// show info
    Show {
        #[arg(value_enum)]
        target: ShowTarget
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ShowTarget {
    #[value(alias = "keys")]
    #[value(alias = "key")]
    #[value(alias = "Keys")]
    #[value(alias = "Key")]
    #[value(alias = "all_keys")]
    #[value(alias = "allKeys")]
    #[value(alias = "ALL_KEYS")]
    #[value(alias = "ALLKEYS")]
    #[value(alias = "all-keys")]
    #[value(alias = "allkeys")]
    #[value(alias = "0")]
    Keys,
    #[value(alias = "my_public_key")]
    #[value(alias = "myPublicKey")]
    #[value(alias = "MY_PUBLIC_KEY")]
    #[value(alias = "my-public-key")]
    #[value(alias = "MY-PUBLIC-KEY")]
    #[value(alias = "my_public")]
    #[value(alias = "myPublic")]
    #[value(alias = "MY_PUBLIC")]
    #[value(alias = "my-public")]
    #[value(alias = "MY-PUBLIC")]

    #[value(alias = "my_pub_key")]
    #[value(alias = "myPubKey")]
    #[value(alias = "MY_PUB_KEY")]
    #[value(alias = "my-pub-key")]
    #[value(alias = "MY-PUB-KEY")]
    #[value(alias = "my_pub")]
    #[value(alias = "myPub")]
    #[value(alias = "MY_PUB")]
    #[value(alias = "my-pub")]
    #[value(alias = "MY-PUB")]
    #[value(alias = "1")]
    MyPubKey,
    #[value(alias = "their_public_key")]
    #[value(alias = "theirPublicKey")]
    #[value(alias = "THEIR_PUBLIC_KEY")]
    #[value(alias = "their-public-key")]
    #[value(alias = "THEIR-PUBLIC-KEY")]
    #[value(alias = "their_public")]
    #[value(alias = "theirPublic")]
    #[value(alias = "THEIR_PUBLIC")]
    #[value(alias = "their-public")]
    #[value(alias = "THEIR-PUBLIC")]

    #[value(alias = "their_pub_key")]
    #[value(alias = "theirPubKey")]
    #[value(alias = "THEIR_PUB_KEY")]
    #[value(alias = "their-pub-key")]
    #[value(alias = "THEIR-PUB-KEY")]
    #[value(alias = "their_pub")]
    #[value(alias = "theirPub")]
    #[value(alias = "THEIR_PUB")]
    #[value(alias = "their-pub")]
    #[value(alias = "their-PUB")]
    #[value(alias = "2")]
    TheirPubKey,
    #[value(alias = "secret_key")]
    #[value(alias = "secretKey")]
    #[value(alias = "SECRET_KEY")]
    #[value(alias = "secret-key")]
    #[value(alias = "SECRET-KEY")]
    #[value(alias = "secret")]
    #[value(alias = "SECRET")]

    #[value(alias = "sec_key")]
    #[value(alias = "secKey")]
    #[value(alias = "SEC_KEY")]
    #[value(alias = "sec-key")]
    #[value(alias = "SEC-KEY")]
    #[value(alias = "sec")]
    #[value(alias = "SEC")]
    #[value(alias = "3")]
    SecKey,

    #[value(alias = "public")]
    #[value(alias = "PUBLIC")]
    #[value(alias = "pub")]
    #[value(alias = "PUB")]
    #[value(alias = "1.5")]
    PublicKey,
    #[value(alias = "plugin")]
    #[value(alias = "Plugin")]
    #[value(alias = "PLUGIN")]

    #[value(alias = "plugins")]
    #[value(alias = "Plugins")]
    #[value(alias = "PLUGINS")]
    #[value(alias = "4")]
    Plugin
}