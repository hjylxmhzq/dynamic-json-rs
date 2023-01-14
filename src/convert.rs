use crate::{InsertJsonObject, JsonObject, JsonType};

impl From<&str> for JsonType {
    fn from(s: &str) -> Self {
        JsonType::String(s.to_string())
    }
}

impl From<String> for JsonType {
    fn from(s: String) -> Self {
        JsonType::String(s)
    }
}

impl From<i32> for JsonType {
    fn from(v: i32) -> Self {
        JsonType::Number(f64::from(v))
    }
}

impl From<f64> for JsonType {
    fn from(v: f64) -> Self {
        JsonType::Number(f64::from(v))
    }
}

impl From<bool> for JsonType {
    fn from(v: bool) -> Self {
        JsonType::Bool(v)
    }
}

impl From<JsonObject> for JsonType {
    fn from(v: JsonObject) -> Self {
        JsonType::Object(v)
    }
}

impl<T: Into<JsonType>> From<Vec<T>> for JsonType {
    fn from(v: Vec<T>) -> Self {
        let mut arr = vec![];
        for i in v {
            arr.push(i.into());
        }
        JsonType::Array(arr)
    }
}

impl From<&i32> for JsonType {
    fn from(v: &i32) -> Self {
        JsonType::Number(f64::from(*v))
    }
}

impl From<&f64> for JsonType {
    fn from(v: &f64) -> Self {
        JsonType::Number(f64::from(*v))
    }
}

impl From<&bool> for JsonType {
    fn from(v: &bool) -> Self {
        JsonType::Bool(*v)
    }
}

impl From<&JsonObject> for JsonType {
    fn from(v: &JsonObject) -> Self {
        JsonType::Object(v.clone())
    }
}

impl<V: Into<JsonType>> FromIterator<V> for JsonType {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut arr = vec![];
        for num in iter {
            arr.push(num.into());
        }
        JsonType::Array(arr)
    }
}

impl<V: Into<JsonType>> FromIterator<(String, V)> for JsonType {
    fn from_iter<T: IntoIterator<Item = (String, V)>>(iter: T) -> Self {
        let mut obj = JsonObject::new();
        for item in iter {
            obj.insert(&item.0, item.1.into());
        }
        JsonType::Object(obj)
    }
}

impl<V: Into<JsonType>> FromIterator<(String, V)> for JsonObject {
    fn from_iter<T: IntoIterator<Item = (String, V)>>(iter: T) -> Self {
        let mut obj = JsonObject::new();
        for item in iter {
            obj.insert(&item.0, item.1.into());
        }
        obj
    }
}

impl<'a, V: Into<JsonType>> FromIterator<(&'a str, V)> for JsonType {
    fn from_iter<T: IntoIterator<Item = (&'a str, V)>>(iter: T) -> Self {
        let mut obj = JsonObject::new();
        for item in iter {
            obj.insert(&item.0, item.1.into());
        }
        JsonType::Object(obj)
    }
}

impl<'a, V: Into<JsonType>> FromIterator<(&'a str, V)> for JsonObject {
    fn from_iter<T: IntoIterator<Item = (&'a str, V)>>(iter: T) -> Self {
        let mut obj = JsonObject::new();
        for item in iter {
            obj.insert(&item.0, item.1.into());
        }
        obj
    }
}

impl ToString for JsonType {
    fn to_string(&self) -> String {
        self.serialize()
    }
}
