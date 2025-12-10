#[derive(Debug)]
struct Node {
    data: String,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct Stack {
    vals: Option<Node>,
}

pub fn init() -> Stack {
    return Stack {
        vals: None,
    };
}

pub fn push(val: String, mut s: Stack) -> Stack{
    
    let old_head = s.vals.take();

    
    let new_node = Node {
        data: val,
        next: old_head.map(Box::new),
    };

    s.vals = Some(new_node);
    s
}

pub fn pop(mut s: Stack) -> (Option<String>, Stack) {
    match s.vals.take() {
        None => {
        
            (None, s)
        }
        Some(node) => {
            
            let value = node.data;

            
            s.vals = node.next.map(|boxed| *boxed);

            (Some(value), s)
        }
    }
}

