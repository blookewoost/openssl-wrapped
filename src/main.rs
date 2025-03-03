use std::{fs, io::{self, Write}, process::{Command, Stdio}};

fn main() {
    let password = "ARandomPassPhraseOhYeah";
    let bits = 2048;
    let key_open = "keys/open.priv.pem";
    let key_locked = "keys/locked.priv.pem";
    let key_temp = "keys/temp.priv.pem";

    // Generate an unencrypted RSA private key.
    generate_unencrypted_key(key_open, bits).unwrap();
    lock_existing_key(key_open, key_locked, password).unwrap();
    unlock_encrypted_key(key_locked, key_temp, password).unwrap();

    let original = fs::read_to_string(key_open).unwrap();
    let after = fs::read_to_string(key_temp).unwrap();

    assert_eq!(original, after)
}

fn generate_unencrypted_key(outfile: &str, bits: u32) -> Result<bool, io::Error> {
    let bitstring = bits.to_string();
    let mut child = Command::new("openssl")
        .arg("genrsa")
        .arg("-out")
        .arg(outfile)
        .arg(bitstring)
        .spawn()?;

    let status = child.wait()?;
    Ok(status.success())
}

// openssl rsa -in decrypted_key.pem -aes256 -passout stdin -out encrypted_key.pem
fn lock_existing_key(infile: &str, outfile: &str, password: &str) -> Result<bool, io::Error> {
    let mut child = Command::new("openssl")
        .arg("rsa")
        .arg("-in")
        .arg(infile)
        .arg("-aes256")
        .arg("-passout")
        .arg("stdin")
        .arg("-out")
        .arg(outfile)
        .stdin(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().take().unwrap();
    writeln!(child_stdin, "{password}")?;

    let status = child.wait()?;
    Ok(status.success())
}

fn generate_encrypted_key(outfile: &str, bits: u32, password: &str) -> Result<bool, io::Error> {
    let bitstring = bits.to_string();
    let mut child = Command::new("openssl")
        .arg("genrsa")
        .arg("-aes256")
        .arg("-passout")
        .arg("stdin")
        .arg("-out")
        .arg(outfile)
        .arg(bitstring)
        .stdin(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().take().unwrap();
    writeln!(child_stdin, "{password}")?;

    let status = child.wait()?;

    Ok(status.success())
}

fn unlock_encrypted_key(infile: &str, outfile: &str, password: &str) -> Result<bool, io::Error> {
    let mut child = Command::new("openssl")
        .arg("rsa")
        .arg("-in")
        .arg(infile)
        .arg("-passin")
        .arg("stdin")
        .arg("-out")
        .arg(outfile)
        .stdin(Stdio::piped())
        .spawn()?;

    let child_stdin = child.stdin.as_mut().take().unwrap();
    writeln!(child_stdin, "{password}")?;

    let status = child.wait()?;

    Ok(status.success())
}
