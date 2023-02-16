use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

impl From<bool> for Json {
    fn from(value: bool) -> Self {
        Json::Boolean(value)
    }
}

impl From<&str> for Json {
    fn from(value: &str) -> Self {
        Json::String(value.into())
    }
}

impl From<f64> for Json {
    fn from(value: f64) -> Self {
        Json::Number(value)
    }
}

impl From<f32> for Json {
    fn from(value: f32) -> Self {
        Json::Number(value.into())
    }
}

// impl From<Vec<&str>> for Json {
//     fn from(values: Vec<&str>) -> Self {

//         Json::Array(
//             values
//                 .into_iter()
//                 .map(|value| Json::String(value.into()))
//                 .collect(),
//         )
//     }
// }

// impl From<u32> for Json {
//     fn from(value: u32) -> Self {
//         Json::Number(value.into())
//     }
// }

// impl From<isize> for Json {
//     fn from(value: isize) -> Self {
//         Json::Number(value.into())
//     }
// }

macro_rules! json {
    (null) => { Json::Null };
    // (true) => { Json::Boolean(true)};
    // (false) => { Json::Boolean(false)};
    ([ $( $value:tt ),* ]) => {
        {
            let array = vec![ $(json!($value) ),*];
            Json::Array(array)
        }
        // let mut arr: Vec<&str> = Vec::new();
        // $(
        //     arr.push($value);  
        // )+
        // Json::from(arr)
    };
    // ({ ??? }) => {
    //     Json::Object(
    //         ???
    //     )
    // };
    ($value:expr) => {
        Json::from($value)
    };


    // ($x:expr) => {
    //     if let s = Json::String($x) { s }
    //     else if let n = Json::Number($x) { n }
    //     else if let b = Json::Boolean($x) { b }
    //     else {Json::Null}
    //     // Json::String($x)
    //     // ??? // Number, String, Boolean
    // };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_null_value() {
        assert_eq!(json!(null), Json::Null);
    }

    #[test]
    fn parse_a_valid_boolean() {
        assert_eq!(json!(true), Json::Boolean(true));
    }
    #[test]
    fn parse_a_valid_boolean_false() {
        assert_eq!(json!(false), Json::Boolean(false));
    }

    #[test]
    fn parse_a_valid_string() {
        assert_eq!(json!("Hello"), Json::String(String::from("Hello")));
    }

    #[test]
    fn parse_a_valid_array() {
        assert_eq!(
            json!(["a", "b", "c"]),
            Json::Array(vec![
                Json::String(String::from("a")),
                Json::String(String::from("b")),
                Json::String(String::from("c")),
            ])
        );
    }

    #[test]
    fn parse_a_valid_array_of_arrays() {
        assert_eq!(
            json!([["a", "b"], ["c"]]),
            Json::Array(vec![
                Json::Array(vec![
                    Json::String(String::from("a")),
                    Json::String(String::from("b"))
                ]),
                Json::Array(vec![Json::String(String::from("c"))]),
            ])
        );
    }

    // #[test]
    // fn parse_a_valid_object() {
    //     assert_eq!(json!({
    //         "Hello": "world",
    //         "Test": 1,
    //         "Names": [
    //             "John",
    //             "Doe"
    //         ]
    //     }), Json::Object(
    //         vec![
    //             (String::from("Hello"), Json::String(String::from("world"))),
    //             (String::from("Test"), Json::Number(1.0)),
    //             (String::from("Names"), Json::Array(vec![
    //                 Json::String(String::from("John")),
    //                 Json::String(String::from("Doe")),
    //             ])),
    //         ].into_iter().collect()
    //     ));
    // }

    // // #[test]
    // fn parse_a_valid_number() {
    //     assert_eq!(json!(1), Json::Number(1.0));
    //     assert_eq!(json!(1u8), Json::Number(1.0));
    //     assert_eq!(json!(1u16), Json::Number(1.0));
    //     assert_eq!(json!(1u32), Json::Number(1.0));
    //     assert_eq!(json!(1u64), Json::Number(1.0));
    //     assert_eq!(json!(1u128), Json::Number(1.0));
    //     assert_eq!(json!(1usize), Json::Number(1.0));
    //     assert_eq!(json!(1i8), Json::Number(1.0));
    //     assert_eq!(json!(1i16), Json::Number(1.0));
    //     assert_eq!(json!(1i32), Json::Number(1.0));
    //     assert_eq!(json!(1i64), Json::Number(1.0));
    //     assert_eq!(json!(1i128), Json::Number(1.0));
    //     assert_eq!(json!(1isize), Json::Number(1.0));
    // }

    // // #[test]
    // fn parse_a_valid_float() {
    //     assert_eq!(json!(1.0f32), Json::Number(1.0));
    //     assert_eq!(json!(1.0f64), Json::Number(1.0));
    // }
}
