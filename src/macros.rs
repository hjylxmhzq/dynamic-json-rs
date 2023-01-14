#[macro_export]
macro_rules! json_obj {
    () => {
        {
            JsonObject::new();
        }
    };
    ($key:expr => $value:expr) => {
        {
            vec![($key, JsonType::from($value))].into_iter().collect::<JsonObject>()
        }
    };
    ($key:expr => $value:expr,$($kx:expr => $vx:expr),*) => {
        {
            vec![($key, JsonType::from($value)), $(($kx, JsonType::from($vx))),*].into_iter().collect::<JsonType>()
        }
    };
    ($key:expr => $value:expr;$($kx:expr => $vx:expr),*) => {
        {
            vec![($key, JsonType::from($value)), $(($kx, JsonType::from($vx))),*].into_iter().collect::<JsonType>()
        }
    }
}
#[macro_export]
macro_rules! json_arr {
    () => {
        {
            let arr = vec![];
            JsonType::Array(arr)
        }
    };
    ($value:expr) => {
        {
            let arr = vec![JsonType::from($value)];
            JsonType::Array(arr)
        }
    };
    ($value:expr,$($vx:expr),*) => {
        {
            let json: JsonType = vec![JsonType::from($value), $(JsonType::from($vx)),*].into();
            json
        }
    };
    ($value:expr; $n:expr) => {
        {
            let json: JsonType = vec![JsonType::from($value); $n].into();
            json
        }
    }
}
#[macro_export]
macro_rules! json_null {
    () => {
        JsonType::Null
    };
}

#[cfg(test)]
mod test {
    use crate::{JsonObject, JsonType};
    #[test]
    fn macro_json_obj() {
        let json = json_obj! {
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
    }
}
