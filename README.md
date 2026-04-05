# fast-crlite-query

High-performance CLI utility for mass certificate revocation checking using Mozilla's CRLite (Clubcard/Ribbon) filters.

## Overview

`fast-crlite-query` provides a privacy-preserving and ultra-fast way to check SSL/TLS certificate revocation status locally. Unlike OCSP, it does not leak query data to third parties and can process millions of certificates per second.

This tool implements support for the modern **Clubcard/Ribbon** filter format and correctly handles **Signed Certificate Timestamps (SCTs)**, ensuring browser-level precision in revocation detection.

## Features

- **Blazing Fast**: Process bulk certificate lists at local memory speeds.
- **Privacy First**: Fully offline queries. No OCSP or CRL server connections.
- **SCT Support**: Accurate revocation checking for certificates that require SCT context.
- **SPKI Hashing**: Uses SHA256 Subject Public Key Info (SPKI) hashes for issuer identification.
- **JSON Output**: Easy integration with other tools and pipelines.

## Project Structure

```text
.
├── Cargo.toml          # Rust dependencies (clubcard-crlite, clap, serde)
├── src/
│   └── main.rs         # High-performance Rust query engine
└── scripts/
    └── get_full_params.py  # Python extractor for SPKI hashes and SCTs
```

## Installation

### Prerequisites

- **Rust & Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)
- **Python 3**: For data extraction scripts.

### Build

1. Clone the repository:
   ```bash
   git clone git@github.com:alebedev80/fast-crlite-query.git
   cd fast-crlite-query
   ```

2. Build the Rust binary:
   ```bash
   cargo build --release
   ```

3. Set up Python environment:
   ```bash
   python3 -m venv venv
   source venv/bin/activate
   pip install cryptography
   ```

## Usage

### 1. Extract Certificate Parameters
To check a certificate, you need the SHA256 hash of the issuer's SPKI and the certificate's serial number (plus SCTs for precision).

```bash
source venv/bin/activate
python3 scripts/get_full_params.py certificate.pem issuer.pem > input.txt
```

### 2. Run Query
The tool reads from `stdin` and requires a directory containing Mozilla's `.filter` and `.delta` files.

```bash
cat input.txt | ./target/release/fast-crlite-query --db-dir /path/to/crlite_db --json
```

## Database Updates

CRLite filters are updated by Mozilla several times a day. To keep your local database current, you can use the official Mozilla [rust-query-crlite](https://github.com/mozilla/rust-query-crlite) utility.

A helper script is provided in `scripts/update_db.sh`. It can be scheduled via `cron`:

1.  Point the script to your `rust-query-crlite` binary and target DB directory.
2.  Add a cron job (example in `etc/cron.d/fast-crlite-query`):
    ```bash
    0 */4 * * * root /path/to/fast-crlite-query/scripts/update_db.sh /path/to/rust-query-crlite /var/lib/crlite_db
    ```

## Performance Note
CRLite filters are updated by Mozilla several times a day. For maximum accuracy, ensure your local database is synchronized with the latest filters from Mozilla's servers.

## License
MIT OR Apache-2.0
