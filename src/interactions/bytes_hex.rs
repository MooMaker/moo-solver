//! Serialization of Vec<u8> to 0x prefixed hex string

use {
    serde::{de::Error, Deserialize, Deserializer, Serializer},
    serde_with::{DeserializeAs, SerializeAs},
    std::borrow::Cow,
};

pub fn serialize<S, T>(bytes: T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    let mut v = vec![0u8; 2 + bytes.as_ref().len() * 2];
    v[0] = b'0';
    v[1] = b'x';
    // Unwrap because only possible error is vector wrong size which cannot happen.
    hex::encode_to_slice(bytes, &mut v[2..]).unwrap();
    // Unwrap because encoded data is always valid utf8.
    serializer.serialize_str(&String::from_utf8(v).unwrap())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let prefixed_hex_str = Cow::<str>::deserialize(deserializer)?;
    let hex_str = prefixed_hex_str
        .strip_prefix("0x")
        .ok_or_else(|| D::Error::custom("missing '0x' prefix"))?;
    hex::decode(hex_str).map_err(D::Error::custom)
}

pub struct BytesHex(());

impl<T> SerializeAs<T> for BytesHex
where
    T: AsRef<[u8]>,
{
    fn serialize_as<S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize(bytes, serializer)
    }
}

impl<'de> DeserializeAs<'de, Vec<u8>> for BytesHex {
    fn deserialize_as<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize(deserializer)
    }
}
