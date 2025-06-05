# RsPass - CLI Password Manager

A secure command-line password manager written in Rust, featuring encryption and safe memory handling.

## Features

- **Secure Storage**: Passwords are encrypted using AES-256-GCM
- **Key Hashing**: Master password is hashed with Argon2
- **Simple CLI**: Easy-to-use command-line interface
- **Cross-Platform**: Compatible with Windows, macOS, and Linux

### What RsPass Does NOT Do

- **No Cloud Sync**: Vault is stored locally only
- **No Interface**: Command-line only (no graphical interface)
- **No Browser Integration**: Manual copy/paste is required

## Installation from Source

```bash
git clone <repository-url>
cd rspass
cargo build --release
```

The binary will be available at `target/release/rspass`.

## Storage Location

Vault files are stored at:
- **Linux/macOS**: `~/.config/rspass/vault.enc`
- **Windows**: `%USERPROFILE%\.config\rspass\vault.enc`


## Usage

### Initialize a New Vault

```bash
rspass init
```

Creates a new encrypted password vault. You'll be prompted to set a master password.

### Add a Password

```bash
# Interactive (prompts for password)
rspass add github

# With password as an argument (less secure - visible in shell history)
rspass add github --password mySecretPassword123
```

### Retrieve a Password

```bash
rspass get github
```

### List All Services

```bash
rspass list
```

### Update a Password

```bash
# Interactive (prompts for password)
rspass update github

# With password as an argument (less secure - visible in shell history)
rspass update github --password newPassword456
```

### Remove a Password

```bash
rspass remove github
```

## Disclaimer

This is a personal project. While it follows strong security practices, it has not undergone professionally security testing or auditing. For production use, consider established password managers like 1Password, Bitwarden, or KeePass.
