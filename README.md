# dynamic_json

A rust lib for parsing and serializing json with non-predefined schema.

## usage

use JsonType to create any type of json object

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

