wit_bindgen_rust::export!("json_flatten.wit");

struct JsonFlatten;

extern crate jsonpath_lib;
extern crate serde_json;

use crate::json_flatten::FlattenedBigint;
use crate::json_flatten::FlattenedDouble;
use crate::json_flatten::FlattenedJson;
use crate::json_flatten::FlattenedString;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[inline(never)]
#[no_mangle]
#[allow(non_snake_case)]
fn ERROR__MALFORMED_JSON() {
    panic!("ERROR__MALFORMED_JSON");
}

#[inline(never)]
#[no_mangle]
#[allow(non_snake_case)]
fn ERROR__MALFORMED_JSONPATH() {
    panic!("ERROR__MALFORMED_JSONPATH");
}

#[inline(never)]
#[no_mangle]
#[allow(non_snake_case)]
fn ERROR__JSON_VALUE_SERIALIZATION() {
    panic!("ERROR__JSON_VALUE_SERIALIZATION");
}

#[derive(Serialize, Deserialize)]
struct JsonNV {
    name: String,
    value: serde_json::Value,
}

impl PartialEq for FlattenedBigint {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl PartialEq for FlattenedDouble {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl PartialEq for FlattenedString {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}
impl PartialEq for FlattenedJson {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}

// You can only flatten the children of the object the jsonpath references.
// Therefore, only objects or arrays make sense.  Scalars don't have any
// children, and will always return an empty array.  FIXME: They should
// probably return null, but S2 doesn't support nullability currently.
macro_rules! json_flatten_any {
    ($type:tt, $json:ident, $exprs:ident, $from_closure:expr) => {{
        let mut exprs = $exprs;
        if exprs.len() == 0 {
            exprs = vec![String::from("$")];
        }
        let mut res: Vec<$type> = vec![];
        let deser = serde_json::from_str($json.as_str())
            .map_err(|_| ERROR__MALFORMED_JSON())
            .unwrap();
        for expr in exprs {
            let selected = jsonpath_lib::select(&deser, expr.as_str())
                .map_err(|_| ERROR__MALFORMED_JSONPATH())
                .unwrap();
            for item in selected.iter() {
                match item {
                    JsonValue::Array(array)  => {
                        for (index, value) in array.iter().enumerate() {
                            res.push($type {
                                name: index.to_string(),
                                value: $from_closure(value),
                            });
                        }
                    },
                    JsonValue::Object(object) => {
                        for (name, value) in object {
                            res.push($type {
                                name: name.clone(),
                                value: $from_closure(value),
                            });
                        }
                    },
                    _ => (),
                }
            }
        }
        res
    }}

}

macro_rules! json_flatten_nonjson {
    ($type:tt, $json:ident, $exprs:ident) => {{
        json_flatten_any!(
            $type,
            $json,
            $exprs, 
            |value: &JsonValue| {
                serde_json::from_value(value.clone()).unwrap_or_default()
            }
        )
    }}
}

impl json_flatten::JsonFlatten for JsonFlatten {

    fn json_flatten_json(json: String, exprs: Vec<String>) -> Vec<FlattenedJson> {
        json_flatten_any!(
            FlattenedJson,
            json,
            exprs, 
            |value: &JsonValue| value.to_string()
        )
    }

    fn json_flatten_bigint(json: String, exprs: Vec<String>) -> Vec<FlattenedBigint> {
        json_flatten_nonjson!(FlattenedBigint, json, exprs)
    }

    fn json_flatten_double(json: String, exprs: Vec<String>) -> Vec<FlattenedDouble> {
        json_flatten_nonjson!(FlattenedDouble, json, exprs)
    }

    fn json_flatten_string(json: String, exprs: Vec<String>) -> Vec<FlattenedString> {
        json_flatten_nonjson!(FlattenedString, json, exprs)
    }
}

#[cfg(test)]
mod tests {
    use super::json_flatten::JsonFlatten as _;
    use super::*;

    const TEST_INPUT_OBJECT: &str = r#"
        {
            "category": "web",
            "language": "en",
            "title": "XQuery Kick Start",
            "authors": {
                "first": "James McGovern",
                "second": "Per Bothner",
                "third": "Kurt Cagle",
                "fourth": "James Linn",
                "fifth": "Vaidyanathon Nagarajan"
            },
            "year": 2003,
            "price": 49.99
        }
    "#;

    const TEST_INPUT_ARRAY: &str = r#"
        [
            {
                "category": "web",
                "language": "en",
                "title": "XQuery Kick Start",
                "authors": {
                    "first": "James McGovern",
                    "second": "Per Bothner",
                    "third": "Kurt Cagle",
                    "fourth": "James Linn",
                    "fifth": "Vaidyanathon Nagarajan"
                },
                "year": 2003,
                "price": 49.99
            },
            {
                "category": "programming",
                "language": "en",
                "title": "The C Programming Language",
                "authors": {
                    "first": "Brian Kernighan",
                    "second": "Dennis Ritchie"
                },
                "year": 1983,
                "price": 53.60
            }
        ]
    "#;

    #[test]
    fn test_flatten_object_as_bigint() {
        let res = JsonFlatten::json_flatten_bigint(
            TEST_INPUT_OBJECT.to_string(),
            ["$".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedBigint{ name: "category".to_string(), value: 0    },
                FlattenedBigint{ name: "language".to_string(), value: 0    },
                FlattenedBigint{ name: "title".to_string(),    value: 0    },
                FlattenedBigint{ name: "authors".to_string(),  value: 0    },
                FlattenedBigint{ name: "year".to_string(),     value: 2003 },
                FlattenedBigint{ name: "price".to_string(),    value: 0    },
            ]);

        let res = JsonFlatten::json_flatten_bigint(
            TEST_INPUT_OBJECT.to_string(),
            ["$".to_string(), "$.authors".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedBigint{ name: "category".to_string(), value: 0    },
                FlattenedBigint{ name: "language".to_string(), value: 0    },
                FlattenedBigint{ name: "title".to_string(),    value: 0    },
                FlattenedBigint{ name: "authors".to_string(),  value: 0    },
                FlattenedBigint{ name: "year".to_string(),     value: 2003 },
                FlattenedBigint{ name: "price".to_string(),    value: 0    },
                FlattenedBigint{ name: "first".to_string(),    value: 0    },
                FlattenedBigint{ name: "second".to_string(),   value: 0    },
                FlattenedBigint{ name: "third".to_string(),    value: 0    },
                FlattenedBigint{ name: "fourth".to_string(),   value: 0    },
                FlattenedBigint{ name: "fifth".to_string(),    value: 0    },
            ]);

        let res = JsonFlatten::json_flatten_bigint(
            TEST_INPUT_OBJECT.to_string(),
            ["$.category".to_string()].to_vec());
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_flatten_object_as_double() {
        let res = JsonFlatten::json_flatten_double(
            TEST_INPUT_OBJECT.to_string(),
            ["$".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedDouble{ name: "category".to_string(), value: 0.0    },
                FlattenedDouble{ name: "language".to_string(), value: 0.0    },
                FlattenedDouble{ name: "title".to_string(),    value: 0.0    },
                FlattenedDouble{ name: "authors".to_string(),  value: 0.0    },
                FlattenedDouble{ name: "year".to_string(),     value: 2003.0 },
                FlattenedDouble{ name: "price".to_string(),    value: 49.99  },
            ]);

        let res = JsonFlatten::json_flatten_double(
            TEST_INPUT_OBJECT.to_string(),
            ["$".to_string(), "$.authors".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedDouble{ name: "category".to_string(), value: 0.0    },
                FlattenedDouble{ name: "language".to_string(), value: 0.0    },
                FlattenedDouble{ name: "title".to_string(),    value: 0.0    },
                FlattenedDouble{ name: "authors".to_string(),  value: 0.0    },
                FlattenedDouble{ name: "year".to_string(),     value: 2003.0 },
                FlattenedDouble{ name: "price".to_string(),    value: 49.99  },
                FlattenedDouble{ name: "first".to_string(),    value: 0.0    },
                FlattenedDouble{ name: "second".to_string(),   value: 0.0    },
                FlattenedDouble{ name: "third".to_string(),    value: 0.0    },
                FlattenedDouble{ name: "fourth".to_string(),   value: 0.0    },
                FlattenedDouble{ name: "fifth".to_string(),    value: 0.0    },
            ]);

        let res = JsonFlatten::json_flatten_double(
            TEST_INPUT_OBJECT.to_string(),
            ["$.category".to_string()].to_vec());
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_flatten_object_as_string() {
        let res = JsonFlatten::json_flatten_string(
            TEST_INPUT_OBJECT.to_string(),
            ["$".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedString{ name: "category".to_string(), value: "web".to_string()               },
                FlattenedString{ name: "language".to_string(), value: "en".to_string()                },
                FlattenedString{ name: "title".to_string(),    value: "XQuery Kick Start".to_string() },
                FlattenedString{ name: "authors".to_string(),  value: "".to_string()                  },
                FlattenedString{ name: "year".to_string(),     value: "".to_string()                  },
                FlattenedString{ name: "price".to_string(),    value: "".to_string()                  },
            ]);

        let res = JsonFlatten::json_flatten_string(
            TEST_INPUT_OBJECT.to_string(),
            ["$".to_string(), "$.authors".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedString{ name: "category".to_string(), value: "web".to_string()                    },
                FlattenedString{ name: "language".to_string(), value: "en".to_string()                     },
                FlattenedString{ name: "title".to_string(),    value: "XQuery Kick Start".to_string()      },
                FlattenedString{ name: "authors".to_string(),  value: "".to_string()                       },
                FlattenedString{ name: "year".to_string(),     value: "".to_string()                       },
                FlattenedString{ name: "price".to_string(),    value: "".to_string()                       },
                FlattenedString{ name: "first".to_string(),    value: "James McGovern".to_string()         },
                FlattenedString{ name: "second".to_string(),   value: "Per Bothner".to_string()            },
                FlattenedString{ name: "third".to_string(),    value: "Kurt Cagle".to_string()             },
                FlattenedString{ name: "fourth".to_string(),   value: "James Linn".to_string()             },
                FlattenedString{ name: "fifth".to_string(),    value: "Vaidyanathon Nagarajan".to_string() },
            ]);

        let res = JsonFlatten::json_flatten_string(
            TEST_INPUT_OBJECT.to_string(),
            ["$.category".to_string()].to_vec());
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_flatten_object_as_json() {
        let res = JsonFlatten::json_flatten_json(
            TEST_INPUT_OBJECT.to_string(),
            ["$".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedJson{ name: "category".to_string(), value: "\"web\"".to_string()               },
                FlattenedJson{ name: "language".to_string(), value: "\"en\"".to_string()                },
                FlattenedJson{ name: "title".to_string(),    value: "\"XQuery Kick Start\"".to_string() },
                FlattenedJson{ name: "authors".to_string(),  value: "{\"first\":\"James McGovern\",\"second\":\"Per Bothner\",\"third\":\"Kurt Cagle\",\"fourth\":\"James Linn\",\"fifth\":\"Vaidyanathon Nagarajan\"}".to_string() },
                FlattenedJson{ name: "year".to_string(),     value: "2003".to_string()                  },
                FlattenedJson{ name: "price".to_string(),    value: "49.99".to_string()                 },
            ]);

        let res = JsonFlatten::json_flatten_json(
            TEST_INPUT_OBJECT.to_string(),
            ["$".to_string(), "$.authors".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedJson{ name: "category".to_string(), value: "\"web\"".to_string()                    },
                FlattenedJson{ name: "language".to_string(), value: "\"en\"".to_string()                     },
                FlattenedJson{ name: "title".to_string(),    value: "\"XQuery Kick Start\"".to_string()      },
                FlattenedJson{ name: "authors".to_string(),  value: "{\"first\":\"James McGovern\",\"second\":\"Per Bothner\",\"third\":\"Kurt Cagle\",\"fourth\":\"James Linn\",\"fifth\":\"Vaidyanathon Nagarajan\"}".to_string() },
                FlattenedJson{ name: "year".to_string(),     value: "2003".to_string()                       },
                FlattenedJson{ name: "price".to_string(),    value: "49.99".to_string()                      },
                FlattenedJson{ name: "first".to_string(),    value: "\"James McGovern\"".to_string()         },
                FlattenedJson{ name: "second".to_string(),   value: "\"Per Bothner\"".to_string()            },
                FlattenedJson{ name: "third".to_string(),    value: "\"Kurt Cagle\"".to_string()             },
                FlattenedJson{ name: "fourth".to_string(),   value: "\"James Linn\"".to_string()             },
                FlattenedJson{ name: "fifth".to_string(),    value: "\"Vaidyanathon Nagarajan\"".to_string() },
            ]);

        let res = JsonFlatten::json_flatten_json(
            TEST_INPUT_OBJECT.to_string(),
            ["$.category".to_string()].to_vec());
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_flatten_array_as_bigint() {
        let res = JsonFlatten::json_flatten_bigint(
            TEST_INPUT_ARRAY.to_string(),
            ["$".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedBigint{ name: "0".to_string(), value: 0 },
                FlattenedBigint{ name: "1".to_string(), value: 0 },
            ]);

        let res = JsonFlatten::json_flatten_bigint(
            TEST_INPUT_ARRAY.to_string(),
            ["$".to_string(), "$[0].authors".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedBigint{ name: "0".to_string(),      value: 0 },
                FlattenedBigint{ name: "1".to_string(),      value: 0 },
                FlattenedBigint{ name: "first".to_string(),  value: 0 },
                FlattenedBigint{ name: "second".to_string(), value: 0 },
                FlattenedBigint{ name: "third".to_string(),  value: 0 },
                FlattenedBigint{ name: "fourth".to_string(), value: 0 },
                FlattenedBigint{ name: "fifth".to_string(),  value: 0 },
            ]);
    }

    #[test]
    fn test_flatten_array_as_json() {
        let res = JsonFlatten::json_flatten_json(
            TEST_INPUT_ARRAY.to_string(),
            ["$".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedJson{ name: "0".to_string(), value: "{\"category\":\"web\",\"language\":\"en\",\"title\":\"XQuery Kick Start\",\"authors\":{\"first\":\"James McGovern\",\"second\":\"Per Bothner\",\"third\":\"Kurt Cagle\",\"fourth\":\"James Linn\",\"fifth\":\"Vaidyanathon Nagarajan\"},\"year\":2003,\"price\":49.99}".to_string() },
                FlattenedJson{ name: "1".to_string(), value: "{\"category\":\"programming\",\"language\":\"en\",\"title\":\"The C Programming Language\",\"authors\":{\"first\":\"Brian Kernighan\",\"second\":\"Dennis Ritchie\"},\"year\":1983,\"price\":53.6}".to_string() },
            ]);
    }

    #[test]
    fn test_flatten_string_with_emoji() {
        let res = JsonFlatten::json_flatten_string(
            r#"{"validate.👿": "foo"}"#.to_string(),
            ["$".to_string()].to_vec());
        assert_eq!(
            res,
            vec![
                FlattenedString{ name: "validate.👿".to_string(), value: "foo".to_string() },
            ]);
    }
}
