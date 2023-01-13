use std::{collections::HashMap};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_json() {
        let s = r#"{ "a" : 123 }"#;
        let json = parse(&s);

        let key: String = "a".into();
        let value = 123f64;

        let hm = vec![key]
            .into_iter()
            .zip(vec![JsonType::Number(value)])
            .collect::<HashMap<_, _>>();
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
            if is_null(chars, idx) {
                idx += 4;
                skip_spaces(chars, &mut idx);
                return (JsonType::Null, idx);
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

fn is_null(chars: &Vec<char>, idx: usize) -> bool {
    if idx <= chars.len() - 4 {
        return chars[idx..idx + 4] == "null".chars().collect::<Vec<char>>();
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
