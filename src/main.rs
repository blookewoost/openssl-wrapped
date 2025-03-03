use std::{fs, io::{self, Write}, process::{Command, Stdio}};

const PASSWD: &str = "A_Totally_Random_Password";

fn main() {
    
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

#[test]
fn test_lock_and_unlock_2048() {
    generate_unencrypted_key("keys/open.pem", 2048).unwrap();
    lock_existing_key("keys/open.pem", "keys/locked.pem", PASSWD).unwrap();
    unlock_encrypted_key("keys/locked.pem", "keys/temp.pem", PASSWD).unwrap();

    let original = fs::read_to_string("keys/open.pem").unwrap();
    let after = fs::read_to_string("keys/temp.pem").unwrap();

    assert_eq!(original, after)
}

#[test]
fn test_lock_and_unlock_1024() {
    generate_unencrypted_key("keys/open.pem", 1024).unwrap();
    lock_existing_key("keys/open.pem", "keys/locked.pem", PASSWD).unwrap();
    unlock_encrypted_key("keys/locked.pem", "keys/temp.pem", PASSWD).unwrap();

    let original = fs::read_to_string("keys/open.pem").unwrap();
    let after = fs::read_to_string("keys/temp.pem").unwrap();

    assert_eq!(original, after)
}
