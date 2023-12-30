use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::SkipList;

/// Type of the base list
#[derive(Debug)]
pub struct ListNode<K: Clone + Debug + Ord, V> {
    pub key: K,
    pub value: V,
    pub next: Option<ListNodeRef<K, V>>,
}

pub type ListNodeRef<K, V> = Rc<RefCell<ListNode<K, V>>>;

#[derive(Debug)]
pub struct IndexNode<K: Clone + Debug + Ord, V> {
    pub key: K,
    /// 0 height is the first level of the skiplist, 1 is the next and
    /// n - 2 is the last if n is the number of levels.
    pub height: u8,
    pub next: Option<IndexNodeRef<K, V>>,
    pub next_level: LowerNode<K, V>,
}

pub type IndexNodeRef<K, V> = Rc<RefCell<IndexNode<K, V>>>;

#[derive(Debug)]
pub enum LowerNode<K: Clone + Debug + Ord, V> {
    Index(IndexNodeRef<K, V>),
    Base(ListNodeRef<K, V>),
}

impl<K: Clone + Debug + Ord, V> SkipList<K, V> {
    pub fn new(height: u8) -> Self {
        SkipList {
            heads: (0..(height - 1)).map(|_| None).collect(),
            base: None,
        }
    }
}

impl<K: Clone + Debug + Ord, V> Clone for LowerNode<K, V> {
    fn clone(&self) -> Self {
        match self {
            Self::Index(iref) => Self::Index(iref.clone()),
            Self::Base(nref) => Self::Base(nref.clone()),
        }
    }
}

impl<K: Clone + Debug + Ord, V> ListNode<K, V> {
    pub fn print(head: &Option<ListNodeRef<K, V>>) {
        let mut head = head.clone();

        while let Some(node) = head {
            let b = node.borrow();

            print!("{:?} ", &b.key);
            head = b.next.as_ref().cloned();
        }

        println!();
    }
}

impl<K: Clone + Debug + Ord, V> ListNode<K, V> {
    pub fn clone_key(&self) -> K {
        self.key.clone()
    }
}

impl<K: Clone + Debug + Ord, V> IndexNode<K, V> {
    pub fn clone_key(&self) -> K {
        self.key.clone()
    }
}

impl<K: Clone + Debug + Ord, V> LowerNode<K, V> {
    pub fn clone_key(&self) -> K {
        match self {
            LowerNode::Base(nref) => nref.borrow().key.clone(),
            LowerNode::Index(iref) => iref.borrow().key.clone(),
        }
    }
}
