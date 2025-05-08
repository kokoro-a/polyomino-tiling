struct DancingLinks {
    root: *mut ColumnNode,
    n_rows: isize,
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
            n_rows: 0,
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

    fn add_row(&mut self, row: Vec<isize>) {
        let row_number: isize = self.n_rows + 1;
        self.n_rows = row_number;
        let mut current_column = unsafe { (*self.root).right };
        let mut previous_node: *mut Node = std::ptr::null_mut();
        for &x in row.iter() {
            if current_column == self.root {
                break;
            }
            if x == 1 {
                let new_node: Box<Node> = Box::new(Node {
                    row: row_number, // Placeholder for row number
                    column: current_column,
                    up: std::ptr::null_mut(),
                    down: std::ptr::null_mut(),
                    left: std::ptr::null_mut(),
                    right: std::ptr::null_mut(),
                });
                let new_node_ptr = Box::into_raw(new_node);
                unsafe {
                    (*current_column).size += 1;

                    /* link vertical */
                    let head_node = (*current_column).head;
                    if head_node.is_null() {
                        (*current_column).head = new_node_ptr;
                        (*new_node_ptr).up = new_node_ptr;
                        (*new_node_ptr).down = new_node_ptr;
                    } else {
                        let last_node_before_add = (*head_node).up;
                        (*last_node_before_add).down = new_node_ptr;
                        (*new_node_ptr).up = last_node_before_add;
                        (*new_node_ptr).down = head_node;
                        (*head_node).up = new_node_ptr;
                    }

                    /* link horizontal */
                    (*new_node_ptr).left = previous_node;
                    if !previous_node.is_null() {
                        (*new_node_ptr).left = previous_node;
                        (*previous_node).right = new_node_ptr;
                    }
                }
                previous_node = new_node_ptr;
            }
            current_column = unsafe { (*current_column).right };
        }
        let rightmost_node = previous_node;
        let leftmost_node = {
            let mut current_node = rightmost_node;
            unsafe {
                while !(*current_node).left.is_null() {
                    current_node = (*current_node).left;
                }
            }
            current_node
        };
        unsafe {
            (*leftmost_node).left = rightmost_node;
            (*rightmost_node).right = leftmost_node;
        }
    }

    fn drop_column(&mut self, column: *mut ColumnNode) {
        unsafe {
            if !(*column).head.is_null() {
                let head = (*column).head;
                let mut current = (*head).down;
                while current != head {
                    let next = (*current).down;
                    drop(Box::from_raw(current));
                    current = next;
                }
                drop(Box::from_raw(head));
            }
            drop(Box::from_raw(column));
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
                self.drop_column(current);
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
    row: isize,
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
    dlx.add_row(vec![1, 0]);
    dlx.add_row(vec![0, 1]);
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
