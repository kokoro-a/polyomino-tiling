use core::panic;
use log::{debug, error, info, trace, warn};
use std::iter;

pub struct DancingLinks {
    root: *mut ColumnNode,
    n_rows: usize,
    n_cols: usize,
    columns: Vec<*mut ColumnNode>,
    nodes: Vec<*mut Node>,
}

impl DancingLinks {
    pub fn new() -> Self {
        let mut root: Box<ColumnNode> = Box::new(ColumnNode::new(0));
        root.left = &mut *root;
        root.right = &mut *root;

        Self {
            root: Box::into_raw(root),
            n_rows: 0,
            n_cols: 0,
            columns: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn from_vecs(matrix: &Vec<Vec<usize>>, n_cols: usize) -> Self {
        if matrix.iter().any(|row| row.len() != n_cols) {
            panic!(
                "number of columns in each row must match. n_cols: {}",
                n_cols
            );
        }

        let mut dl = DancingLinks::new();
        for _ in 0..n_cols {
            dl.append_column();
        }

        for row in matrix {
            dl.append_row(row);
        }
        dl
    }

    pub fn solve(&mut self) -> Option<Vec<usize>> {
        self.solve_helper(vec![])
    }

    fn solve_helper(&mut self, mut solution: Vec<usize>) -> Option<Vec<usize>> {
        info!("current candidate solution: {:?}", solution);
        unsafe {
            // 1. If no columns are left, already found a solution
            if (*self.root).right == self.root {
                return Some(solution);
            }

            // 2. Choose a column with the least number of 1s
            let mut best_column = (*self.root).right;
            let mut min_size = (*best_column).size;
            let mut current = (*best_column).right;

            while current != self.root {
                if (*current).size < min_size {
                    min_size = (*current).size;
                    best_column = current;
                }
                current = (*current).right;
            }

            debug!(
                "best column chosen: col={}, size={}",
                (*best_column).index,
                (*best_column).size
            );

            // 2. If there is columns with no 1s, no solution
            if (*best_column).size == 0 {
                return None;
            }

            // 4. Cover the column
            debug!("covering column {}", (*best_column).index);
            (*best_column).cover();

            // 5. For each row in the column:
            let head = (*best_column).head;
            if !head.is_null() {
                let mut row_node = head;
                loop {
                    // 5.1. Add the row index to the solution
                    solution.push((*row_node).row_index);
                    debug!(
                        "row {} selected as part of the solution candidate",
                        (*row_node).row_index
                    );
                    debug!("current candidate solution: {:?}", solution);

                    // 5.2. Cover all columns that the row intersects with
                    debug!(
                        "covering columns that row {} intersects with",
                        (*row_node).row_index
                    );
                    let mut col_node = (*row_node).right;
                    while col_node != row_node {
                        debug!("covering column {}", (*(*col_node).column).index,);
                        (*(*col_node).column).cover();
                        col_node = (*col_node).right;
                    }
                    debug!("all columns covered for row {}", (*row_node).row_index);

                    // 5.4. Recursively call solve_helper
                    // 5.4.1. If a solution is found, return it
                    if let Some(result) = self.solve_helper(solution.clone()) {
                        return Some(result);
                    }

                    // 5.4.2. If no solution, uncover the columns and try next row
                    col_node = (*row_node).left;
                    while col_node != row_node {
                        (*(*col_node).column).uncover();
                        col_node = (*col_node).left;
                    }

                    solution.pop();

                    row_node = (*row_node).down;
                    if row_node == head {
                        break;
                    }
                }
            }

            // 6. When all rows are tried and yet no solution, uncover the column and return None
            (*best_column).uncover();
            None
        }
    }

    pub fn append_column(&mut self) {
        unsafe {
            let old_rightmost = (*self.root).left;
            let new_column: Box<ColumnNode> = Box::new(ColumnNode::new(self.n_cols));
            let new_column_ptr: *mut ColumnNode = Box::into_raw(new_column);
            (*old_rightmost).insert_right(new_column_ptr);
            self.n_cols += 1;
            self.columns.push(new_column_ptr);
        }
    }

    pub fn append_row(&mut self, row: &Vec<usize>) {
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

        // Push the row nodes to the main list
        self.nodes.extend(row_nodes);

        self.n_rows += 1;
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

    pub fn to_vecs(&self) -> Vec<Vec<usize>> {
        let mut matrix = vec![vec![0; self.n_cols]; self.n_rows];

        unsafe {
            // Iterate through each column
            let mut current_col = (*self.root).right;
            while current_col != self.root {
                let col_index = (*current_col).index;

                // Iterate through all nodes in this column
                let head = (*current_col).head;
                if !head.is_null() {
                    let mut current_node = head;
                    loop {
                        let row_index = (*current_node).row_index;
                        matrix[row_index][col_index] = 1;

                        current_node = (*current_node).down;
                        if current_node == head {
                            break;
                        }
                    }
                }

                current_col = (*current_col).right;
            }
        }

        matrix
    }
}

impl Drop for DancingLinks {
    fn drop(&mut self) {
        debug!("Dropping DancingLinks using vectors");

        // Drop all nodes using the vector (avoid traversing corrupted linked lists)
        debug!("Dropping {} nodes", self.nodes.len());
        for &node in &self.nodes {
            unsafe {
                debug!("Dropping node at row {}", (*node).row_index);
                drop(Box::from_raw(node));
            }
        }

        // Drop all columns using the vector
        debug!("Dropping {} columns", self.columns.len());
        for &column in &self.columns {
            unsafe {
                debug!("Dropping column {}", (*column).index);
                drop(Box::from_raw(column));
            }
        }

        // Drop root
        debug!("Dropping root");
        unsafe {
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
            let mut count = 0;
            iter::from_fn(move || {
                count += 1;
                if count > 10 {
                    println!("iterate_nodes_in_column: infinite loop detected!");
                    return None;
                }
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
        unsafe {
            // 1. Unlink the column node from the neighboring columns
            self.unlink();

            // 2. For each node in this column
            let mut current = (*self).head;
            if !current.is_null() {
                debug!(
                    "head is not null, starting to cover column {}",
                    (*self).index
                );
                loop {
                    // 3. For each node in the same row as current
                    debug!("unlinking row {}", (*current).row_index);
                    let mut row_node = (*current).right;
                    while row_node != current {
                        // Unlink the node vertically from its column
                        debug!("unlinking node vertically: {:?}", (*row_node).get_loc());
                        (*row_node).unlink_vertically();
                        row_node = (*row_node).right;
                        debug!("next to unlink vertically: {:?}", (*row_node).get_loc());
                    }
                    debug!("completed unlinking row {}", (*current).row_index);
                    current = (*current).down;
                    if current == (*self).head {
                        break;
                    }
                    debug!("next row to unlink: {}", (*current).row_index);
                }
                debug!("finished covering column {}", (*self).index);
            } else {
                debug!("head is null, nothing to cover in column {}", (*self).index);
            }
        }
    }

    fn uncover(&mut self) {
        unsafe {
            // Do everything in reverse order of cover
            let mut current = (*self).head;
            if !current.is_null() {
                // Go backwards through the column
                current = (*current).up;
                loop {
                    // For each node in the same row as current (in reverse)
                    let mut row_node = (*current).left;
                    while row_node != current {
                        // Relink the node vertically to its column
                        (*row_node).relink_vertically();
                        row_node = (*row_node).left;
                    }
                    if current == (*self).head {
                        break;
                    }
                    current = (*current).up;
                }
            }

            // Relink the column node to neighboring columns
            self.relink();
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
            let column = (*self).column;
            (*up).down = down;
            (*down).up = up;

            // If I am the head now, make down node the new head
            // but down node is me, i.e. I am the only node in this column, make head null
            if (*column).head == self {
                if down == self {
                    (*column).head = std::ptr::null_mut();
                } else {
                    (*column).head = down;
                }
            }
            if (*column).size == 0 {
                panic!("Attempting to unlink from empty column {}", (*column).index);
            }
            (*column).size -= 1;
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
            if (*(*self).column).head.is_null() {
                (*(*self).column).head = self;
            }
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

    fn get_loc(&self) -> (usize, Option<usize>) {
        unsafe {
            let column_index = if self.column.is_null() {
                None
            } else {
                Some((*self.column).index)
            };
            let row_index = self.row_index;
            (row_index, column_index)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dancing_links_simple() {
        let _ = env_logger::try_init();
        let mut dlx = DancingLinks::from_vecs(&vec![vec![1, 0], vec![0, 1]], 2);
        assert_eq!(dlx.n_rows, 2);
        assert_eq!(dlx.n_cols, 2);
        assert_eq!(dlx.to_vecs(), vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(dlx.solve(), Some(vec![0, 1]));
    }

    #[test]
    fn test_dancing_links_complex() {
        let _ = env_logger::try_init();
        let mut dlx = DancingLinks::from_vecs(
            &vec![
                vec![1, 0, 0, 1, 0, 0, 1],
                vec![1, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 0, 1],
                vec![0, 0, 1, 0, 1, 1, 0],
                vec![0, 1, 0, 0, 0, 1, 1],
                vec![0, 1, 0, 0, 0, 0, 1],
            ],
            7,
        );

        assert_eq!(dlx.n_rows, 6);
        assert_eq!(dlx.n_cols, 7);

        let solution = dlx.solve();
        assert!(solution.is_some());
        let mut sol = solution.unwrap();
        sol.sort(); // Sort for consistent comparison
        assert_eq!(sol, vec![1, 3, 5]);
    }

    #[test]
    fn test_dancing_links_no_solution() {
        let _ = env_logger::try_init();
        let mut dlx = DancingLinks::from_vecs(&vec![vec![1, 0], vec![1, 0]], 2);

        assert_eq!(dlx.solve(), None);
    }

    #[test]
    fn test_dancing_links_single_row() {
        // Matrix with single row covering all columns:
        // [1, 1, 1]  <- row 0
        let _ = env_logger::try_init();
        let mut dlx = DancingLinks::from_vecs(&vec![vec![1, 1, 1]], 3);

        assert_eq!(dlx.solve(), Some(vec![0]));
    }
}
