# pg_base62

A PostgreSQL extension for encoding UUIDs to Base62 alphabet and decoding Base62
back to UUID. It is built in Rust with
[pgrx](https://github.com/pgcentralfoundation/pgrx).

## Overview

This extension provides functions to convert PostgreSQL UUIDs to and from Base62
encoded strings. Base62 encoding uses alphanumeric characters (0-9, A-Z, a-z) to
create shorter, URL-safe representations of UUIDs.

It provides two functions:

- `base62_encode(uuid) → text`
- `base62_decode(text) → uuid`

## Usage

### Functions

#### `base62_encode(uuid) → text`

Encodes a UUID into a Base62 string representation.

**Parameters:**

- `uuid`: A PostgreSQL UUID value

**Returns:**

- `text`: The Base62 encoded representation as a URL-safe string

**Example:**

```sql
SELECT base62_encode('f81d4fae-7dec-11d0-a765-00a0c91e6bf6'::uuid);
```

#### `base62_decode(text) → uuid`

Decodes a Base62 encoded string back to a UUID.

**Parameters:**

- `text`: Base62 encoded representation created by `base62_encode`

**Returns:**

- `uuid`: The decoded UUID value

## Installation

### Prerequisites

- PostgreSQL 18 or later (Note: theoretically this can be everything from
  PostgreSQL 14 upwards. I just didn't test this)
- Rust toolchain
- pgrx development tools

### Build and Install

```bash
# Install pgrx if not already installed
cargo install --locked cargo-pgrx

# Initialize pgrx for your PostgreSQL version
cargo pgrx init

# Build and install the extension
cargo pgrx install
```

### Enable the Extension

```sql
create extension pg_base62;
```

Use it like this:

```sql
select base62_encode('0199a3e9-85b2-764a-8ff0-a1fcd5f9a3b2'::uuid);
     base62_encode
───────────────────────
 31CmN3LMJd6n2qPHOlsuY
(1 row)

select base62_decode('31CmN3LMJd6n2qPHOlsuY');
            base62_decode
──────────────────────────────────────
 0199a3e9-85b2-764a-8ff0-a1fcd5f9a3b2
(1 row)
```

## Development

### Running Tests

```bash
# Run Rust unit tests
cargo test

# Run PostgreSQL integration tests
cargo pgrx test
```

### Project Structure

- `src/lib.rs` - Main extension code with encoding functions
- `src/error.rs` - Custom error types
- `src/bin/pgrx_embed.rs` - pgrx embedding binary
- `pg_base62.control` - PostgreSQL extension control file

## Technical Details

- **Base62 Character Set**:
  `0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz`
- **UUID Representation**: UUIDs are converted to 128-bit integers before
  encoding
- **Output Length**: Base62 encoded UUIDs are up to 22 characters long (to be
  precise: $\lceil \log_{62}(2^{128}) \rceil$ → 22 chars …most of the time)

## License

MIT License - see the repository for full license text.

## Links

- [Base62 Wikipedia](https://en.wikipedia.org/wiki/Base62)
- [pgrx Framework](https://github.com/pgcentralfoundation/pgrx)
- [PostgreSQL Extensions](https://www.postgresql.org/docs/18/extend.html)
