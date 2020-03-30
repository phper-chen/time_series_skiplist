extern crate rand;
use chrono::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

const L_MAX_LEVEL: usize = 10;
#[derive(Clone, Debug)]
pub struct SkipList {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64,
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
struct Node {
    next: Vec<Link>,
    pub offset: u64,
    pub command: String,
}
impl Node {
    fn new(next: Vec<Link>, offset: u64, command: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            next,
            offset,
            command,
        }))
    }
}
impl SkipList {
    fn new() -> SkipList{
        SkipList{
            head: None,
            tails: vec![None; L_MAX_LEVEL+1],
            max_level: L_MAX_LEVEL,
            length: 0,
        }
    }
    pub fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }

    pub fn append(&mut self, offset: u64, value: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };
        let new = Node::new(vec![None; level], offset, value);
        if self.head.is_none() {
            self.head = Some(new.clone());
        }
        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone());
            }
            self.tails[i] = Some(new.clone());
        }

        self.length += 1;
    }
    pub fn find(&self, offset: u64) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level;
                let node = head.clone();
                let mut result = None;
                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }
                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next)
                            if next.borrow().offset <= offset => n = next.clone(),
                            _ => break
                        };
                    }
                    if n.borrow().offset == offset {
                        let tmp = n.borrow();
                        result = Some(tmp.command.clone());
                        print!("在第{}层找到了命令{}，执行时间为{}\n", level, result.as_ref().unwrap(), Local.timestamp(offset as i64 / 1000, 0));
                        break;
                    }
                }
                result
            }
            None => None,
        }
    }
}

fn main() {
    let mut skl_head = SkipList::new();
    for i in 0..=100000 {
        let now = Local::now().timestamp_millis() as u64 + 1;
        skl_head.append(now, i.to_string() + &"命令".to_owned() );
    }
    let aim = Local::now().timestamp_millis() as u64 + 100;
    skl_head.append(aim, "Find".to_string() + &"命令".to_owned() );

    match skl_head.find(aim) {
        Some(command) => print!("Command is {}", command),
        None => print!("命令不存在")
    };
}