#[derive(Debug)]
enum PoolItem {
    Available(Vec<i32>),
    Busy
}

#[derive(Debug)]
struct Pool {
    items: Vec<PoolItem>
}

impl Pool {
    fn new(size: usize) -> Self {
        let mut items = Vec::new();

        for _ in 0..size {
            items.push(PoolItem::Available(Vec::new()));
        }

        Pool {
            items: items
        }
    }

    //fn get(&mut self, i: usize) -> Option<Vec<i32>> {
    //    match self.items[i] {
    //        PoolItem::Busy =>
    //            None,
    //        PoolItem::Available(vec) => {
    //            self.items[i] = PoolItem::Busy;
    //            Some((*vec).to_vec())
    //        }
    //    }
    //}
}

fn main() {
    let mut pool = Pool::new(4);

    //let mut v2 = pool.get(2);

    println!("{:?}", pool);
}
