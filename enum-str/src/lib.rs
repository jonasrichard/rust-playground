#[derive(Debug, PartialEq)]
pub enum Incoming<'a> {
    Empty,
    OneString(&'a str),
    StringPair(&'a str, &'a str)
}

pub fn read(data: &[u8]) -> Incoming {
    match data.get(0) {
        None =>
            unimplemented!(),
        Some(1) =>
            Incoming::Empty,
        Some(2) => {
            let len = data.get(1).unwrap();
            let bytes = &data[2..((2 + len) as usize)];
            let first_str = std::str::from_utf8(bytes).unwrap();

            Incoming::OneString(first_str)
        },
        Some(3) =>
            Incoming::StringPair("", ""),
        _ =>
            unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn read_empty() {
        let item = read(&b"\x01"[..]);

        assert_eq!(Incoming::Empty, item);
    }
}
