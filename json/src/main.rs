use std::collections::HashMap;
use std::io::{self, Read};
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum JsonValue {
    JsonNumber(i32),
    JsonString(String)
}

#[derive(Debug, PartialEq)]
enum Json {
    JsonArray(Vec<JsonValue>),
    JsonObject(HashMap<String, Json>)
}

fn read_stdin() -> String {
    let mut buffer = String::new();

    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) =>
            return buffer,
        Err(_) =>
            return "".to_string()
    }
}

// TODO skip whitespaces

fn read_array(input: &mut Chars) -> Result<Json, String> {
    loop {
        match input.next() {
            Some(']') =>
                return Ok(Json::JsonArray(vec![JsonValue::JsonNumber(42)])),
            Some(_) =>
                continue,
            None =>
                return Err("Array is not terminated".to_string())
        }
    }
}

fn read_object(input: &mut Chars) -> Result<Json, String> {
    loop {
        match input.next() {
            Some('}') =>
                return Ok(Json::JsonObject(HashMap::new())),
            Some(_) =>
                continue,
            None =>
                return Err("Object is not terminated".to_string())
        }
    }
}

fn read_json(input: &mut Chars) -> Result<Json, String> {
    match input.next() {
        Some('{') =>
            read_object(input),
        Some('[') =>
            read_array(input),
        Some(_) =>
            Err("Invalid char".to_string()),
        None =>
            Err("Unknown error".to_string())
    }
}

fn main() {
    let input = read_stdin();

    read_json(&mut input.chars());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_array() -> Result<(), String> {
        let arr = read_json(&mut "[1,2]".chars())?;
        assert_eq!(arr, make_array(vec![1, 2]));
        Ok(())
    }

    fn make_array(arr: Vec<i32>) -> Json {
        let mut result = Vec::new();

        for i in arr {
            result.push(JsonValue::JsonNumber(i));
        }

        return Json::JsonArray(result)
    }
}
