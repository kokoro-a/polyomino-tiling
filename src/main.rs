struct DancingLinks {
    root: *mut ColumnNode,
}

impl DancingLinks {
    fn new() -> Self {
        let mut root = Box::new(ColumnNode {
            name: "root".to_string(),
            size: 0,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            head: std::ptr::null_mut(),
        });
        root.left = &mut *root;
        root.right = &mut *root;

        Self {
            root: Box::into_raw(root),
        }
    }

    fn add_column(&mut self, name: String) {
        /*
        Before add_column:
            root -- column1 -- ... -- columnN -- root

        After add_column:
            root -- column1 -- ... -- columnN -- **new_column** --root
        */
        unsafe {
            let last_column_before_add = (*self.root).left;
            let new_column = Box::new(ColumnNode {
                name,
                size: 0,
                left: last_column_before_add,
                right: self.root,
                head: std::ptr::null_mut(),
            });

            let new_column_ptr = Box::into_raw(new_column);
            (*last_column_before_add).right = new_column_ptr;
            (*self.root).left = new_column_ptr;
        }
    }
}

impl Drop for DancingLinks {
    fn drop(&mut self) {
        unsafe {
            let mut current = (*self.root).right;
            while current != self.root {
                let next = (*current).right;
                println!("Dropping column: {}", (*current).name);
                drop(Box::from_raw(current));
                current = next;
            }
            println!("Dropping root");
            drop(Box::from_raw(self.root));
        }
    }
}

struct ColumnNode {
    name: String,
    size: usize,
    left: *mut ColumnNode,
    right: *mut ColumnNode,
    head: *mut Node,
}

struct Node {
    row: usize,
    column: *mut ColumnNode,
    up: *mut Node,
    down: *mut Node,
    left: *mut Node,
    right: *mut Node,
}

fn main() {
    let mut dlx = DancingLinks::new();
    dlx.add_column("Column 1".to_string());
    dlx.add_column("Column 2".to_string());
    // Example usage of the Dancing Links algorithm
    // Add columns and rows to the dlx structure
    // Perform search and cover/uncover operations
    // Print results
    unsafe {
        println!("{:?}", (*dlx.root).name);
        println!("{:?}", (*(*dlx.root).right).name);
        println!("{:?}", (*(*dlx.root).left).name);
    }
}
