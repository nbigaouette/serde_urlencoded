//! Deserialization support for the `application/x-www-form-urlencoded` format.

use serde::de;
use serde::de::value::MapDeserializer;
use std::borrow::Cow;
use url::form_urlencoded::Parse as UrlEncodedParse;
use url::form_urlencoded::parse;

pub use serde::de::value::Error;

/// Deserializes a `application/x-wwww-url-encoded` value from a `&[u8]`.
///
/// ```
/// let meal = vec![
///     ("bread".to_owned(), "baguette".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_urlencoded::from_bytes::<Vec<(String, String)>>(
///         b"bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter"),
///     Ok(meal));
/// ```
pub fn from_bytes<T: de::Deserialize>(input: &[u8]) -> Result<T, Error> {
    T::deserialize(&mut Deserializer::new(parse(input)))
}

/// Deserializes a `application/x-wwww-url-encoded` value from a `&str`.
///
/// ```
/// let meal = vec![
///     ("bread".to_owned(), "baguette".to_owned()),
///     ("cheese".to_owned(), "comté".to_owned()),
///     ("meat".to_owned(), "ham".to_owned()),
///     ("fat".to_owned(), "butter".to_owned()),
/// ];
///
/// assert_eq!(
///     serde_urlencoded::from_str::<Vec<(String, String)>>(
///         "bread=baguette&cheese=comt%C3%A9&meat=ham&fat=butter"),
///     Ok(meal));
/// ```
pub fn from_str<T: de::Deserialize>(input: &str) -> Result<T, Error> {
    from_bytes(input.as_bytes())
}

/// A deserializer for the `application/x-www-form-urlencoded` format.
///
/// * Supported top-level outputs are structs, maps and sequences of pairs,
///   with or without a given length.
///
/// * Main `deserialize` methods defers to `deserialize_map`.
///
/// * Everything else but `deserialize_seq` and `deserialize_seq_fixed_size`
///   defers to `deserialize`.
pub struct Deserializer<'a> {
    inner:
        MapDeserializer<UrlEncodedParse<'a>, Cow<'a, str>, Cow<'a, str>, Error>,
}

impl<'a> Deserializer<'a> {
    /// Returns a new `Deserializer`.
    pub fn new(parser: UrlEncodedParse<'a>) -> Self {
        Deserializer { inner: MapDeserializer::unbounded(parser) }
    }
}

impl<'a> de::Deserializer for Deserializer<'a>
{
    type Error = Error;

    fn deserialize<V>(
            &mut self, visitor: V)
            -> Result<V::Value, Self::Error>
        where V: de::Visitor,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(
            &mut self, mut visitor: V)
            -> Result<V::Value, Self::Error>
        where V: de::Visitor,
    {
        visitor.visit_map(&mut self.inner)
    }

    fn deserialize_seq<V>(
            &mut self, mut visitor: V)
            -> Result<V::Value, Self::Error>
        where V: de::Visitor,
    {
        visitor.visit_seq(&mut self.inner)
    }

    fn deserialize_seq_fixed_size<V>(
            &mut self, _len: usize, mut visitor: V)
            -> Result<V::Value, Self::Error>
        where V: de::Visitor
    {
        visitor.visit_seq(&mut self.inner)
    }

    forward_to_deserialize! {
        bool
        usize
        u8
        u16
        u32
        u64
        isize
        i8
        i16
        i32
        i64
        f32
        f64
        char
        str
        string
        unit
        option
        bytes
        unit_struct
        newtype_struct
        tuple_struct
        struct
        struct_field
        tuple
        enum
        ignored_any
    }
}
