use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce, aead::Aead};
use hkdf::Hkdf;
use sha2::Sha256;
use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use rand_core::{OsRng, RngCore};

use std::env;
use std::fs;
use std::path::Path;

/*
If this template doesn't work, you might have success by modifying the `plugin/default.exe` file available on GitHub. 
Since this template is based on that file but has had everything removed except the basic framework,
 it is possible it might not work—so please bear with me.
*/

/*
This is the skeleton of a default plugin written in Rust.
I've prepared some functions that might be useful. Feel free to use them if you like.
 */

/*
List of arguments passed by plugma
Please note that the arguments are passed as strings.

args[0] = The path of this plugin(This is due to Rust's `env::args()`)
args[1] = The string to process
args[2] = A string indicating either encryption or decryption.("true" => encryption , "false" => decryption)
args[3] = Whether the user prioritizes forced execution or safe execution.("true" => forced,"false" => not forced)
args[4] = Whether or not the user wants output such as non-essential logs other than the results.("true" => wants,"false" => not wants)
*/


fn load_secret(path: &Path) -> StaticSecret {
    let encoded = fs::read_to_string(path).expect("failed to read secret key file");
    let bytes = STANDARD
        .decode(encoded)
        .expect("failed to decode secret key");

    let arr: [u8; 32] = bytes.try_into().expect("secret key must be 32 bytes");
    StaticSecret::from(arr)
}

fn load_public(path: &Path) -> PublicKey {
    let encoded = fs::read_to_string(path).expect("failed to read public key file");
    let bytes = STANDARD
        .decode(encoded)
        .expect("failed to decode public key");

    let arr: [u8; 32] = bytes.try_into().expect("public key must be 32 bytes");
    PublicKey::from(arr)
}


fn derive_chacha_key(my_secret: &StaticSecret, their_public: &PublicKey) -> Key {
    let shared: SharedSecret = my_secret.diffie_hellman(their_public);
    let shared_bytes = shared.as_bytes();

    let hk = Hkdf::<Sha256>::new(None, shared_bytes);
    let mut key_bytes = [0u8; 32];
    hk.expand(b"msg-encryption", &mut key_bytes)
        .expect("HKDF expand failed");

    Key::from_slice(&key_bytes).clone()
}




fn compare_pub_keys(key1: &PublicKey, key2: &PublicKey) -> bool {
    if key1 == key2 { true } else { false }
}

// ----------------------------
// main
// ----------------------------
fn main() {
    let args: Vec<String> = env::args().collect();

    let exe_path = env::current_exe().expect("failed to get exe path");
    let exe_dir = exe_path.parent().expect("failed to get exe dir");

    let my_secret_path = exe_dir.join("../keys/sec.key");
    let my_public_path = exe_dir.join("../keys/pub.key");
    let their_public_path = exe_dir.join("../keys/their_pub.key");

    match args.len() {
        5 => {
            let code = &args[1];
            let mode = &args[2];

            if mode == "true" {
                let result =

                    //=====================================
                    //Here your code for encrypting message
                    //=====================================

                print!("{}", result);
            } else {
                let parts: Vec<&str> = code.split(':').collect();
                if parts.len() != 2 {
                    eprintln!("invalid ciphertext format. expected nonce_b64:ciphertext_b64");
                    std::process::exit(1);
                }
                let nonce_b64 = parts[0];
                let ciphertext_b64 = parts[1];
                let result =
                    //=====================================
                    //Here your code for decrypting message
                    //=====================================
                print!("{}", result);
            }
        }

        _ => {
            eprintln!("invalid arguments for plugin");
            std::process::exit(1);
        }
    }
}
