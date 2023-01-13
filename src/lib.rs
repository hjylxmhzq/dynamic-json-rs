use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_json() {
        let s = r#"{ "a" : 123, "b": false }"#;
        let json = parse(&s);

        let keys = vec![String::from("a"), String::from("b")];
        let values = vec![JsonType::Number(123f64), JsonType::Bool(false)];

        let hm = keys.into_iter().zip(values).collect();
        let espect = JsonType::Object(hm);
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
        let json1: JsonType = r#"{"a": {"b": 1, "c": 2}}"#.into();
        let json2: JsonType = r#"{"a": {"c": 2, "b": 1}}"#.into();
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
        let json_obj: JsonType = hm.into_iter().collect();

        let mut json_hm = HashMap::new();
        json_hm.insert("a".to_string(), JsonType::Number(1.0));
        json_hm.insert("b".to_string(), JsonType::Number(2.0));
        let espect = JsonType::Object(json_hm);
        assert_eq!(json_obj, espect);
    }
}

/// JsonType can be use for defined any structure of json
///
/// define a json object:
/// ```
/// use std::collections::HashMap;
/// use dynamic_json::JsonType;
/// let mut hm = HashMap::<String, JsonType>::new();
/// hm.insert("a".to_string(), JsonType::Number(100f64));
/// let json = JsonType::Object(hm);
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
/// # use dynamic_json::{JsonType, parse};
/// let json_str = r#"{ "a": [1, 2, null, { "b": 3 }] }"#;
/// let json = parse(json_str);
/// let json1: JsonType = json_str.into();
///
/// assert_eq!(json, json1);
///
/// let object_b = vec!["b".to_string()]
///     .into_iter()
///     .zip(vec![JsonType::Number(3f64)])
///     .collect::<HashMap<String, JsonType>>();
/// // { "b": 3 }
///
/// let arr_a = vec![JsonType::Number(1f64), JsonType::Number(2f64), JsonType::Null, JsonType::Object(object_b)];
/// // [1, 2, null, { "b": 3 }]
///
/// let object_a = vec!["a".to_string()]
///     .into_iter()
///     .zip(vec![JsonType::Array(arr_a)])
///     .collect::<HashMap<String, JsonType>>();
/// // { "a": [1, 2, null, { "b": 3 }] }
///
/// let espect = JsonType::Object(object_a);
/// assert_eq!(json1, espect);
/// ```
#[derive(Debug, PartialEq)]
pub enum JsonType {
    Object(HashMap<String, JsonType>),
    Array(Vec<JsonType>),
    Null,
    Bool(bool),
    Number(f64),
    String(String),
}

impl From<&str> for JsonType {
    fn from(s: &str) -> Self {
        parse(s)
    }
}

impl From<String> for JsonType {
    fn from(s: String) -> Self {
        parse(&s)
    }
}

macro_rules! impl_from_iter_to_array {
    ($t:ident, $y:ident) => {
        impl FromIterator<$t> for JsonType {
            fn from_iter<T: IntoIterator<Item = $t>>(iter: T) -> Self {
                let mut arr = vec![];
                for num in iter {
                    arr.push(JsonType::$y(num.try_into().unwrap()));
                }
                JsonType::Array(arr)
            }
        }
    };
}

macro_rules! impl_from_ref_iter_to_array {
    ($t:ident, $y:ident) => {
        impl<'a> FromIterator<&'a $t> for JsonType {
            fn from_iter<T: IntoIterator<Item = &'a $t>>(iter: T) -> Self {
                let mut arr = vec![];
                for num in iter {
                    arr.push(JsonType::$y(f64::from(*num)));
                }
                JsonType::Array(arr)
            }
        }
    };
}

impl_from_iter_to_array!(i32, Number);
impl_from_iter_to_array!(f32, Number);
impl_from_iter_to_array!(f64, Number);
impl_from_iter_to_array!(bool, Bool);
impl_from_iter_to_array!(String, String);
impl_from_ref_iter_to_array!(i32, Number);
impl_from_ref_iter_to_array!(f32, Number);
impl_from_ref_iter_to_array!(f64, Number);

impl FromIterator<(String, JsonType)> for JsonType {
    fn from_iter<T: IntoIterator<Item = (String, JsonType)>>(iter: T) -> Self {
        let mut obj = HashMap::new();
        for item in iter {
            obj.insert(item.0, item.1);
        }
        JsonType::Object(obj)
    }
}

impl<'a> FromIterator<(&'a str, JsonType)> for JsonType {
    fn from_iter<T: IntoIterator<Item = (&'a str, JsonType)>>(iter: T) -> Self {
        let mut obj = HashMap::new();
        for item in iter {
            obj.insert(item.0.to_string(), item.1);
        }
        JsonType::Object(obj)
    }
}

macro_rules! impl_from_iter_to_object {
    ($t:ident, $y:ident) => {
        impl FromIterator<(String, $t)> for JsonType {
            fn from_iter<T: IntoIterator<Item = (String, $t)>>(iter: T) -> Self {
                let mut obj = HashMap::new();
                for item in iter {
                    obj.insert(item.0, JsonType::$y(item.1.try_into().unwrap()));
                }
                JsonType::Object(obj)
            }
        }

        impl<'a> FromIterator<(&'a str, $t)> for JsonType {
            fn from_iter<T: IntoIterator<Item = (&'a str, $t)>>(iter: T) -> Self {
                let mut obj = HashMap::new();
                for item in iter {
                    obj.insert(item.0.to_string(), JsonType::$y(item.1.try_into().unwrap()));
                }
                JsonType::Object(obj)
            }
        }
    };
}

impl_from_iter_to_object!(i32, Number);
impl_from_iter_to_object!(f32, Number);
impl_from_iter_to_object!(f64, Number);
impl_from_iter_to_object!(bool, Bool);
impl_from_iter_to_object!(String, String);

impl ToString for JsonType {
    fn to_string(&self) -> String {
        self.serialize()
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
                    serialize(value, indent, acc_indent + indent),
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
            let mut obj = HashMap::new();

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
                obj.insert(key, child);
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

fn is_valid_ending_quote(chars: &Vec<char>, idx: usize) -> bool {
    if chars[idx] == '"' {
        if idx == 0 {
            return false;
        }
        let mut is_escaped = false;
        let mut i = idx - 1;
        while chars[i] == '\\' {
            is_escaped = !is_escaped;
            if i == 0 {
                break;
            }
            i -= 1;
        }
        return !is_escaped;
    } else {
        return false;
    }
}

fn match_literal(chars: &Vec<char>, idx: usize, literal: &str) -> bool {
    if idx <= chars.len() - literal.len() {
        for (i, c) in literal.chars().enumerate() {
            if chars[idx + i] != c {
                return false;
            }
            return true;
        }
    }
    return false;
}

fn skip_spaces(chars: &Vec<char>, idx: &mut usize) {
    skip_chars(chars, idx, &[' ', '\r', '\n']);
}

fn skip_chars(chars: &Vec<char>, idx: &mut usize, to_skip: &[char]) {
    while *idx < chars.len() && to_skip.contains(&chars[*idx]) {
        *idx += 1;
    }
}

fn skip_util_char(chars: &Vec<char>, idx: &mut usize, util_chars: &[char]) {
    while *idx < chars.len() && !util_chars.contains(&chars[*idx]) {
        *idx += 1;
    }
}

fn nearby_content(chars: &Vec<char>, idx: usize) -> String {
    let start = if idx < 50 { 0 } else { idx - 50 };
    chars[start..=idx].iter().collect::<String>()
}

fn parse_error(chars: &Vec<char>, idx: usize) -> ! {
    panic!("{}", nearby_content(chars, idx));
}
