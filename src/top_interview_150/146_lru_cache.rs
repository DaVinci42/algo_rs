use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
struct DoubleLinkedList {
    key: i32,
    val: i32,
    pre: Option<Rc<RefCell<DoubleLinkedList>>>,
    next: Option<Rc<RefCell<DoubleLinkedList>>>,
}

impl DoubleLinkedList {
    fn new(key: i32, val: i32) -> Self {
        Self {
            key,
            val,
            pre: None,
            next: None,
        }
    }
}

struct LRUCache {
    map: HashMap<i32, Rc<RefCell<DoubleLinkedList>>>,
    capacoty: usize,
    head: Option<Rc<RefCell<DoubleLinkedList>>>,
    tail: Option<Rc<RefCell<DoubleLinkedList>>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            capacoty: capacity as usize,
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, key: i32, val: i32) {
        if self.capacoty == self.map.len() {
            let k = self.tail.as_ref().unwrap().borrow().key;
            self.del(k);
        }

        let node = Rc::new(RefCell::new(DoubleLinkedList::new(key, val)));
        if let Some(head) = self.head.clone() {
            node.borrow_mut().next = self.head.clone();
            head.borrow_mut().pre = Some(node.clone());
            self.head = Some(node.clone());
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));
        }
        self.map.insert(key, node);
    }

    fn del(&mut self, key: i32) {
        let node = self.map.remove(&key).unwrap();
        let b = &node.borrow();
        let (pre, next) = (&b.pre, &b.next);
        match (pre, next) {
            (Some(_), Some(_)) => {
                pre.as_ref().unwrap().borrow_mut().next = next.clone();
                next.as_ref().unwrap().borrow_mut().pre = pre.clone();
            }
            (Some(_), None) => {
                pre.as_ref().unwrap().borrow_mut().next = None;
                self.tail = pre.clone();
            }
            (None, Some(_)) => {
                next.as_ref().unwrap().borrow_mut().pre = None;
                self.head = next.clone();
            }
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
        }
    }

    fn get(&mut self, k: i32) -> i32 {
        if let Some(v) = self.map.get(&k).map(|n| n.borrow().val) {
            self.del(k);
            self.add(k, v);
            v
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.del(key);
            self.add(key, value);
        } else {
            self.add(key, value);
        }
    }
}
