#[derive(Debug, PartialEq)]
enum PoolItem {
    Available(i32),
    Busy
}

#[derive(Debug)]
struct Pool {
    conns: Vec<PoolItem>
}

impl Pool {
    fn new() -> Self {
        let mut cs = Vec::new();

        for i in 0..4 {
            cs.push(PoolItem::Available(i));
        }

        Pool {
            conns: cs
        }
    }

    fn get(&mut self) -> Option<PoolItem> {
        match self.conns.iter().position(|e| *e != PoolItem::Busy) {
            None =>
                None,
            Some(pos) => {
                println!("{}", pos);
                let (_left, right) = self.conns.split_at_mut(pos);

                match right[0] {
                    PoolItem::Busy =>
                        None,
                    PoolItem::Available(n) => {
                        right[0] = PoolItem::Busy;
                        Some(PoolItem::Available(n))
                    }
                }
            }
        }
    }

    fn release(&mut self, item: PoolItem) {
        match item {
            PoolItem::Available(n) =>
                match self.conns.iter().position(|e| *e == PoolItem::Busy) {
                    None =>
                        // panic
                        (),
                    Some(pos) => {
                        let e = self.conns.get_mut(pos).unwrap();
                        *e = PoolItem::Available(n);
                        ()
                    }
                },
            _ =>
                ()
        }
    }
}

fn main() {
    let mut pool = Pool::new();

    let res2 = pool.get().unwrap();
    println!("{:?}", pool);

    pool.release(res2);
    println!("{:?}", pool);
}
