# RsPass - CLI Password Manager

A secure command-line password manager written in Rust, featuring encryption and safe memory handling.

## Installation from Source

```bash
git clone <repository-url>
cd rspass
cargo build --release
```

The binary will be available at `target/release/rspass`.

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

# With password as an argument (less secure - visible in shell)
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

# With password as an argument (less secure - visible in shell)
rspass update github --password newPassword456
```

### Remove a Password

```bash
rspass remove github
```

## Disclaimer

This is a personal project. While it implements strong security practices, it hasn't undergone professional security testing and auditing. For production use, consider established password managers like 1Password, Bitwarden, or KeePass.
