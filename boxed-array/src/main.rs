struct Queue {
    data: Box<[u32]>,
    free: usize
}

impl Queue {
    fn new(size: usize) -> Self {
        Queue {
            data: (0..size).map(|_| 0).collect::<Vec<u32>>().into_boxed_slice(),
            free: 0
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn put(&mut self, elem: u32) -> Result<(), ()> {
        if self.free < self.data.len() - 1 {
            self.data[self.free] = elem;
            self.free += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    fn get(&mut self) -> Option<u32> {
        if self.free == 0 {
            None
        } else {
            self.free -= 1;
            Some(self.data[self.free])
        }
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put() {
        let mut q = Queue::new(4);

        assert_eq!(4, q.size());

        q.put(1).expect("?");
        q.put(2).expect("?");

        assert_eq!(Some(2), q.get());
        assert_eq!(Some(1), q.get());
        assert_eq!(None, q.get());
    }
}
