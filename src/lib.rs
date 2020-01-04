use std::marker::PhantomData;

mod decoders;

pub use decoders::{field, map1, string};

pub trait Decoder<'a, DecodesTo> {
    // OK, so theoretically this needs to store some functions & some collection of arguments.
    // Since functions need to be of differing lengths we probably need a trait rather than a struct
    // with different implementations for lenghts of arguments.
    //
    // Structs could probably be generic over the types of the arguments?
    //
    // Or alternatively all functions have to take a JSON.Value enum and do the decoding based on that.
    fn decode(&self, value: &serde_json::Value) -> Result<DecodesTo, DecodeError>;
}

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    MissingField(String),
    IncorrectType(String, String),
    SerdeError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: test out DSL

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        field_one: String,
    }

    impl TestStruct {
        fn new(field_one: String) -> Self {
            TestStruct {
                field_one: field_one,
            }
        }
    }

    #[test]
    fn decode_a_struct() {
        let selection_set = map1(TestStruct::new, field("field_one", string()));

        let json = serde_json::from_str(r#"{"field_one": "test"}"#).unwrap();

        assert_eq!(
            selection_set.decode(&json),
            Ok(TestStruct {
                field_one: "test".to_string()
            })
        )
    }
}
