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

    fn insert(&mut self, key: u32, value: String) -> bool {
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

    ht.insert(3, "Test".to_string());
    ht.print();
}
