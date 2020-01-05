use std::marker::PhantomData;

mod decoders;

pub use decoders::{
    boolean, field, float, integer, list, map1, map10, map11, map12, map2, map3, map4, map5, map6,
    map7, map8, map9, string,
};

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
    InvalidInteger(String),
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
        let decoder = map1(TestStruct::new, field("field_one", string()));

        let json = serde_json::from_str(r#"{"field_one": "test"}"#).unwrap();

        assert_eq!(
            decoder.decode(&json),
            Ok(TestStruct {
                field_one: "test".to_string()
            })
        )
    }

    #[derive(Debug, PartialEq)]
    struct Test4Struct {
        field_one: String,
        field_two: i64,
        field_three: bool,
        field_four: f64,
    }

    impl Test4Struct {
        fn new(field_one: String, field_two: i64, field_three: bool, field_four: f64) -> Self {
            Test4Struct {
                field_one,
                field_two,
                field_three,
                field_four,
            }
        }
    }

    // TODO: HashMaps, Arrays etc.
    // TODO: failure cases.

    #[test]
    fn one_of_the_macro_map_fns() {
        let decoder = map4(
            Test4Struct::new,
            field("field_one", string()),
            field("field_two", integer()),
            field("field_three", boolean()),
            field("field_four", float()),
        );

        let json = serde_json::json!({"field_one": "test", "field_two": 10000, "field_three": true, "field_four": 1.0});

        assert_eq!(
            decoder.decode(&json),
            Ok(Test4Struct {
                field_one: "test".to_string(),
                field_two: 10000,
                field_three: true,
                field_four: 1.0
            })
        )
    }

    #[test]
    fn decoding_a_list() {
        let decoder = list::<_, Vec<_>, _>(string());

        let json = serde_json::json!(["one", "two", "three", "four"]);

        assert_eq!(
            decoder.decode(&json),
            Ok(vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string()
            ])
        )
    }
}
