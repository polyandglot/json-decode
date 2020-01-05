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

pub fn integer<'a, I: From<i64>>() -> impl Decoder<'a, I> {
    IntDecoder {
        phantom: PhantomData,
    }
}

pub struct IntDecoder<I: From<i64>> {
    phantom: PhantomData<I>,
}

impl<'a, I> Decoder<'a, I> for IntDecoder<I>
where
    I: From<i64>,
{
    fn decode(&self, value: &serde_json::Value) -> Result<I, DecodeError> {
        match value {
            serde_json::Value::Number(n) => n
                .as_i64()
                .map(Into::into)
                .ok_or(DecodeError::InvalidInteger(value.to_string())),
            _ => Err(DecodeError::IncorrectType(
                "Number".to_string(),
                value.to_string(),
            )),
        }
    }
}

pub fn unsigned_integer<'a, I: From<u64>>() -> impl Decoder<'a, I> {
    UIntDecoder {
        phantom: PhantomData,
    }
}

pub struct UIntDecoder<I: From<u64>> {
    phantom: PhantomData<I>,
}

impl<'a, I> Decoder<'a, I> for UIntDecoder<I>
where
    I: From<u64>,
{
    fn decode(&self, value: &serde_json::Value) -> Result<I, DecodeError> {
        match value {
            serde_json::Value::Number(n) => n
                .as_u64()
                .map(Into::into)
                .ok_or(DecodeError::InvalidInteger(value.to_string())),
            _ => Err(DecodeError::IncorrectType(
                "Number".to_string(),
                value.to_string(),
            )),
        }
    }
}

pub fn float<'a, I: From<f64>>() -> impl Decoder<'a, I> {
    FloatDecoder {
        phantom: PhantomData,
    }
}

pub struct FloatDecoder<I: From<f64>> {
    phantom: PhantomData<I>,
}

impl<'a, I> Decoder<'a, I> for FloatDecoder<I>
where
    I: From<f64>,
{
    fn decode(&self, value: &serde_json::Value) -> Result<I, DecodeError> {
        match value {
            serde_json::Value::Number(n) => n
                .as_f64()
                .map(Into::into)
                .ok_or(DecodeError::InvalidInteger(value.to_string())),
            _ => Err(DecodeError::IncorrectType(
                "Number".to_string(),
                value.to_string(),
            )),
        }
    }
}

pub fn boolean<'a>() -> impl Decoder<'a, bool> {
    BooleanDecoder {}
}

pub struct BooleanDecoder {}

impl<'a> Decoder<'a, bool> for BooleanDecoder {
    fn decode(&self, value: &serde_json::Value) -> Result<bool, DecodeError> {
        match value {
            serde_json::Value::Bool(b) => Ok(*b),
            _ => Err(DecodeError::IncorrectType(
                "Boolean".to_string(),
                value.to_string(),
            )),
        }
    }
}

macro_rules! define_map_decoder {
    ($fn_name:ident, $struct_name:ident, $($i:ident),+) => {
        pub fn $fn_name<'a, F, $($i,)+ NewDecodesTo>(
            func: F,
            $($i: impl Decoder<'a, $i> +'static,)+
        ) -> impl Decoder<'a, NewDecodesTo>
        where F: Fn($($i, )+) -> NewDecodesTo + 'a
        {
            $struct_name {
                func: Box::new(func),
                decoders: (($(Box::new($i), )+))
            }
        }

        struct $struct_name<'a, DecodesTo, $($i,)+> {
            func: Box<dyn Fn($($i,)+) -> DecodesTo + 'a>,
            decoders: ($(Box<dyn Decoder<'a, $i>>,)+ )
        }

        impl<'a, DecodesTo, $($i,)+> Decoder<'a, DecodesTo>
        for $struct_name<'a, DecodesTo, $($i,)+> {
            fn decode(&self, value: &serde_json::Value) -> Result<DecodesTo, DecodeError> {
                let ($($i, )+) = &self.decoders;
                $(
                    let $i = (*$i).decode(value)?;
                )+
                let result = (*self.func)($($i, )+);
                Ok(result)
            }
        }
    }
}

define_map_decoder!(map2, Fn2Decoder, _1, _2);
define_map_decoder!(map3, Fn3Decoder, _1, _2, _3);
define_map_decoder!(map4, Fn4Decoder, _1, _2, _3, _4);
define_map_decoder!(map5, Fn5Decoder, _1, _2, _3, _4, _5);
define_map_decoder!(map6, Fn6Decoder, _1, _2, _3, _4, _5, _6);
define_map_decoder!(map7, Fn7Decoder, _1, _2, _3, _4, _5, _6, _7);
define_map_decoder!(map8, Fn8Decoder, _1, _2, _3, _4, _5, _6, _7, _8);
define_map_decoder!(map9, Fn9Decoder, _1, _2, _3, _4, _5, _6, _7, _8, _9);
define_map_decoder!(map10, Fn10Decoder, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10);
define_map_decoder!(
    map11,
    Fn11Decoder,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11
);
define_map_decoder!(
    map12,
    Fn12Decoder,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11,
    _12
);
define_map_decoder!(
    map13,
    Fn13Decoder,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11,
    _12,
    _13
);

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
