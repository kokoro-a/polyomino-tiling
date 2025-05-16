use core::panic;
use std::iter;

struct DancingLinks {
    root: *mut ColumnNode,
    n_rows: usize,
    n_cols: usize,
    columns: Vec<*mut ColumnNode>,
}

impl DancingLinks {
    fn new() -> Self {
        let mut root: Box<ColumnNode> = Box::new(ColumnNode::new(0));
        root.left = &mut *root;
        root.right = &mut *root;

        Self {
            root: Box::into_raw(root),
            n_rows: 0,
            n_cols: 0,
            columns: Vec::new(),
        }
    }

    fn solve(&mut self, solution: Vec<usize>) -> Option<Vec<usize>> {
        let columns_sorted = {
            let cs = self.columns.clone();
            cs.sort_by_key(|&col| unsafe { (*col).size });
            cs
        };
        columns_sorted.iter().for_each(|&col| unsafe {
            // Cover the column
            (*col).cover();
            (*col).iterate_column().for_each(|node| {
                let solution_candidate = (*node).row_index;
                let mut current = (*node).right;
                iter::from_fn(move || {
                    if current == node {
                        return None;
                    } else {
                        let next = (*current).right;
                        return Some(current);
                        current = next;
                    }
                })
                .for_each(|node| unsafe { (*node).unlink_vertically() });
            })
        });
    }

    fn append_column(&mut self) {
        unsafe {
            let old_rightmost = (*self.root).left;
            let new_column: Box<ColumnNode> = Box::new(ColumnNode::new(self.n_cols));
            let new_column_ptr: *mut ColumnNode = Box::into_raw(new_column);
            (*old_rightmost).insert_right(new_column_ptr);
            self.n_cols += 1;
            self.columns.push(new_column_ptr);
        }
    }

    fn append_row(&mut self, row: Vec<isize>) {
        if row.len() != self.n_cols as usize {
            panic!("Row length does not match number of columns");
        }

        let row_nodes = row
            .iter()
            .enumerate()
            .filter(|&(_, &val)| val != 0)
            .map(|(i, _)| {
                let column = self.columns[i];
                let new_node: Box<Node> = Box::new(Node::new(self.n_rows, column));
                let new_node_ptr = Box::into_raw(new_node);
                unsafe { (*column).append_node(new_node_ptr) };
                new_node_ptr
            })
            .collect::<Vec<_>>();

        // Link the nodes in the row
        if row_nodes.len() > 0 {
            let first_node = row_nodes[0];
            unsafe {
                (*first_node).left = first_node;
                (*first_node).right = first_node;
            }
            row_nodes.iter().skip(1).for_each(|&node| unsafe {
                (*first_node).insert_left(node);
            });
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
                println!("Dropping column: {}", (*current).index);
                self.drop_column(current);
                current = next;
            }
            println!("Dropping root");
            drop(Box::from_raw(self.root));
        }
    }
}

struct ColumnNode {
    index: usize,
    size: usize,
    left: *mut ColumnNode,
    right: *mut ColumnNode,
    head: *mut Node,
}

impl ColumnNode {
    fn new(index: usize) -> Self {
        Self {
            index,
            size: 0,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            head: std::ptr::null_mut(),
        }
    }

    fn insert_right(&mut self, new_column: *mut ColumnNode) {
        unsafe {
            if new_column.is_null() {
                panic!("New column is null");
            }
            let old_right = (*self).right;
            if old_right.is_null() {
                panic!("Right column is null");
            }
            (*self).right = new_column;
            (*new_column).left = self;
            (*new_column).right = old_right;
            (*old_right).left = new_column;
        }
    }

    fn append_node(&mut self, new_node: *mut Node) {
        unsafe {
            if (*self).head.is_null() {
                (*self).head = new_node;
                (*new_node).up = new_node;
                (*new_node).down = new_node;
            } else {
                let head = (*self).head;
                if head.is_null() {
                    panic!("Head is null");
                }
                (*head).insert_up(new_node);
            }
            (*self).size += 1;
        }
    }

    fn iterate_column(&self) -> impl Iterator<Item = *mut Node> {
        unsafe {
            let mut current = (*self).head;
            iter::from_fn(move || {
                if current.is_null() {
                    None
                } else {
                    let node = current;
                    current = (*current).down;
                    Some(node)
                }
            })
        }
    }

    fn cover(&mut self) {
        self.unlink();
        self.iterate_column().for_each(|node| unsafe {
            (*node).unlink_horizontally();

            let mut current = node;
            iter::from_fn(move || {
                current = (*current).left;
                if current == node { None } else { Some(current) }
            })
            .for_each(|node| {
                (*node).unlink_vertically();
                (*(*node).column).size -= 1;
            });
        });
    }

    fn unlink(&mut self) {
        unsafe {
            let left = (*self).left;
            let right = (*self).right;
            (*left).right = right;
            (*right).left = left;
        }
    }

    fn relink(&mut self) {
        unsafe {
            let left = (*self).left;
            let right = (*self).right;
            (*right).left = self;
            (*left).right = self;
        }
    }
}

struct Node {
    row_index: usize,
    column: *mut ColumnNode,
    up: *mut Node,
    down: *mut Node,
    left: *mut Node,
    right: *mut Node,
}

impl Node {
    fn new(row_index: usize, column: *mut ColumnNode) -> Self {
        Self {
            row_index,
            column,
            up: std::ptr::null_mut(),
            down: std::ptr::null_mut(),
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
        }
    }
    fn insert_down(&mut self, new_node: *mut Node) {
        unsafe {
            if new_node.is_null() {
                panic!("New node is null");
            }
            let old_down = (*self).down;
            if old_down.is_null() {
                panic!("Down node is null");
            }
            (*self).down = new_node;
            (*new_node).up = self;
            (*new_node).down = old_down;
            (*old_down).up = new_node;
        }
    }

    fn insert_right(&mut self, new_node: *mut Node) {
        unsafe {
            if new_node.is_null() {
                panic!("New node is null");
            }
            let old_right = (*self).right;
            if old_right.is_null() {
                panic!("Right node is null");
            }
            (*self).right = new_node;
            (*new_node).left = self;
            (*new_node).right = old_right;
            (*old_right).left = new_node;
        }
    }

    fn insert_up(&mut self, new_node: *mut Node) {
        unsafe {
            (*self.up).insert_down(new_node);
        }
    }

    fn insert_left(&mut self, new_node: *mut Node) {
        unsafe {
            (*self.left).insert_right(new_node);
        }
    }

    fn unlink_vertically(&mut self) {
        unsafe {
            let up = (*self).up;
            let down = (*self).down;
            (*up).down = down;
            (*down).up = up;
            (*(*self).column).size -= 1;
        }
    }

    fn unlink_horizontally(&mut self) {
        unsafe {
            let left = (*self).left;
            let right = (*self).right;
            (*left).right = right;
            (*right).left = left;
        }
    }

    fn relink_vertically(&mut self) {
        unsafe {
            let up = (*self).up;
            let down = (*self).down;
            (*up).down = self;
            (*down).up = self;
            (*(*self).column).size += 1;
        }
    }

    fn relink_horizontally(&mut self) {
        unsafe {
            let left = (*self).left;
            let right = (*self).right;
            (*left).right = self;
            (*right).left = self;
        }
    }
}

fn main() {
    let mut dlx = DancingLinks::new();
    dlx.append_column();
    dlx.append_column();
    dlx.append_row(vec![1, 0]);
    dlx.append_row(vec![0, 1]);
    // Example usage of the Dancing Links algorithm
    // Add columns and rows to the dlx structure
    // Perform search and cover/uncover operations
    // Print results
    unsafe {
        println!("{:?}", (*dlx.root).index);
        println!("{:?}", (*(*dlx.root).right).index);
        println!("{:?}", (*(*dlx.root).left).index);
    }
}
