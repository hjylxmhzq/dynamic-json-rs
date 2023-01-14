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
use dynamic_json::{JsonType, JsonObject, InsertJsonObject};
let mut json_obj = JsonObject::new();
json_obj.insert("a", 100);
let json = JsonType::Object(json_obj);
// json: { "a": 100 }

// all JsonType
pub enum JsonType {
    Object(JsonObject),
    Array(Vec<JsonType>),
    Null,
    Bool(bool),
    Number(f64),
    String(String),
}
```

define a json array:

```rust
use dynamic_json::JsonType;
let arr = vec![
  JsonType::Number(100f64),
  JsonType::Null,
  JsonType::String("this is a string".to_string()),
];
let json = JsonType::Array(arr);
// json: [100, null, "this is a string"]
```

parse string to JsonType:

```rust
use dynamic_json::{JsonType, ToJson, JsonObject, InsertJsonObject, parse};
let json_str = r#"{ "a": [1, 2, null, { "b": 3 }] }"#;

// use parse() function
let json = parse(json_str);
let json1 = json_str.to_json();

assert_eq!(json, json1);

let object_b: JsonType = vec!["b".to_string()]
    .into_iter()
    .zip(vec![JsonType::Number(3f64)])
    .collect();
// { "b": 3 }

let arr_a = vec![
  JsonType::Number(1f64),
  JsonType::Number(2f64),
  JsonType::Null,
  object_b,
];
// [1, 2, null, { "b": 3 }]

let object_a = JsonObject::new();
object_a.insert("a", JsonType::Array(arr_a));
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

serialize JsonType to string:

```rust
let json: JsonType = r#"{"a": {"b": 1, "c": 2}}"#.into();

let serialized = json.serialize();
// {"a":{"c":2,"b":1}}

let prettied_serialized = json.pretty_serialize(2);
// {
//   "a": {
//     "c": 2,
//     "b": 1
//   }
// }
```

convert other types to json:

```rust
// JsonType is implemented many FromIterator<T> trait, so you can convert collection types to json
let arr = vec![1.0, 2.0, 3.0, 4.0];
let json_arr: JsonType = arr.iter().collect();
let espect = JsonType::Array(vec![
    JsonType::Number(1.0),
    JsonType::Number(2.0),
    JsonType::Number(3.0),
    JsonType::Number(4.0),
]);
assert_eq!(json_arr, espect);

let mut hm = HashMap::new();
hm.insert("a", 1);
hm.insert("b", 2);
let json_obj: JsonType = hm.into_iter().collect();

let mut json_hm = HashMap::new();
json_hm.insert("a".to_string(), JsonType::Number(1.0));
json_hm.insert("b".to_string(), JsonType::Number(2.0));
let espect = JsonType::Object(json_hm);
assert_eq!(json_obj, espect);

let json_string: JsonType = "abc".into();
assert(JsonType::String("abc".to_string()), json_string);

let json_number: JsonType = 100.into();
assert(JsonType::Number(100), json_number);

let json_bool: JsonType = true.into();
assert(JsonType::Bool(true), json_bool);
```

if you want to create a json object with complex stucture, some macros will help you

```rust
use dynamic_json::{JsonType, json_arr, json_obj, json_null};

let json: JsonType = json_obj! {
    "a" => 123,
    "b" => "this is a string",
    "c" => true,
    "d" => json_arr![1, json_null!(), 2],
    "nested" => json_obj! {
        "array_len_5_filled_zero" => json_arr![0; 5],
        "obj_array" => json_arr![
            json_obj! {
                "key1" => "value1"
            },
            json_obj! {
                "key2" => "value2"
            }
        ]
    }
};
let value2 = json.get("nested.obj_array.1.key2").unwrap();
assert_eq!(*value2, JsonType::from("value2"));
```
