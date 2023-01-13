# dynamic_json

A rust lib for parsing and serializing json with non-predefined schema.

## usage

add dependency in Cargo.toml

```toml
[dependencies]
dynamic_json = { git = "https://github.com/hjylxmhzq/dynamic-json-rs.git" }
```

## samples

use JsonType to create any type of json object:

```rust
use std::collections::HashMap;
use dynamic_json::JsonType;
let mut hm = HashMap::<String, JsonType>::new();
hm.insert("a".to_string(), JsonType::Number(100f64));
let json = JsonType::Object(hm);
// json: { "a": 100 }

// all JsonType
pub enum JsonType {
    Object(HashMap<String, JsonType>),
    Array(Vec<JsonType>),
    Null,
    Number(f64),
    String(String),
}
```

define a json array:

```rust
# use std::collections::HashMap;
# use dynamic_json::JsonType;
let arr = vec![JsonType::Number(100f64), JsonType::Null, JsonType::String("this is a string".to_string())];
let json = JsonType::Array(arr);
// json: [100, null, "this is a string"]
```

parse string to JsonType:

```rust
# use std::collections::HashMap;
# use dynamic_json::{JsonType, parse};
let json_str = r#"{ "a": [1, 2, null, { "b": 3 }] }"#;

// use parse() function
let json = parse(json_str);

// or use into()
// JsonType also implemented From<&str> and From<String> trait
let json1: JsonType = json_str.into();

assert_eq!(json, json1);

let object_b = vec!["b".to_string()]
    .into_iter()
    .zip(vec![JsonType::Number(3f64)])
    .collect::<HashMap<String, JsonType>>();
// { "b": 3 }

let arr_a = vec![JsonType::Number(1f64), JsonType::Number(2f64), JsonType::Null, JsonType::Object(object_b)];
// [1, 2, null, { "b": 3 }]

let object_a = vec!["a".to_string()]
    .into_iter()
    .zip(vec![JsonType::Array(arr_a)])
    .collect::<HashMap<String, JsonType>>();
// { "a": [1, 2, null, { "b": 3 }] }

let espect = JsonType::Object(object_a);
assert_eq!(json1, espect);
```

get sub item of json:

```rust
use dynamic_json::{JsonType, parse};
let s = r#"{ "a": { "b": 123, "c": [1, 2, 3] } }"#;
let json = parse(&s);
let b = json.get("a.b").unwrap();
if let JsonType::Number(num) = b {
    assert_eq!(*num, 123f64);
} else {
    panic!("path error");
}

let value = json.get("a.c.1").unwrap();
if let JsonType::Number(num) = value {
    assert_eq!(*num, 2f64);
} else {
    panic!("path error");
}
```

also you can compare two json:

```rust
// JsonType is derived PartialEq, so we can use assert_eq && ==
// be careful that number in json is described by f64, sometimes they can not be compared in simple ways.
let json1: JsonType = r#"{"a": {"b": 1, "c": 2}}"#.into();
let json2: JsonType = r#"{"a": {"c": 2, "b": 1}}"#.into();
assert_eq!(json1, json2);
assert!(json1 == json2);
```
