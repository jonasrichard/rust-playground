use std::collections::HashMap;
use std::io::{self, BufReader, Read};

#[derive(Debug, PartialEq)]
enum Json {
    JsonNumber(i32),
    JsonString(String),
    JsonArray(Vec<Json>),
    JsonObject(HashMap<String, Json>)
}

// TODO skip whitespaces

fn read_number(input: &Vec<char>, pos: &mut usize) -> Result<Json, String> {
    let mut result = String::new();

    while *pos < input.len() {
        match input[*pos] {
            c if c >= '0' && c <= '9' =>
                result.push(c),
            _ => {
                *pos -= 1;
                return Ok(Json::JsonNumber(result.parse::<i32>().unwrap()))
            }
        }
        *pos += 1
    }

    Ok(Json::JsonNumber(result.parse::<i32>().unwrap()))
}

fn read_string(input: &Vec<char>, pos: &mut usize) -> Result<String, String> {
    let mut string = String::new();

    // skip the opening "
    *pos += 1;

    while *pos < input.len() {
        match input[*pos] {
            '"' => {
                return Ok(string)
            },
            c =>
                string.push(c)
        }

        *pos += 1
    }

    Err("Reaching the end without closing \"".to_string())
}

fn read_value(input: &Vec<char>, mut pos: &mut usize) -> Result<Json, String> {
    match input[*pos] {
        '"' => {
            let value = read_string(input, &mut pos)?;
            return Ok(Json::JsonString(value))
        },
        c if c >= '0' && c <= '9' =>
            read_number(input, &mut pos),
        _ =>
            Err("Unknown char".to_string())
    }
}

fn read_array(input: &Vec<char>, pos: &mut usize) -> Result<Json, String> {
    let mut array = Vec::new();

    while *pos < input.len() {
        match input[*pos] {
            ']' =>
                return Ok(Json::JsonArray(array)),
            n if n >= '0' && n <= '9' => {
                match read_number(input, pos) {
                    Ok(number) =>
                        array.push(number),
                    Err(e) =>
                        return Err(e)
                }
            }
            ',' =>
                (),
            _ =>
                ()
        }
        *pos += 1
    }

    Err("Reacing the end without ]".to_string())
}

fn read_object(input: &Vec<char>, mut pos: &mut usize) -> Result<Json, String> {
    let mut result = HashMap::new();

    while *pos < input.len() {
        match input[*pos] {
            '}' =>
                return Ok(Json::JsonObject(result)),
            '"' => {
                // field name, color and value
                let name = read_string(&input, &mut pos)?;

                // step over the ':'
                while *pos < input.len() {
                    if input[*pos] == ':' {
                        *pos += 1;
                        break
                    }
                    *pos += 1
                }

                let value = read_value(&input, &mut pos)?;

                println!("{:?}", value);

                result.insert(name, value);
            },
            _ =>
                ()
        }

        *pos += 1
    }

    Err("Missing }".to_string())
}

fn read_json_from_chars(input: &Vec<char>, pos: &mut usize) -> Result<Json, String> {
    while *pos < input.len() {
        match input[*pos] {
            '{' =>
                return read_object(input, pos),
            '[' =>
                return read_array(input, pos),
            _ =>
                *pos += 1
        }
    }

    Err("Missing data".to_string())
}

fn read_json(input: String) -> Result<Json, String> {
    let chars = input.chars().collect();
    let mut pos: usize = 0;

    return read_json_from_chars(&chars, &mut pos);
}

fn main() {
    let mut input_file = BufReader::new(io::stdin());
    let mut input = String::new();

    input_file.read_to_string(&mut input).expect("Cannot read file");

    read_json(input).expect("json format error");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_array() -> Result<(), String> {
        let array = read_json("[1,2]".to_string()).expect("");

        assert_eq!(array, make_array(vec![1, 2]));
        Ok(())
    }

    #[test]
    fn test_object() -> Result<(), String> {
        let obj = read_json("{\"field1\":5,\"field2\":\"apple\"}".to_string()).expect("");

        let mut map = HashMap::new();
        map.insert("field1".to_string(), Json::JsonNumber(5));
        map.insert("field2".to_string(), Json::JsonString("apple".to_string()));

        let json_obj = Json::JsonObject(map);
        assert_eq!(obj, json_obj);
        Ok(())
    }

    fn make_array(arr: Vec<i32>) -> Json {
        let mut result = Vec::new();

        for i in arr {
            result.push(Json::JsonNumber(i));
        }

        return Json::JsonArray(result)
    }
}
