use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zerocopy::AsBytes;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(Debug, AsBytes, Clone, Copy, FromBytes, FromZeroes)]
#[repr(transparent)]
pub struct CharArray<const N: usize>([u8; N]);

#[derive(Debug, AsBytes, Clone, Copy, FromBytes, FromZeroes)]
#[repr(transparent)]
pub struct Bool(u8);

impl<'de, const N: usize> Deserialize<'de> for CharArray<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize the incoming data as a `String`
        let s: String = String::deserialize(deserializer)?;

        let mut chars = [0; N];

        // Copy characters from the string into the fixed-size array
        for (i, c) in s.chars().take(N).enumerate() {
            chars[i] = c as u8;
        }

        // Return the populated char array
        Ok(CharArray(chars))
    }
}

impl<'de> Deserialize<'de> for Bool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let b = bool::deserialize(deserializer)?;

        Ok(Bool(b.as_bytes()[0]))
    }
}

impl Serialize for Bool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            0 => serializer.serialize_bool(false),
            _ => serializer.serialize_bool(true),
        }
    }
}

impl<const N: usize> Serialize for CharArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Collect non-null characters into a string, ignoring `'\0'`
        let str_bytes = self
            .0
            .iter()
            .take_while(|&c| *c != 0)
            .map(|&c| c)
            .collect::<Vec<u8>>();

        let string = String::from_utf8_lossy(&str_bytes);
        // Serialize the string
        serializer.serialize_str(&string)
    }
}
