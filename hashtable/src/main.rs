struct Item {
    key: u32,
    value: String
}

struct Hashtable {
    size: usize,
    values: Vec<Option<Item>>
}

impl Hashtable {
    fn new() -> Self {
        let mut vals = Vec::<Option<Item>>::new();

        vals.resize_with(64, || None);

        Hashtable {
            size: 64,
            values: vals
        }
    }

    fn hash(&self, key: u32) -> usize {
        (key % self.size as u32) as usize
    }

    fn put(&mut self, key: u32, value: String) -> bool {
        let h = self.hash(key);

        match self.values[h] {
            None => {
                self.values[h].get_or_insert(Item{key: key, value: value});
                true
            },
            _ =>
                false
        }
    }

    fn get(&self, key: u32) -> Option<String> {
        let h = self.hash(key);

        match self.values[h].as_ref() {
            Some(item) if item.key == h as u32 =>
                Some(String::from(&item.value)),
            _ =>
                None
        }
    }

    fn print(&self) {
        for i in 0..self.values.len() {
            match &self.values[i] {
                None =>
                    (),
                Some(item) =>
                    println!("{} => {}", item.key, item.value)
            }
        }
    }
}

fn main() {
    let mut ht = Hashtable::new();

    ht.put(3, "Test".to_string());
    ht.print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut ht = Hashtable::new();
        ht.put(5, "Apple".to_string());

        assert_eq!(ht.get(5), Some("Apple".to_string()));
        assert_eq!(ht.get(5 + 64), None);
        assert_eq!(ht.get(4), None);
    }
}
