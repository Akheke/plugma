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
    Keys,
    #[value(alias = "public_key")]
    #[value(alias = "publicKey")]
    #[value(alias = "PUBLIC_KEY")]
    #[value(alias = "public-key")]
    #[value(alias = "PUBLIC-KEY")]
    #[value(alias = "public")]
    #[value(alias = "PUBLIC")]

    #[value(alias = "pub_key")]
    #[value(alias = "pubKey")]
    #[value(alias = "PUB_KEY")]
    #[value(alias = "pub-key")]
    #[value(alias = "PUB-KEY")]
    #[value(alias = "pub")]
    #[value(alias = "PUB")]
    PubKey,
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
    SecKey,
}