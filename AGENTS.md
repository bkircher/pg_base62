# AGENTS.md

## Project Overview

`pg_base62` is a PostgreSQL extension built with pgrx that provides Base62
encoding/decoding for UUIDs. The extension converts standard PostgreSQL UUIDs to
compact, URL-safe Base62 strings.

## Build and Test Commands

```bash
# Build and install the extension
cargo pgrx install

# Run Rust unit tests
cargo test

# Run PostgreSQL integration tests (with pgrx test framework)
cargo pgrx test

# Run tests for a specific PostgreSQL version
cargo pgrx test pg18
```

## Architecture

### Core Components

- **`src/lib.rs`**: Main extension module exposing PostgreSQL functions via pgrx
  macros
  - `base62_encode()`: Converts UUID → u128 → Base62 string (up to 22 chars)
  - `base62_decode()`: Converts a Base62 encoded string to a UUID

- **`src/error.rs`**: Custom error types using `thiserror`
  - `InvalidInput`: UUID length validation failure
  - `EncodeError`: Base62 encoding failure
  - `DecodeError`: Base62 decoding failure

- **`src/bin/pgrx_embed.rs`**: pgrx embedding binary required for extension
  packaging

### Key Dependencies

- `pgrx`: PostgreSQL extension framework (default: pg18)
- `base62`: Base62 encoding implementation
- `uuid`: UUID handling with std and v7 features
- `thiserror`: Error type derivation

### pgrx Integration

Functions exposed to PostgreSQL use the `#[pg_extern]` macro. Tests use
`#[pg_test]` to run inside a PostgreSQL instance. The `pg_test` module provides
setup hooks for test configuration.

### Encoding Implementation

UUIDs are treated as 128-bit big-endian integers. The `base62::encode_bytes()`
function encodes into a fixed 22-byte buffer, returning the actual encoded
length. Invalid inputs or encoding failures return `Base62Error` variants.

## PostgreSQL Version Support

Default target is PostgreSQL 18 (feature flag `pg18`). To support other
versions, modify features in `Cargo.toml` and use pgrx's version-specific
feature flags. You probably also need to add missing uuidv7 function via
extensions.

## Extension Installation

After building with `cargo pgrx install`, enable in PostgreSQL with:

```sql
create extension pg_base62;
select base62_encode('f81d4fae-7dec-11d0-a765-00a0c91e6bf6'::uuid);
```
