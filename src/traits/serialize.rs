use crate::error::Result;

pub trait MySerialize {
    fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> Result<()>;
}

pub trait MyDeserialize: Sized {
    fn try_deserialize(buf: &[u8]) -> Result<Self>;
}

// Prioritize bincode if both features are enabled, otherwise use json
#[cfg(all(feature = "bincode", not(feature = "json")))]
mod bincode_only_impls {
    use super::*;
    use crate::error::ErrorCode;
    use serde::{Serialize, de::DeserializeOwned};
    use bincode::config::legacy;
    use bincode::serde::{encode_into_std_write, decode_from_slice};

    impl<T> MySerialize for T
    where
        T: Serialize,
    {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
            encode_into_std_write(self, writer, legacy())
                .map_err(|_| ErrorCode::InvalidData)
                .map(|_| ())
        }
    }

    impl<T> MyDeserialize for T
    where
        T: DeserializeOwned,
    {
        fn try_deserialize(buf: &[u8]) -> Result<Self> {
            let (result, _) = decode_from_slice(buf, legacy()).map_err(|_| ErrorCode::InvalidData)?;
            Ok(result)
        }
    }
}

#[cfg(all(feature = "json", not(feature = "bincode")))]
mod json_only_impls {
    use super::*;
    use crate::error::ErrorCode;
    use serde::{Serialize, de::DeserializeOwned};

    impl<T> MySerialize for T
    where
        T: Serialize,
    {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
            serde_json::to_writer(writer, self).map_err(|_| ErrorCode::InvalidData)
        }
    }

    impl<T> MyDeserialize for T
    where
        T: DeserializeOwned,
    {
        fn try_deserialize(buf: &[u8]) -> Result<Self> {
            serde_json::from_slice(buf).map_err(|_| ErrorCode::InvalidData)
        }
    }
}

// When both features are enabled, prefer bincode
#[cfg(all(feature = "bincode", feature = "json"))]
mod both_features_impls {
    use super::*;
    use crate::error::ErrorCode;
    use serde::{Serialize, de::DeserializeOwned};
    use bincode::config::legacy;
    use bincode::serde::{encode_into_std_write, decode_from_slice};

    impl<T> MySerialize for T
    where
        T: Serialize,
    {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> Result<()> {
            encode_into_std_write(self, writer, legacy())
                .map_err(|_| ErrorCode::InvalidData)
                .map(|_| ())
        }
    }

    impl<T> MyDeserialize for T
    where
        T: DeserializeOwned,
    {
        fn try_deserialize(buf: &[u8]) -> Result<Self> {
            let (result, _) = decode_from_slice(buf, legacy()).map_err(|_| ErrorCode::InvalidData)?;
            Ok(result)
        }
    }
}
