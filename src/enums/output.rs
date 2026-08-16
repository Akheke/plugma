// src/enums/output.rs
use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq)]
pub enum Output {
    #[value(
        name = "std",
        alias = "stdout",
        alias = "0",
        help = "output to stdout(the --path option is not required)"
    )]
    Std,

    #[value(
        name = "file",
        alias = "1",
        help = "output to a .txt file(requires the --path option)"
    )]
    File,
}
