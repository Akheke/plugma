# plugma for Windows11 & Linux(ver0.2.3)
"plugma" is a modular encryption CLI powered by executable plugins—mix, match, or swap algorithms effortlessly.

## DEMO
This is a free tool that allows two people to exchange encrypted messages once they have exchanged a shared key.
It was created to help users learn about encryption and to perform encryption using their own encryption algorithms.

## Features
Executable Plugin Architecture: Encryption processors run as standalone binaries, making them easy to chain or replace.

## About Plugins in This Tool
This tool performs encryption by calling external executable files. You can invoke these processes by specifying the absolute paths to the executable files in a file named .order, which is stored in the plugma configuration directory.

## Changes from the previous version(ver0.2.2)
- The `set` command has been added
- To improve usability, I have reorganized the aliases for commands and options


## Requirements

- Rust compiler: rustc 1.96.0 (ac68faa20 2026-05-25)
- Cargo (included with Rust)

## Dependencies (Cargo.toml)
The following crates are used in this project:

- base64 = "0.22.1"
- chacha20poly1305 = "0.10"
- clap = { version = "4.6.1", features = ["derive"] }
- hex = "0.4.3"
- rand = "0.8"
- hkdf = "0.12"
- sha2 = "0.10"
- x25519-dalek = "1"
- rand_core = "0.5"
- crossterm = "0.29.0"
- dirs-next = "2.0.0"


Cargo automatically resolves and installs all dependencies.

## Quick start
As a premise, you have to prepare follow things:
- download plugma and add it to your PATH
### 1.install CLI
If you can use Cargo, you can install it with command
```
cargo install plugma
```
### 2.generate secret & public key
```
plugma key
```
### 3.register the other person’s public key
```
plugma register
```
### 4.encrypt text
```
plugma enc -o std -u default -t <your text>
```
### 5.decode text
```
plugma dec -o std -u default -t <your text>
```

## Setting up Dependencies

All dependencies listed in `Cargo.toml` can be installed automatically using Cargo:

```bash
cargo fetch
```
## Usage
### 1.Implement plugma
- When downloading binaries:
    It should work if you run the executable file named “plugma.exe” or “plugma.” I recommend adding it to your PATH.

- Build the project:

```bash
cargo build
```

- If cargo is available in your environment,you can install it using the following command:
```bash
cargo install plugma
```

### 2.Setting Up the Environment
Please launch plugma. plugma will set up the environment the first time you launch it.
```
plugma -V
```

### 3.Create your private/public key pair.

```
plugma key
```

### 4.Display the public key.

```
plugma show myPub
```
Send public key to the person you're communicating with by any means you choose.

### 5.Register the other party's public key in plugma

```
plugma reg
```
When you run the command above, you will be prompted to enter a public key, so please enter the other party's public key.

### 6.Encrypt Text
Please specify the name of the .order file without the file extension.
If you haven't made any changes, it will work if you specify “default”


- When copying content
```
plugma enc -o std -u <.order file path> -t <your text>
```
- When outputting to a file
```
plugma enc -o file --op <file path> -u <.order file path> -t <your text>
```
- When loading a file to be encrypted
```
plugma enc -o std -u <.order file path> --tp <your file path>
```

### Decode Text

- When copying content
```
plugma dec -o std -u <.order file> -t <your text>
```
- When outputting to a file
```
plugma dec -o file --op <file path> -u <.order file> -t <your text>
```
- When loading a file to be encrypted
```
plugma dec -o std -u <.order file> --tp <your file path>
```

## Note
This tool was developed by an individual and is unstable.
I recommend using it solely for recreational purposes.
The creator assumes no responsibility for any damages resulting from the use of this tool.


## For Users Installing Non-Default plugins
Please note that when installing a non-default plugin, in order for `plugma` to recognize the .order file, you must specify the absolute path to the plugin’s executable file and then place that file in the `plugma/plugma_data/plugin/` directory. The `plugma` directory should be located within your user’s configuration directory. For now, you can’t go wrong by placing it in the same directory as the `default.order` file. If it’s still not found,Either run “plugma show key” to check the path, or try searching the C: drive.

## To Developers
To install a plugin for encryption processing, the following requirements must be met:
Place a .order file containing the absolute path to the plugin’s executable file in the `plugma/plugma_data/plugin/` directory. (The name of the .order file will be recognized by plugma as the plugin name.)

Note: When including the absolute paths of multiple executable files in a .order file, separate the paths with a semicolon (;).
As a reference for plugin development, I have included the default implementation code as a template.

## Author
* Akheke

## License

"plugma" is under [MIT license](https://en.wikipedia.org/wiki/MIT_License) or [Apache-2.0](https://en.wikipedia.org/wiki/Apache_License).

Have fun!!
Thank you!