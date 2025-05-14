struct Node<T> {
    value: T,
    up: *mut Node<T>,
    down: *mut Node<T>,
    left: *mut Node<T>,
    right: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        let node = Node {
            value,
            up: std::ptr::null_mut(),
            down: std::ptr::null_mut(),
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
        };
        let node_ptr = Box::into_raw(Box::new(node));
        unsafe {
            (*node_ptr).up = node_ptr;
            (*node_ptr).down = node_ptr;
            (*node_ptr).left = node_ptr;
            (*node_ptr).right = node_ptr;
        }
        node
    }
}

struct Linked2DList<T> {
    top_left: *mut Node<T>,
}

impl Linked2DList<T> {
    fn new() -> Self {
        Linked2DList {
            top_left: std::ptr::null_mut(),
        }
    }

    fn insert(&mut self, value: T) {
        let new_node = Box::into_raw(Box::new(Node::new(value)));
        if self.top_left.is_null() {
            self.top_left = new_node;
            return;
        }
    }
}
