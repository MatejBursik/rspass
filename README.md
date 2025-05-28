# RsPass - CLI Password Manager

A secure command-line password manager written in Rust, featuring encryption and safe memory handling.

## Installation from Source

```bash
git clone <repository-url>
cd rpass
cargo build --release
```

The binary will be available at `target/release/rpass`.

## Usage

### Initialize a New Vault

```bash
rpass init
```

Creates a new encrypted password vault. You'll be prompted to set a master password.

### Add a Password

```bash
# Interactive (prompts for password)
rpass add github

# With password as argument (less secure - visible in shell history)
rpass add github --password mySecretPassword123
```

### Retrieve a Password

```bash
rpass get github
```

### List All Services

```bash
rpass list
```

### Update a Password

```bash
# Interactive
rpass update github

# With password argument
rpass update github --password newPassword456
```

### Remove a Password

```bash
rpass remove github
```

## Disclaimer

This is a personal project. While it implements strong security practices, it hasn't undergone professional security testing and auditing. For production use, consider established password managers like 1Password, Bitwarden, or KeePass.
