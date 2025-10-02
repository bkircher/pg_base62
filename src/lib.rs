use error::Base62Error;
use pgrx::prelude::*;

::pgrx::pg_module_magic!();

mod error;

#[pg_extern]
fn base62_encode(input: pgrx::Uuid) -> Result<String, Base62Error> {
    let value = if input.len() == 16 {
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(input.as_bytes());
        u128::from_be_bytes(bytes)
    } else {
        return Err(Base62Error::InvalidInput);
    };

    let mut buf = [0u8; 22];
    if let Ok(len) = base62::encode_bytes(value, &mut buf) {
        core::str::from_utf8(&buf[..len])
            .map(|s| s.to_string())
            .map_err(|_| Base62Error::EncodeError)
    } else {
        Err(Base62Error::EncodeError)
    }
}

#[pg_extern]
fn base62_decode(input: &str) -> Result<pgrx::Uuid, Base62Error> {
    let value = base62::decode(input).map_err(|_| Base62Error::DecodeError)?;
    Ok(pgrx::Uuid::from_bytes(value.to_be_bytes()))
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;
    use uuid::Uuid;

    /// Tests encoding a known valid UUID
    #[pg_test]
    fn test_base62_encode_known_uuid() {
        let uuid = Uuid::parse_str("0199a3dc-8bdf-7126-a3cf-cc8b933e852a").unwrap();
        let pg_uuid = pgrx::Uuid::from_bytes(*uuid.as_bytes());

        let result = crate::base62_encode(pg_uuid);
        assert!(result.is_ok(), "Expected encode without error");
        let output = result.unwrap();

        assert!(!output.is_empty(), "Expected output string to be non-empty");
        assert!(
            matches!(output.len(), 21 | 22),
            "Unexpected encoded length: {}",
            output.len()
        );

        let valid_chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        for c in output.chars() {
            assert!(valid_chars.contains(c), "Invalid base62 character: {}", c);
        }
    }

    /// Tests that encoding/decoding round-trip works for a single UUID.
    #[pg_test]
    fn test_base62_encode_decode_roundtrip() {
        let uuid = Uuid::parse_str("0199a3dc-8bee-73f5-b998-c96cfee3d2df").unwrap();
        let pg_uuid = pgrx::Uuid::from_bytes(*uuid.as_bytes());

        let encode_result = crate::base62_encode(pg_uuid);
        assert!(encode_result.is_ok());

        let output = encode_result.unwrap();

        let decode_result = crate::base62_decode(&output);
        assert!(decode_result.is_ok(), "Expected decode without error");

        let decoded_uuid = decode_result.unwrap();
        assert_eq!(
            decoded_uuid.as_bytes(),
            pg_uuid.as_bytes(),
            "Expected UUID to be same"
        );
    }

    /// Tests encoding a nil UUID
    #[pg_test]
    fn test_base62_encode_null_uuid() {
        let uuid = Uuid::nil();
        let pg_uuid = pgrx::Uuid::from_bytes(*uuid.as_bytes());

        let result = crate::base62_encode(pg_uuid);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.is_empty());
    }

    #[pg_test]
    fn test_base62_known_pairs_roundtrip() {
        let cases = [
            (
                "0199a419-e48c-7107-933d-2ae94df8eadc",
                "31CnfRQGzIq5PTVq2bs1E",
            ),
            (
                "0199a419-e482-7e85-838e-40ca7f5ac9d3",
                "31CnfRPIrur4cvcySLF47",
            ),
            (
                "0199a419-e46e-7199-90a4-fe4dc6efc5ad",
                "31CnfRNLgjQ2SJ9rPi8DN",
            ),
        ];

        for (uuid_str, expected_base62) in cases {
            let uuid = Uuid::parse_str(uuid_str).expect("valid UUID literal");
            let pg_uuid = pgrx::Uuid::from_bytes(*uuid.as_bytes());

            let encoded = crate::base62_encode(pg_uuid).expect("encode succeeds");
            assert_eq!(
                encoded, expected_base62,
                "unexpected base62 string for {}",
                uuid_str
            );

            let decoded_from_encoded =
                crate::base62_decode(&encoded).expect("decode roundtrip succeeds");
            assert_eq!(
                decoded_from_encoded.as_bytes(),
                pg_uuid.as_bytes(),
                "unexpected mismatch for {}",
                uuid_str
            );

            let decoded_from_expected =
                crate::base62_decode(expected_base62).expect("decode expected succeeds");
            assert_eq!(
                decoded_from_expected.as_bytes(),
                pg_uuid.as_bytes(),
                "expected string did not decode to original UUID: {}",
                uuid_str
            );
        }
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of the extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
