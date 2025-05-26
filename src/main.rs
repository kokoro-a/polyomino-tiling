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
        // uncovered columns
        let columns = {
            let mut c = self.iterate_columns().collect::<Vec<_>>();
            c.sort_by_key(|&col| unsafe { (*col).size });
            c
        };

        // if there are no uncovered columns, we have a solution
        if columns.is_empty() {
            return Some(solution);
        }

        let c = columns[0];

        // if there exists a uncovered column with size 0, it means no solution
        if unsafe { (*c).size } == 0 {
            return None;
        }

        unsafe {
            (*c).cover();
        }
        
        // Get snapshot of nodes before cover operations might affect them
        let nodes_in_column: Vec<*mut Node> = {
            let mut nodes = Vec::new();
            if unsafe { !(*c).head.is_null() } {
                let head = unsafe { (*c).head };
                let mut current = head;
                loop {
                    nodes.push(current);
                    current = unsafe { (*current).down };
                    if current == head {
                        break;
                    }
                }
            }
            nodes
        };
        
        for node_in_column in nodes_in_column {
            let solution_candidate = unsafe { (*node_in_column).row_index };

            // Cover all other columns in this row
            let mut covered_columns = Vec::new();
            let mut node_in_row = unsafe { (*node_in_column).right };
            while node_in_row != node_in_column {
                unsafe {
                    let col = (*node_in_row).column;
                    (*col).cover();
                    covered_columns.push(col);
                    node_in_row = (*node_in_row).right;
                }
            }
            
            if let Some(s) = self.solve({
                let mut s = solution.clone();
                s.push(solution_candidate);
                s
            }) {
                return Some(s);
            }
            
            // Backtrack: uncover columns in reverse order
            for &col in covered_columns.iter().rev() {
                unsafe {
                    (*col).uncover();
                }
            }
        }
        
        unsafe {
            (*c).uncover();
        }
        
        None
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
        
        self.n_rows += 1;
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

    fn iterate_columns(&self) -> impl Iterator<Item = *mut ColumnNode> {
        unsafe {
            let mut current = (*self.root).right;
            iter::from_fn(move || {
                if current == self.root {
                    None
                } else {
                    let column = current;
                    current = (*current).right;
                    Some(column)
                }
            })
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

    fn iterate_nodes_in_column(&self) -> impl Iterator<Item = *mut Node> {
        unsafe {
            let head = (*self).head;
            let mut current = head;
            let mut first = true;
            iter::from_fn(move || {
                if current.is_null() || (!first && current == head) {
                    None
                } else {
                    let node = current;
                    current = (*current).down;
                    first = false;
                    Some(node)
                }
            })
        }
    }

    fn cover(&mut self) {
        self.unlink();
        let nodes: Vec<*mut Node> = self.iterate_nodes_in_column().collect();
        for node in nodes {
            unsafe {
                let mut current = (*node).right;
                while current != node {
                    (*current).unlink_vertically();
                    current = (*current).right;
                }
            }
        }
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

    fn uncover(&mut self) {
        let nodes: Vec<*mut Node> = self.iterate_nodes_in_column().collect();
        for node in nodes.iter().rev() {
            unsafe {
                let mut current = (**node).left;
                while current != *node {
                    (*current).relink_vertically();
                    current = (*current).left;
                }
            }
        }
        self.relink();
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
    // Test 1: Simple case
    let mut dlx = DancingLinks::new();
    dlx.append_column();
    dlx.append_column();
    dlx.append_row(vec![1, 0]);
    dlx.append_row(vec![0, 1]);
    
    println!("Test 1 - Simple case:");
    match dlx.solve(vec![]) {
        Some(solution) => println!("Solution: {:?}", solution),
        None => println!("No solution"),
    }
    
    // Test 2: Another simple case
    let mut dlx2 = DancingLinks::new();
    dlx2.append_column();
    dlx2.append_column(); 
    dlx2.append_row(vec![1, 1]);
    
    println!("Test 2 - Single row covers all:");
    match dlx2.solve(vec![]) {
        Some(solution) => println!("Solution: {:?}", solution),
        None => println!("No solution"),
    }
}
