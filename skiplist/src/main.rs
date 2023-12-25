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
    next: Option<ListNodeRef<K, V>>,
}

type ListNodeRef<K, V> = Rc<RefCell<ListNode<K, V>>>;

#[derive(Debug)]
struct IndexNode<K: Clone + Ord, V> {
    key: K,
    height: u8,
    next: Option<IndexNodeRef<K, V>>,
    next_level: LowerNode<K, V>,
}

type IndexNodeRef<K, V> = Rc<RefCell<IndexNode<K, V>>>;

#[derive(Debug)]
enum LowerNode<K: Clone + Ord, V> {
    Index(IndexNodeRef<K, V>),
    Base(ListNodeRef<K, V>),
}

#[derive(Debug)]
struct SkipList<K: Clone + Ord, V> {
    // 0 is the lowest index list, and n is the highest, the less dense list
    heads: Vec<Option<IndexNodeRef<K, V>>>,
    base: Option<ListNodeRef<K, V>>,
}

enum SearchStep<K: Clone + Ord, V> {
    Found(ListNodeRef<K, V>),
    NotFound(K),
    Step(LowerNode<K, V>),
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
    fn prepend(entry: &mut Option<ListNodeRef<K, V>>, key: &K, value: V) {
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
        entry: &mut Option<ListNodeRef<K, V>>,
        key: &K,
        value: V,
    ) -> Option<ListNodeRef<K, V>> {
        if entry.is_none() {
            Self::prepend(entry, key, value);

            return entry.clone();
        }

        let mut prev: Option<ListNodeRef<K, V>> = None;
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
            Some(ref p) => {
                p.borrow_mut().next = Some(new_elem.clone());
            }
            None => {
                entry.replace(new_elem.clone());
            }
        }

        Some(new_elem)
    }

    // Given the position of the previous nodes in each level, we can do
    // the insert.
    fn levelled_insert(
        &mut self,
        level_prevs: Vec<Option<IndexNodeRef<K, V>>>,
        base_prev: Option<ListNodeRef<K, V>>,
        key: &K,
        value: V,
    ) {
        // Insert into the base list
        let base_prev_next = match base_prev {
            None => None,
            Some(ref base) => base.borrow().next.clone(),
        };

        let base_node = Rc::new(RefCell::new(ListNode {
            key: key.clone(),
            value,
            next: base_prev_next,
        }));

        let mut node_to_next_level = LowerNode::Base(base_node);

        for (i, prev) in level_prevs.into_iter().enumerate() {
            // If the list is empty or the choice function gives true, we insert
            if prev.is_none() {
                if Self::choice_to_insert(i as u8 + 1) {
                    match prev {
                        None => {
                            let new_head = Rc::new(RefCell::new(IndexNode {
                                key: key.clone(),
                                height: i as u8,
                                next: None,
                                next_level: node_to_next_level,
                            }));

                            self.heads[i] = Some(new_head.clone());

                            node_to_next_level = LowerNode::Index(new_head);
                        }
                        Some(prev_node) => {
                            let new_head = Rc::new(RefCell::new(IndexNode {
                                key: key.clone(),
                                height: i as u8,
                                next: Some(Rc::clone(&prev_node)),
                                next_level: node_to_next_level,
                            }));

                            prev_node.borrow_mut().next = Some(new_head.clone());

                            node_to_next_level = LowerNode::Index(new_head);
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    pub fn insert(&mut self, key: &K, value: V) -> bool {
        if self.heads.first().unwrap().is_none() {
            // check if it is really what we want
            let prev_heads = self.heads.clone();
            let base_head = self.base.clone();

            Self::levelled_insert(self, prev_heads, base_head, key, value);

            return true;
        }

        let mut level_prevs = vec![None; self.heads.len()];
        let mut base_prev: Option<ListNodeRef<K, V>> = None;
        let mut idx = self.heads.last().unwrap().as_ref().cloned();
        let mut level = self.heads.len();
        let mut level_prev: Option<IndexNodeRef<K, V>> = None;

        loop {
            let idx_rc = match idx {
                None => unreachable!(),
                Some(ref irc) => Rc::clone(irc),
            };

            let compare = idx_rc.borrow().key.cmp(key);

            match compare {
                Ordering::Less => {
                    level_prev = Some(Rc::clone(&idx_rc));
                    idx = idx_rc.borrow().next.as_ref().cloned();
                }
                Ordering::Equal => {
                    return false;
                }
                Ordering::Greater => {
                    if level > 0 {
                        // we are in the index nodes
                        level_prevs[level - 1] = level_prev.clone();
                        level -= 0;
                        level_prev = None;

                        match level_prev {
                            None => todo!(),
                            Some(ref level_prev_ref) => {
                                idx = match level_prev_ref.borrow().next_level {
                                    LowerNode::Index(ref i) => Some(Rc::clone(i)),
                                    LowerNode::Base(_) => unreachable!(),
                                }
                            }
                        }
                    } else {
                        // we are in base node level
                    }
                }
            }
            // if idx node is less than value
            //   set prev to this node
            //   and go on by next
            // if greater, we need to insert before this node
            //   so we need to check if prev is set
            //     if it is not null, good we need to go one level down
            //     if it is null, well can it be? unreachable
        }

        // build the previous nodes Vec after which we might insert the elem
        // in the skipnodes
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

    /// Step one in the skiplist with comparing the key and return with
    /// not found, found or step if there are more steps in the search.
    fn search_step(node: SearchStep<K, V>, key: &K) -> SearchStep<K, V> {
        match node {
            result @ SearchStep::NotFound(_) => result,
            result @ SearchStep::Found(_) => result,
            SearchStep::Step(LowerNode::Index(iref)) => {
                let borrowed = iref.borrow();

                if borrowed.key < *key {
                    match &borrowed.next {
                        None => SearchStep::NotFound(key.clone()),
                        Some(bnext) => SearchStep::Step(LowerNode::Index(Rc::clone(&bnext))),
                    }
                } else {
                    match &borrowed.next_level {
                        LowerNode::Index(iiref) => {
                            SearchStep::Step(LowerNode::Index(Rc::clone(&iiref)))
                        }
                        LowerNode::Base(nref) => {
                            SearchStep::Step(LowerNode::Base(Rc::clone(&nref)))
                        }
                    }
                }
            }
            SearchStep::Step(LowerNode::Base(nref)) => {
                let borrowed = nref.borrow();
                let cmp = borrowed.key.cmp(key);

                match cmp {
                    Ordering::Less => match &borrowed.next {
                        None => SearchStep::NotFound(key.clone()),
                        Some(next_node) => SearchStep::Step(LowerNode::Base(Rc::clone(&next_node))),
                    },
                    Ordering::Equal => SearchStep::Found(nref.clone()),
                    Ordering::Greater => SearchStep::NotFound(key.clone()),
                }
            }
        }
    }

    fn contains_key(&self, key: &K) -> bool {
        let head = self.heads.last().unwrap();

        let mut prev = None;
        let mut p: Option<LowerNode<K, V>> = head.as_ref().cloned().map(|v| LowerNode::Index(v));

        loop {
            match p {
                None => todo!(),
                Some(LowerNode::Index(indexRef)) => {
                    let compare = indexRef.borrow().key.cmp(key);

                    match compare {
                        Ordering::Less => {
                            prev = Some(Rc::clone(&indexRef));
                            p = indexRef
                                .borrow()
                                .next
                                .as_ref()
                                .map(|v| LowerNode::Index(v.clone()));
                        }
                        Ordering::Equal => {
                            return true;
                        }
                        Ordering::Greater => {
                            prev = None;
                            p = match &indexRef.borrow().next_level {
                                LowerNode::Base(nref) => Some(LowerNode::Base(nref.clone())),
                                LowerNode::Index(iref) => Some(LowerNode::Index(iref.clone())),
                            };
                        }
                    }
                }
                Some(LowerNode::Base(ref nodeRef)) => {
                    //let mut nref = nodeRef;

                    //loop {
                    //    if nref.borrow().key < *key {
                    //        nref = match &nref.borrow().next {
                    //            None => {
                    //                return false;
                    //            }
                    //            Some(next_ref) => Rc::clone(&next_ref),
                    //        };
                    //    }
                    //}
                }
            }
        }
    }

    fn print(&self, printer: Box<dyn Fn(&K)>) {
        for i in (0..self.heads.len()).rev() {
            let mut h = self.heads[i].clone();

            print!("({}) ", i);

            while let Some(boxed) = h {
                let br = boxed.borrow();

                printer(&br.key);

                h = br.next.as_ref().cloned();
            }

            println!();
        }

        ListNode::print(&self.base, printer);
    }
}

impl<K: Clone + Ord, V> ListNode<K, V> {
    fn print(head: &Option<ListNodeRef<K, V>>, printer: Box<dyn Fn(&K)>) {
        let mut head = head.clone();

        while let Some(node) = head {
            let b = node.borrow();

            printer(&b.key);
            head = b.next.as_ref().cloned();
        }
    }
}

fn main() {
    let mut list = SkipList::<usize, String>::new(4);

    for i in [10, 8, 12, 9, 15] {
        println!("Inserting {}", i);

        list.insert(&i, format!("{}", i));
        list.print(Box::new(|k| print!("{} ", k)));
    }

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
    use std::{cell::RefCell, cmp::Ordering, rc::Rc};

    use crate::{IndexNode, IndexNodeRef, ListNode, ListNodeRef, LowerNode, SearchStep, SkipList};

    fn lookup_in_base<K: Clone + Ord, V>(
        head: Option<ListNodeRef<K, V>>,
        key: &K,
    ) -> ListNodeRef<K, V> {
        let mut head = head;

        loop {
            let head_rc = match head {
                None => break,
                Some(h) => Rc::clone(&h),
            };

            if head_rc.borrow().key.cmp(key) == Ordering::Equal {
                return head_rc.clone();
            }

            head = head_rc.borrow().next.as_ref().cloned();
        }

        panic!("Cannot find key in the list");
    }

    fn lookup_in_index<K: Clone + Ord, V>(
        head: Option<IndexNodeRef<K, V>>,
        key: &K,
    ) -> IndexNodeRef<K, V> {
        let mut head = head;

        loop {
            let head_rc = match head {
                None => break,
                Some(h) => Rc::clone(&h),
            };

            if head_rc.borrow().key.cmp(key) == Ordering::Equal {
                return head_rc.clone();
            }

            head = head_rc.borrow().next.as_ref().cloned();
        }

        panic!("Cannot find key in the list");
    }

    fn base_from_vec<K: Clone + Ord, V>(values: Vec<(K, V)>) -> Option<ListNodeRef<K, V>> {
        let mut base: Option<ListNodeRef<K, V>> = None;
        let mut prev = None;

        for (k, v) in values {
            let p = ListNode {
                key: k.clone(),
                value: v,
                next: None,
            };

            let p_rc = Rc::new(RefCell::new(p));

            match prev {
                None => {
                    base = Some(p_rc.clone());
                    prev = Some(p_rc);
                }
                Some(ref prev_rc) => {
                    prev_rc.borrow_mut().next = Some(p_rc.clone());
                    prev = Some(p_rc);
                }
            }
        }

        base
    }

    fn indexes_from_vecs<K: Clone + Ord, V>(
        values: Vec<Vec<K>>,
        base: Option<ListNodeRef<K, V>>,
    ) -> Vec<Option<IndexNodeRef<K, V>>> {
        let mut heads = vec![None; values.len()];

        for (level, level_vec) in values.into_iter().enumerate() {
            let mut head = None;
            let mut prev = None;

            for k in level_vec {
                let lower = match level {
                    0 => {
                        // lookup in the base list
                        LowerNode::Base(lookup_in_base(base.as_ref().cloned(), &k))
                    }
                    _ => LowerNode::Index(lookup_in_index(heads[level - 1].as_ref().cloned(), &k)),
                };

                // look the key in either base or index list
                let p = IndexNode {
                    key: k.clone(),
                    next: None,
                    height: level as u8,
                    next_level: lower,
                };

                let p_rc = Rc::new(RefCell::new(p));

                match prev {
                    None => {
                        head = Some(p_rc.clone());
                        prev = Some(p_rc);
                    }
                    Some(ref prev_rc) => {
                        prev_rc.borrow_mut().next = Some(p_rc);
                    }
                }
            }

            heads[level] = head;
        }

        heads
    }

    fn from_vecs<K: Clone + Ord, V>(base: Vec<(K, V)>, indexes: Vec<Vec<K>>) -> SkipList<K, V> {
        let base_list = base_from_vec(base);
        let index_lists = indexes_from_vecs(indexes, base_list.as_ref().cloned());

        SkipList {
            heads: index_lists,
            base: base_list,
        }
    }

    fn to_pair<T: Copy>(values: Vec<T>) -> Vec<(T, T)> {
        let mut result = vec![];

        for v in values {
            result.push((v, v));
        }

        result
    }

    fn pop<K: Clone + Ord, V>(head: &mut Option<ListNodeRef<K, V>>) -> Option<K> {
        match head.take() {
            None => None,
            Some(node) => {
                let k = node.borrow().key.clone();

                match &node.borrow().next {
                    None => {
                        *head = None;
                    }
                    Some(n) => {
                        head.replace(n.clone());
                    }
                }

                Some(k)
            }
        }
    }

    #[test]
    fn base_from_vec_test() {
        let mut list = base_from_vec(to_pair(vec![2, 6, 9, 11, 15]));

        ListNode::print(&list, Box::new(|k| print!("{} ", k)));

        assert_eq!(Some(2), pop(&mut list));
        assert_eq!(Some(6), pop(&mut list));
        assert_eq!(Some(9), pop(&mut list));
        assert_eq!(Some(11), pop(&mut list));
        assert_eq!(Some(15), pop(&mut list));
    }

    #[test]
    fn print() {
        let list = from_vecs(
            to_pair(vec![4, 6, 9, 10, 15, 19, 25]),
            vec![vec![4, 10, 19], vec![4, 19], vec![4]],
        );

        list.print(Box::new(|v| print!("{} ", v)));
    }

    //#[test]
    //fn insert_into_empty_list() {
    //    let mut list = SkipList::<usize, String>::new(4);

    //    list.insert(&5, "5".into());

    //    assert!(list.contains_key(&5));
    //}

    #[test]
    fn search_step_test() {
        let list = from_vecs(
            to_pair(vec![4, 6, 9, 10, 15, 19, 25]),
            vec![vec![4, 10, 19], vec![4, 19], vec![4]],
        );

        let h = Rc::clone(list.heads.last().unwrap().as_ref().unwrap());

        let mut step = SearchStep::Step(LowerNode::Index(h));

        step = SkipList::search_step(step, &15);
        if let SearchStep::Step(LowerNode::Index(iref)) = &step {
            assert_eq!(4, iref.borrow().key);
            assert_eq!(2, iref.borrow().height);
        }

        step = SkipList::search_step(step, &15);
        if let SearchStep::Step(LowerNode::Index(iref)) = &step {
            assert_eq!(10, iref.borrow().key);
            assert_eq!(2, iref.borrow().height);
        }

        step = SkipList::search_step(step, &15);
        if let SearchStep::Step(LowerNode::Index(iref)) = &step {
            assert_eq!(10, iref.borrow().key);
            assert_eq!(1, iref.borrow().height);
        }

        step = SkipList::search_step(step, &15);
        if let SearchStep::Step(LowerNode::Base(nref)) = &step {
            assert_eq!(10, nref.borrow().key);
        }

        step = SkipList::search_step(step, &15);
        if let SearchStep::Found(node_ref) = &step {
            assert_eq!(15, node_ref.borrow().key);
        }
    }

    //#[test]
    //fn insert_first() {
    //    let mut list = SkipList::<usize>::new(4);

    //    list.insert(8);
    //    list.insert(5);

    //    let level0 = list_to_vec(&list.heads[0]);

    //    assert_eq!(vec![5, 8], level0);
    //}
}