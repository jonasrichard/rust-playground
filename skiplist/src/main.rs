use std::{
    cell::{Ref, RefCell, RefMut},
    cmp::Ordering,
    rc::Rc,
};

use rand::prelude::*;

/// Type of the base list
#[derive(Debug)]
struct ListNode<K: Clone + Ord, V> {
    key: K,
    value: V,
    next: Option<Rc<RefCell<ListNode<K, V>>>>,
}

#[derive(Debug)]
struct IndexNode<K: Clone + Ord, V> {
    key: K,
    height: u8,
    next: Option<Rc<RefCell<IndexNode<K, V>>>>,
    next_level: Rc<RefCell<LowerNode<K, V>>>,
}

#[derive(Debug)]
enum LowerNode<K: Clone + Ord, V> {
    Index(IndexNode<K, V>),
    Base(ListNode<K, V>),
}

#[derive(Debug)]
struct SkipList<K: Clone + Ord, V> {
    heads: Vec<Option<Rc<RefCell<IndexNode<K, V>>>>>,
    base: Option<Rc<RefCell<ListNode<K, V>>>>,
}

impl<K: Clone + Ord, V> SkipList<K, V> {
    fn new(height: u8) -> Self {
        SkipList {
            heads: (0..(height - 1)).map(|_| None).collect(),
            base: None,
        }
    }
}

impl<K: Clone + std::fmt::Debug + Ord, V> SkipList<K, V> {
    /// Insert a new element in the head of the base list
    fn prepend(entry: &mut Option<Rc<RefCell<ListNode<K, V>>>>, key: &K, value: V) {
        let new_elem = Rc::new(RefCell::new(ListNode {
            key: key.clone(),
            value,
            next: None,
        }));

        if entry.is_some() {
            let next = entry.as_ref().cloned();

            new_elem.borrow_mut().next = next;
        }

        entry.replace(new_elem);
    }

    // When the look up for a place of the new entry finished we have an entry
    // reference in the base list where we can go on until we find the right
    // place of the new element. If we insert a new element we return its
    // reference, if the element is already in the list we return None.
    fn insert_into_base_list(
        entry: &mut Option<Rc<RefCell<ListNode<K, V>>>>,
        key: &K,
        value: V,
    ) -> Option<Rc<RefCell<ListNode<K, V>>>> {
        if entry.is_none() {
            Self::prepend(entry, key, value);

            return entry.clone();
        }

        let mut prev: Option<Rc<RefCell<ListNode<K, V>>>> = None;
        let mut p = match entry {
            Some(ref node) => Some(Rc::clone(node)),
            None => unreachable!(),
        };

        loop {
            let prc = match p {
                Some(ref pp) => Rc::clone(pp),
                None => unreachable!(),
            };

            let compare = prc.borrow().key.cmp(key);

            match compare {
                Ordering::Less => {
                    // do everything after the match block
                }
                Ordering::Equal => {
                    return None;
                }
                Ordering::Greater => {
                    let new_elem = Rc::new(RefCell::new(ListNode {
                        key: key.clone(),
                        value,
                        next: Some(Rc::clone(&prc)),
                    }));

                    match prev {
                        Some(ref p) => {
                            p.borrow_mut().next = Some(new_elem.clone());
                            //if let Some(ref mut prev_next) = p.borrow_mut().next {
                            //    *prev_next = new_elem.clone();
                            //}
                        }
                        None => {
                            // entry.next = new_elem.clone()
                            entry.replace(new_elem.clone());
                        }
                    }

                    return Some(new_elem);
                }
            }

            prev = Some(Rc::clone(&prc));

            match RefCell::borrow(&prc).next {
                Some(ref next) => {
                    p = Some(Rc::clone(next));
                }
                None => break,
            };
        }

        let new_elem = Rc::new(RefCell::new(ListNode {
            key: key.clone(),
            value,
            next: None,
        }));

        match prev {
            Some(ref mut p) => {
                let mut prev_next = p.borrow_mut();

                prev_next.next = Some(new_elem.clone());
            }
            None => {
                entry.replace(new_elem.clone());
            }
        }

        Some(new_elem)
    }

    fn insert(&mut self, key: &K, value: V) -> bool {
        // build the previous nodes Vec after which we might insert the elem
        // in the skipnodes
        true
    }

    fn choice_to_insert(level: u8) -> bool {
        if level == 0 {
            true
        } else {
            rand::random::<u32>() % (1 << (level << 1)) < (1 << level)
        }
        // level     probability
        // 0            1.0
        // 1            0.25    (1/4)
        // 2            0.0625  (1/16)
        // 3            1/64
    }

    //fn print(&self) {
    //    for i in (0..self.heads.len()).rev() {
    //        let mut h = self.heads[i].clone();

    //        print!("({}) ", i);

    //        while let Some(boxed) = h {
    //            let br = boxed.borrow();

    //            print!("{:?} ", br.value);

    //            h = br.next.as_ref().cloned();
    //        }

    //        println!();
    //    }
    //}
}

impl<K: Clone + Ord, V> ListNode<K, V> {
    fn print(head: &Option<Rc<RefCell<ListNode<K, V>>>>, printer: Box<dyn Fn(&K, &V)>) {
        let mut head = head.clone();

        while let Some(node) = head {
            let b = node.borrow();

            printer(&b.key, &b.value);
            head = b.next.as_ref().cloned();
        }
    }
}

fn main() {
    //let mut list = SkipList::<usize, String>::new(4);
    let mut base = None;

    for i in [10, 8, 12, 9, 15] {
        //let n = SkipList::<usize, String>::insert_into_base_list(&mut base, &i, format!("{}", i));
        SkipList::insert_into_base_list(&mut base, &i, format!("{}", i));
        println!("{:?}", base);
        //println!("new {:?}", n);
    }

    ListNode::print(&base, Box::new(|k, v| print!("{:?} {:?} ", k, v)));

    //for _ in 0..5 {
    //    let value = rand::random::<usize>() % 50;

    //    println!("Insert {}", value);

    //    SkipList::<usize, String>::insert_into_base_list(&mut base, &value, format!("{}", value));

    //    println!("{:?}", base);

    //    ListNode::print(&base, Box::new(|k, v| print!("{:?} {:?} ", k, v)));
    //}
}

#[cfg(test)]
mod tests {

    //fn list_to_vec<T: Clone + std::fmt::Debug + Ord>(head: &Option<Rc<SkipNode<T>>>) -> Vec<T> {
    //    let mut head = head;
    //    let mut result = vec![];

    //    while let Some(h) = head {
    //        result.push(h.value.clone());

    //        head = &h.next;
    //    }

    //    result
    //}

    //#[test]
    //fn insert_first() {
    //    let mut list = SkipList::<usize>::new(4);

    //    list.insert(8);
    //    list.insert(5);

    //    let level0 = list_to_vec(&list.heads[0]);

    //    assert_eq!(vec![5, 8], level0);
    //}
}
