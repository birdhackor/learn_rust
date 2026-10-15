use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

struct Node {
    id: i32,
    parent: RefCell<Weak<Node>>,
    children: Vec<Rc<Node>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("清理節點id={}", self.id);
    }
}

fn main() {
    let leaf = Rc::new(Node {
        id: 1,
        parent: RefCell::new(Weak::new()),
        children: Vec::new(),
    });
    {
        let branch = Rc::new(Node {
            id: 2,
            parent: RefCell::new(Weak::new()),
            children: vec![Rc::clone(&leaf)],
        });
        {
            let mut parent = leaf.parent.borrow_mut();
            *parent = Rc::downgrade(&branch);
        }
        println!("父節點有子節點={}", branch.children.len());
        {
            let parent = leaf.parent.borrow();
            match parent.upgrade() {
                Some(owner) => println!("子節點找到父id={}", owner.id),
                None => println!("沒有父節點"),
            }
        }
        println!("branch的strong={}", Rc::strong_count(&branch));
        println!("leaf的strong={}", Rc::strong_count(&leaf));
    }
    println!("父結束後leaf的strong={}", Rc::strong_count(&leaf));
    {
        let parent = leaf.parent.borrow();
        match parent.upgrade() {
            Some(owner) => println!("仍有父id={}", owner.id),
            None => println!("子節點的父已結束"),
        }
    }
}
