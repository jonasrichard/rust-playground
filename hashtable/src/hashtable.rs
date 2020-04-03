type HashFn<K> = fn(K) -> usize;

struct Entry<K: PartialEq, V> {
    key: K,
    value: V
}

struct Hashtable2<K: PartialEq + Copy, V> {
    entries: Vec<Option<Entry<K, V>>>,
    hasher: HashFn<K>
}

impl<K: PartialEq + Copy, V> Hashtable2<K, V> {
    fn new(size: usize, hasher: HashFn<K>) -> Self {
        Hashtable2 {
            entries: (0..size).map(|_| None).collect(),
            hasher: hasher
        }
    }

    fn size(&self) -> usize {
        self.entries.len()
    }

    fn get(&self, key: K) -> Option<&V> {
        let h = (self.hasher)(key);
        let mut i = h;

        loop {
            match self.entries[i] {
                Some(ref entry) if entry.key == key =>
                    return Some(&entry.value),
                None =>
                    return None,
                _ =>
                    i = (i + 1) % self.entries.len()
            }
            if h == i {
                return None
            }
        }
    }

    fn put(&mut self, key: K, value: V) -> bool {
        let h = (self.hasher)(key);
        let mut i = h;

        loop {
            match self.entries[i] {
                Some(ref entry) if entry.key == key => {
                    self.entries[i] = Some(Entry{ key: key, value: value});
                    return true
                },
                None => {
                    self.entries[i] = Some(Entry{ key: key, value: value });
                    return true
                },
                _ =>
                    i = (i + 1) % self.entries.len()
            }
            if i == h {
                return false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let mut ht: Hashtable2<i32, String> = Hashtable2::new(8, |i| (i % 8) as usize);
        ht.put(4, "Apple".to_string());

        assert_eq!(8, ht.size());
        assert_eq!("Apple", ht.get(4).unwrap());
    }
}
