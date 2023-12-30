use std::{cmp::Ordering, fmt::Debug, rc::Rc};

use crate::node::{ListNodeRef, LowerNode};

pub enum SearchStep<K: Clone + Debug + Ord, V> {
    Found(ListNodeRef<K, V>),
    NotFound(K),
    InProgress,
}

#[derive(Debug)]
pub struct SearchIter<K: Clone + Debug + Ord, V> {
    pub key: K,
    pub prev: Option<LowerNode<K, V>>,
    pub current: Option<LowerNode<K, V>>,
}

impl<K: Clone + Debug + Ord, V> SearchIter<K, V> {
    /// Step one in the skiplist with comparing the key and return with
    /// not found, found or step if there are more steps in the search.
    pub fn search_step(&mut self) -> SearchStep<K, V> {
        if self.current.is_none() {
            return SearchStep::NotFound(self.key.clone());
        }

        let current_rc = match &self.current {
            None => unreachable!(),
            Some(ln) => ln.clone(),
        };

        match current_rc {
            LowerNode::Index(iref) => {
                self.prev = Some(LowerNode::Index(Rc::clone(&iref)));

                let borrowed = iref.borrow();
                let cmp = borrowed.key.cmp(&self.key);

                match cmp {
                    Ordering::Less => {
                        match &borrowed.next {
                            None => {
                                self.prev = None;
                                self.current = Some(iref.borrow().next_level.clone());
                            }
                            Some(next_ref) => match next_ref.borrow().key.cmp(&self.key) {
                                Ordering::Less => {
                                    self.prev = Some(LowerNode::Index(Rc::clone(&iref)));
                                    self.current = Some(LowerNode::Index(Rc::clone(next_ref)));
                                }
                                Ordering::Equal | Ordering::Greater => {
                                    self.prev = None;
                                    self.current = Some(iref.borrow().next_level.clone());
                                }
                            },
                        }
                        SearchStep::InProgress
                    }
                    Ordering::Equal => {
                        self.prev = None;
                        self.current = Some(borrowed.next_level.clone());
                        SearchStep::InProgress
                    }
                    // We check if next.key is greater than the pattern
                    Ordering::Greater => match self.prev.take() {
                        None => {
                            self.current = None;
                            SearchStep::NotFound(self.key.clone())
                        }
                        Some(LowerNode::Index(iiref)) => {
                            self.current = Some(iiref.borrow().next_level.clone());
                            SearchStep::InProgress
                        }
                        Some(LowerNode::Base(_)) => {
                            self.current = None;
                            SearchStep::NotFound(self.key.clone())
                        }
                    },
                }
            }
            LowerNode::Base(nref) => {
                self.prev = Some(LowerNode::Base(Rc::clone(&nref)));

                let borrowed = nref.borrow();
                let cmp = borrowed.key.cmp(&self.key);

                match cmp {
                    Ordering::Less => {
                        self.current = borrowed.next.as_ref().cloned().map(|v| LowerNode::Base(v));
                        SearchStep::InProgress
                    }
                    Ordering::Equal => {
                        self.current = None;
                        SearchStep::Found(Rc::clone(&nref))
                    }
                    Ordering::Greater => {
                        self.current = None;
                        SearchStep::NotFound(self.key.clone())
                    }
                }
            }
        }
    }
}
