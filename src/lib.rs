mod convert;
mod utils;
mod macros;

use std::collections::{hash_map::{IntoIter, Iter}, HashMap};
use utils::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_json() {
        let s = r#"{ "a" : 123, "b": false }"#;
        let json = parse(&s);

        let keys = vec![String::from("a"), String::from("b")];
        let values = vec![JsonType::Number(123f64), JsonType::Bool(false)];

        let espect: JsonType = keys.into_iter().zip(values).collect();
        assert_eq!(json, espect);
    }

    #[test]
    fn get_path() {
        let s = r#"{ "a": { "b": 123, "c": [1, 2, 3] } }"#;
        let json = parse(&s);
        let b = json.get("a.b").unwrap();
        if let JsonType::Number(num) = b {
            assert_eq!(*num, 123f64);
        } else {
            panic!("get path error");
        }

        let value = json.get("a.c.1").unwrap();
        if let JsonType::Number(num) = value {
            assert_eq!(*num, 2f64);
        } else {
            panic!("get path error");
        }
    }

    #[test]
    fn serialize_json() {
        let s = r#"{ "a": { "c": [1, 2, "3"] } }"#;
        let json = parse(&s);
        let serialized = json.serialize();
        let espect = r#"{"a":{"c":[1,2,"3"]}}"#;
        assert_eq!(serialized, espect);
    }

    #[test]
    fn pertty_serialize_json() {
        let s = r#"{ "a": { "c": [1, 2, "3"] } }"#;
        let json = parse(&s);
        let serialized = json.pretty_serialize(4);
        let espect = r#"{
    "a": {
        "c": [
            1,
            2,
            "3"
        ]
    }
}"#;
        assert_eq!(serialized, espect);
    }

    #[test]
    fn equalize() {
        let json1: JsonType = r#"{"a": {"b": 1, "c": 2}}"#.to_json();
        let json2: JsonType = r#"{"a": {"c": 2, "b": 1}}"#.to_json();
        assert_eq!(json1, json2);
        assert!(json1 == json2);
    }

    #[test]
    fn convert() {
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
        let result: JsonType = hm.into_iter().collect();

        let mut json_obj = JsonObject::new();
        json_obj.insert("a", 1);
        json_obj.insert("b", 2);
        let espect = JsonType::Object(json_obj);
        assert_eq!(result, espect);
    }
}

/// JsonType can be use for defined any structure of json
///
/// define a json object:
/// ```
/// use dynamic_json::{JsonType, JsonObject, InsertJsonObject};
/// let mut json_obj = JsonObject::new();
/// json_obj.insert("a", 100);
/// let json = JsonType::Object(json_obj);
/// // json: { "a": 100 }
/// ```
///
/// define a json array:
/// ```
/// # use std::collections::HashMap;
/// # use dynamic_json::JsonType;
/// let arr = vec![JsonType::Number(100f64), JsonType::Null, JsonType::String("this is a string".to_string())];
/// let json = JsonType::Array(arr);
/// // json: [100, null, "this is a string"]
/// ```
///
/// parse a string to JsonType:
/// ```
/// # use std::collections::HashMap;
/// # use dynamic_json::{JsonType, parse, ToJson};
/// let json_str = r#"{ "a": [1, 2, null, { "b": 3 }] }"#;
/// let json = parse(json_str);
/// let json1 = json_str.to_json();
///
/// assert_eq!(json, json1);
///
/// let object_b = vec!["b".to_string()]
///     .into_iter()
///     .zip(vec![JsonType::Number(3f64)])
///     .collect::<JsonType>();
/// // { "b": 3 }
///
/// let arr_a = vec![JsonType::Number(1f64), JsonType::Number(2f64), JsonType::Null, object_b];
/// // [1, 2, null, { "b": 3 }]
///
/// let espect = vec!["a".to_string()]
///     .into_iter()
///     .zip(vec![JsonType::Array(arr_a)])
///     .collect::<JsonType>();
/// // { "a": [1, 2, null, { "b": 3 }] }
///
/// assert_eq!(json1, espect);
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum JsonType {
    Object(JsonObject),
    Array(Vec<JsonType>),
    Null,
    Bool(bool),
    Number(f64),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct JsonObject {
    inner_map: HashMap<String, JsonType>,
}

pub trait InsertJsonObject<T> {
    fn insert(&mut self, k: &str, v: T) -> Option<JsonType>;
}

impl JsonObject {
    pub fn new() -> Self {
        JsonObject {
            inner_map: HashMap::new(),
        }
    }
    pub fn get(&self, k: &str) -> Option<&JsonType> {
        self.inner_map.get(k)
    }
    pub fn remove(&mut self, k: &str) -> Option<JsonType> {
        self.inner_map.remove(k)
    }
    pub fn contains(&self, k: &str) -> bool {
        self.inner_map.contains_key(k)
    }
}

impl<T: Into<JsonType>> InsertJsonObject<T> for JsonObject {
    fn insert(&mut self, k: &str, v: T) -> Option<JsonType> {
        self.inner_map.insert(k.to_string(), v.into())
    }
}

impl IntoIterator for JsonObject {
    type Item = (String, JsonType);
    type IntoIter = IntoIter<String, JsonType>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner_map.into_iter()
    }
}

impl<'a> IntoIterator for &'a JsonObject {
    type Item = (&'a String, &'a JsonType);
    type IntoIter = Iter<'a, String, JsonType>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner_map.iter()
    }
}

impl JsonType {
    /// serialize a Json Object to string
    pub fn serialize(&self) -> String {
        serialize(self, 0, 0)
    }

    /// serialize a Json Object to string with indent and line wrap
    pub fn pretty_serialize(&self, indent: u32) -> String {
        serialize(self, indent, indent)
    }

    /// get sub item inside a JsonType
    ///
    /// samples:
    ///
    /// ```
    /// use dynamic_json::{JsonType, parse};
    /// let s = r#"{ "a": { "b": 123, "c": [1, 2, 3] } }"#;
    /// let json = parse(&s);
    /// let b = json.get("a.b").unwrap();
    /// if let JsonType::Number(num) = b {
    ///     assert_eq!(*num, 123f64);
    /// } else {
    ///     panic!("get item error");
    /// }

    /// let value = json.get("a.c.1").unwrap();
    /// if let JsonType::Number(num) = value {
    ///     assert_eq!(*num, 2f64);
    /// } else {
    ///     panic!("get item error");
    /// }
    /// ```
    pub fn get(&self, path: &str) -> Result<&JsonType, String> {
        let path_list: Vec<&str> = path.split('.').into_iter().collect();
        let mut node = self;
        let mut deep = 0;
        for key in path_list.iter() {
            deep += 1;
            match node {
                JsonType::Object(obj) => {
                    if let Some(value) = obj.get(*key) {
                        node = value;
                        continue;
                    } else {
                        let p = path_list.clone();
                        let s = p[0..deep].join(".");
                        return Err(format!("can not found item {}", &s));
                    }
                }
                JsonType::Array(arr) => {
                    if let Ok(idx) = key.parse::<usize>() {
                        node = &arr[idx];
                    } else {
                        let p = path_list.clone();
                        let s = p[0..deep].join(".");
                        return Err(format!("can not found item {}", &s));
                    }
                }
                _ => {
                    let p = path_list.clone();
                    let s = p[0..deep].join(".");
                    return Err(format!("can not found item {}", &s));
                }
            }
        }

        Ok(node)
    }
}

pub fn serialize(json: &JsonType, indent: u32, acc_indent: u32) -> String {
    let wrap = if indent > 0 {
        " ".repeat(acc_indent as usize)
    } else {
        "".to_string()
    };
    let wrap_start = if indent > 0 { "\n" } else { "" };
    let wrap_end = if indent > 0 {
        "\n".to_string() + &" ".repeat((acc_indent - indent) as usize)
    } else {
        "".to_string()
    };
    let gap = if indent > 0 { " " } else { "" };
    let sep = format!(",{}", wrap_start);
    match json {
        JsonType::Null => "null".to_string(),
        JsonType::Number(num) => num.to_string(),
        JsonType::String(s) => {
            format!(r#""{}""#, s)
        }
        JsonType::Bool(boolean) => boolean.to_string(),
        JsonType::Object(obj) => {
            let mut items: Vec<String> = vec![];
            for (key, value) in obj {
                items.push(format!(
                    r#"{}"{}":{}{}"#,
                    wrap,
                    key,
                    gap,
                    serialize(&value, indent, acc_indent + indent),
                ));
            }

            let s = format!("{{{}{}{}}}", wrap_start, items.join(&sep), wrap_end);
            s
        }
        JsonType::Array(arr) => {
            let mut items = vec![];
            for item in arr {
                items.push(format!(
                    "{}{}",
                    wrap,
                    serialize(item, indent, acc_indent + indent)
                ));
            }
            format!("[{}{}{}]", wrap_start, items.join(&sep), wrap_end)
        }
    }
}

pub trait ToJson {
    fn to_json(&self) -> JsonType;
}

impl ToJson for String {
    fn to_json(&self) -> JsonType {
        parse(self)
    }
}

impl ToJson for &str {
    fn to_json(&self) -> JsonType {
        parse(*self)
    }
}

pub fn parse(s: &str) -> JsonType {
    let mut chars: Vec<char> = s.chars().collect();
    let (json, _) = dynamic_json(&mut chars, 0);
    json
}

fn dynamic_json(chars: &mut Vec<char>, start: usize) -> (JsonType, usize) {
    let mut idx = start;

    skip_spaces(chars, &mut idx);

    let c = chars[idx];

    match c {
        '{' => {
            let mut obj = JsonObject::new();

            idx += 1;

            while chars[idx] != '}' {
                skip_spaces(chars, &mut idx);
                if chars[idx] == '}' {
                    break;
                }
                skip_chars(chars, &mut idx, &['"']);
                let mut key = String::new();
                while !is_valid_ending_quote(&chars, idx) {
                    key.push(chars[idx]);
                    idx += 1;
                }
                idx += 1;
                skip_util_char(chars, &mut idx, &[':']);
                idx += 1;
                let (child, new_idx) = dynamic_json(chars, idx);
                idx = new_idx;
                obj.insert(&key, child);
                skip_chars(chars, &mut idx, &[' ', '\r', '\n', ',', '"']);
            }
            return (JsonType::Object(obj), idx + 1);
        }
        '[' => {
            let mut arr = Vec::<JsonType>::new();
            idx += 1;
            while chars[idx] != ']' {
                skip_spaces(chars, &mut idx);
                if chars[idx] == ']' {
                    break;
                }
                let (child, new_idx) = dynamic_json(chars, idx);
                idx = new_idx;
                arr.push(child);
                skip_chars(chars, &mut idx, &[',', ' ', '\r', '\n']);
            }
            (JsonType::Array(arr), idx + 1)
        }
        '0'..='9' => {
            let mut i = idx;
            while (chars[i] <= '9' && chars[i] >= '0') || chars[i] == '.' {
                i += 1;
            }
            let s: String = chars[idx..i].iter().collect();
            let num: f64 = s.trim().parse().unwrap();
            return (JsonType::Number(num), i);
        }
        '"' => {
            let mut i = idx + 1;
            while !is_valid_ending_quote(chars, i) {
                i += 1;
            }
            let s: String = chars[idx + 1..i].iter().collect();
            i += 1;
            return (JsonType::String(s), i);
        }
        'n' => {
            if match_literal(chars, idx, "null") {
                idx += 4;
                skip_spaces(chars, &mut idx);
                return (JsonType::Null, idx);
            }
            parse_error(chars, idx);
        }
        't' => {
            if match_literal(chars, idx, "true") {
                idx += 4;
                skip_spaces(chars, &mut idx);
                return (JsonType::Bool(true), idx);
            }
            parse_error(chars, idx);
        }
        'f' => {
            if match_literal(chars, idx, "false") {
                idx += 5;
                skip_spaces(chars, &mut idx);
                return (JsonType::Bool(false), idx);
            }
            parse_error(chars, idx);
        }
        _ => parse_error(chars, idx),
    }
}
