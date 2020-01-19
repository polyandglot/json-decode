use super::{DecodeError, Decoder};
use std::iter::FromIterator;
use std::marker::PhantomData;

pub type BoxDecoder<'a, T> = Box<dyn Decoder<'a, T> + 'a + Send + Sync>;

pub fn field<'a, T>(field_name: &str, decoder: BoxDecoder<'a, T>) -> BoxDecoder<'a, T>
where
    T: 'a,
{
    Box::new(FieldDecoder {
        field_name: field_name.to_string(),
        inner_decoder: decoder,
    })
}

pub struct FieldDecoder<'a, DecodesTo> {
    field_name: String,
    inner_decoder: BoxDecoder<'a, DecodesTo>,
}

impl<'a, DecodesTo> Decoder<'a, DecodesTo> for FieldDecoder<'a, DecodesTo> {
    fn decode(&self, value: &serde_json::Value) -> Result<DecodesTo, DecodeError> {
        match value {
            serde_json::Value::Object(map) => map
                .get(&self.field_name)
                .ok_or(DecodeError::MissingField(
                    self.field_name.clone(),
                    value.to_string(),
                ))
                .and_then(|inner_value| (*self.inner_decoder).decode(inner_value)),
            _ => Err(DecodeError::IncorrectType(
                "Object".to_string(),
                value.to_string(),
            )),
        }
    }
}

pub fn string() -> BoxDecoder<'static, String> {
    Box::new(StringDecoder {})
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

pub fn integer<I: From<i64>>() -> BoxDecoder<'static, I>
where
    I: 'static + Send + Sync,
{
    Box::new(IntDecoder {
        phantom: PhantomData,
    })
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

pub fn unsigned_integer<'a, I: From<u64>>() -> Box<dyn Decoder<'a, I> + 'a>
where
    I: 'a,
{
    Box::new(UIntDecoder {
        phantom: PhantomData,
    })
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

pub fn float<F: From<f64>>() -> BoxDecoder<'static, F>
where
    F: 'static + Send + Sync,
{
    Box::new(FloatDecoder {
        phantom: PhantomData,
    })
}

pub struct FloatDecoder<I: From<f64>> {
    phantom: PhantomData<I>,
}

// TODO: Probably don't need from - just force f64 etc.
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

pub fn boolean() -> BoxDecoder<'static, bool> {
    Box::new(BooleanDecoder {})
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

pub fn option<'a, DecodesTo>(
    decoder: BoxDecoder<'a, DecodesTo>,
) -> BoxDecoder<'a, Option<DecodesTo>>
where
    DecodesTo: 'a + Send + Sync,
{
    Box::new(OptionDecoder {
        inner_decoder: decoder,
    })
}

pub struct OptionDecoder<'a, DecodesTo> {
    inner_decoder: BoxDecoder<'a, DecodesTo>,
}

impl<'a, DecodesTo> Decoder<'a, Option<DecodesTo>> for OptionDecoder<'a, DecodesTo>
where
    DecodesTo: 'a,
{
    fn decode(&self, value: &serde_json::Value) -> Result<Option<DecodesTo>, DecodeError> {
        match value {
            serde_json::Value::Null => Ok(None),
            other => self.inner_decoder.decode(value).map(Some),
        }
    }
}

// TODO: Difficulties using this due to type inference problems
// look to re-work the interface somehow
pub fn list<'a, Item, Collection>(decoder: BoxDecoder<'a, Item>) -> BoxDecoder<'a, Collection>
where
    Collection: FromIterator<Item> + 'a + Send + Sync,
    Item: 'a,
{
    Box::new(ListDecoder {
        inner_decoder: decoder,
        phantom: PhantomData,
    })
}

pub struct ListDecoder<'a, Item, DecodesTo: FromIterator<Item>> {
    phantom: PhantomData<DecodesTo>,
    inner_decoder: BoxDecoder<'a, Item>,
}

impl<'a, Item, DecodesTo> Decoder<'a, DecodesTo> for ListDecoder<'a, Item, DecodesTo>
where
    DecodesTo: FromIterator<Item>,
{
    fn decode(&self, value: &serde_json::Value) -> Result<DecodesTo, DecodeError> {
        match value {
            serde_json::Value::Array(vec) => vec
                .iter()
                .map(|item| (*self.inner_decoder).decode(item))
                .collect(),
            _ => Err(DecodeError::IncorrectType(
                "Array".to_string(),
                value.to_string(),
            )),
        }
    }
}

// TODO: Do we need the lifetimes here
pub fn map<'a, F, T1, NewDecodesTo>(func: F, d1: BoxDecoder<'a, T1>) -> BoxDecoder<'a, NewDecodesTo>
where
    F: (Fn(T1) -> NewDecodesTo) + 'a + Send + Sync,
    NewDecodesTo: 'a,
    T1: 'a,
{
    Box::new(DecoderFn1 {
        func: Box::new(func),
        decoder: d1,
    })
}

pub struct DecoderFn1<'a, DecodesTo, Argument1> {
    func: Box<dyn Fn(Argument1) -> DecodesTo + 'a + Send + Sync>,
    decoder: BoxDecoder<'a, Argument1>,
}

impl<'a, DecodesTo, Argument1> Decoder<'a, DecodesTo> for DecoderFn1<'a, DecodesTo, Argument1> {
    fn decode(&self, value: &serde_json::Value) -> Result<DecodesTo, DecodeError> {
        let arg0 = self.decoder.decode(value)?;
        Ok((*self.func)(arg0))
    }
}

macro_rules! define_map_decoder {
    ($fn_name:ident, $struct_name:ident, $($i:ident),+) => {
        pub fn $fn_name<'a, F, $($i,)+ NewDecodesTo>(
            func: F,
            $($i: BoxDecoder<'a, $i>,)+
        ) -> BoxDecoder<'a, NewDecodesTo>
        where F: Fn($($i, )+) -> NewDecodesTo + 'a + Send + Sync,
            NewDecodesTo: 'a,
            $($i: 'a,)+
        {
            Box::new($struct_name {
                func: Box::new(func),
                decoders: (($($i, )+))
            })
        }

        struct $struct_name<'a, DecodesTo, $($i,)+> {
            func: Box<dyn Fn($($i,)+) -> DecodesTo + 'a + Send + Sync>,
            decoders: ($(BoxDecoder<'a, $i>,)+ )
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
define_map_decoder!(
    map14,
    Fn14Decoder,
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
    _13,
    _14
);
define_map_decoder!(
    map15,
    Fn15Decoder,
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
    _13,
    _14,
    _15
);
define_map_decoder!(
    map16,
    Fn16Decoder,
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
    _13,
    _14,
    _15,
    _16
);
define_map_decoder!(
    map17,
    Fn17Decoder,
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
    _13,
    _14,
    _15,
    _16,
    _17
);
define_map_decoder!(
    map18,
    Fn18Decoder,
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
    _13,
    _14,
    _15,
    _16,
    _17,
    _18
);
define_map_decoder!(
    map19,
    Fn19Decoder,
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
    _13,
    _14,
    _15,
    _16,
    _17,
    _18,
    _19
);
define_map_decoder!(
    map20,
    Fn20Decoder,
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
    _13,
    _14,
    _15,
    _16,
    _17,
    _18,
    _19,
    _20
);

pub fn serde<T>() -> BoxDecoder<'static, T>
where
    for<'de> T: serde::Deserialize<'de> + 'static + Send + Sync,
{
    Box::new(SerdeDecoder {
        phantom: PhantomData,
    })
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

pub fn json() -> BoxDecoder<'static, serde_json::Value> {
    Box::new(JsonDecoder {})
}

pub struct JsonDecoder {}

impl<'a> Decoder<'a, serde_json::Value> for JsonDecoder {
    fn decode(&self, value: &serde_json::Value) -> Result<serde_json::Value, DecodeError> {
        // TODO: Figure out if we can get rid of this clone somehow?
        Ok(value.clone())
    }
}
