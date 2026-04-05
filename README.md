# fast-crlite-query

High-performance CLI utility for mass certificate revocation checking using Mozilla's CRLite (Clubcard/Ribbon) filters.

## Overview

`fast-crlite-query` (also known as `crlite-mass-check`) provides a privacy-preserving and ultra-fast way to check SSL/TLS certificate revocation status locally. Unlike OCSP, it does not leak query data to third parties and can process millions of certificates per second.

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

### 2. Run Mass Check
The tool reads from `stdin` and requires a directory containing Mozilla's `.filter` and `.delta` files.

```bash
cat input.txt | ./target/release/fast-crlite-query --db-dir /path/to/crlite_db --json
```

**Input Format:**
`<ISSUER_SPKI_SHA256_HEX> <SERIAL_HEX> <LOG_ID_HEX:TIMESTAMP_MS> ...`

### 3. Understanding Statuses
- `Revoked`: Certificate found in the filter/delta (definitely revoked).
- `NotRevoked`: Certificate not found (considered valid within the current DB).

## Performance Note
CRLite filters are updated by Mozilla several times a day. For maximum accuracy, ensure your local database is synchronized with the latest filters from Mozilla's servers.

## License
MIT OR Apache-2.0
