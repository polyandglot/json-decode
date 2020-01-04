use super::{DecodeError, Decoder};
use std::marker::PhantomData;

pub fn field<'a, T>(
    field_name: &str,
    decoder: impl Decoder<'a, T> + 'static,
) -> impl Decoder<'a, T> {
    FieldDecoder {
        field_name: field_name.to_string(),
        inner_decoder: Box::new(decoder),
    }
}

pub struct FieldDecoder<'a, DecodesTo> {
    field_name: String,
    inner_decoder: Box<dyn Decoder<'a, DecodesTo>>,
}

impl<'a, DecodesTo> Decoder<'a, DecodesTo> for FieldDecoder<'a, DecodesTo> {
    fn decode(&self, value: &serde_json::Value) -> Result<DecodesTo, DecodeError> {
        match value {
            serde_json::Value::Object(map) => map
                .get(&self.field_name)
                .ok_or(DecodeError::MissingField(self.field_name.clone()))
                .and_then(|value| (*self.inner_decoder).decode(value)),
            _ => Err(DecodeError::IncorrectType(
                "Object".to_string(),
                value.to_string(),
            )),
        }
    }
}

pub fn string<'a>() -> impl Decoder<'a, String> {
    StringDecoder {}
}

pub struct StringDecoder {}

impl<'a> Decoder<'a, String> for StringDecoder {
    fn decode(&self, value: &serde_json::Value) -> Result<String, DecodeError> {
        match value {
            serde_json::Value::String(s) => Ok(s.clone()),
            _ => Err(DecodeError::IncorrectType(
                "String".to_string(),
                value.to_string(),
            )),
        }
    }
}

pub fn map1<'a, F, T1, NewDecodesTo>(
    func: F,
    d1: impl Decoder<'a, T1> + 'static,
) -> impl Decoder<'a, NewDecodesTo>
where
    F: Fn(T1) -> NewDecodesTo + 'a,
{
    DecoderFn1 {
        func: Box::new(func),
        decoder: Box::new(d1),
    }
}

pub struct DecoderFn1<'a, DecodesTo, Argument1> {
    func: Box<dyn Fn(Argument1) -> DecodesTo + 'a>,
    decoder: Box<dyn Decoder<'a, Argument1>>,
}

impl<'a, DecodesTo, Argument1> Decoder<'a, DecodesTo> for DecoderFn1<'a, DecodesTo, Argument1> {
    fn decode(&self, value: &serde_json::Value) -> Result<DecodesTo, DecodeError> {
        let arg0 = (*self.decoder).decode(value)?;
        Ok((*self.func)(arg0))
    }
}

pub struct SerdeDecoder<T> {
    phantom: PhantomData<T>,
}

impl<T> SerdeDecoder<T> {
    fn new() -> SerdeDecoder<T> {
        SerdeDecoder {
            phantom: PhantomData,
        }
    }
}

impl<'a, DecodesTo> Decoder<'a, DecodesTo> for SerdeDecoder<DecodesTo>
where
    for<'de> DecodesTo: serde::Deserialize<'de>,
{
    fn decode(&self, value: &serde_json::Value) -> Result<DecodesTo, DecodeError> {
        // TODO: Figure out if we can get rid of this clone somehow?
        serde_json::from_value(value.clone()).map_err(|e| DecodeError::SerdeError(e.to_string()))
    }
}
