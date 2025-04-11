struct Node {
    left: *mut Node,
    right: *mut Node,
    up: *mut Node,
    down: *mut Node,
    column: *mut ColumnNode,
}

struct ColumnNode {
    left: *mut ColumnNode,
    right: *mut ColumnNode,
    head: *mut Node,
    size: usize,
}

struct DancingLinks {
    root: *mut ColumnNode,
}

unsafe fn cover(column: *mut ColumnNode) {
    (*(*column).left).right = (*column).right;
    (*(*column).right).left = (*column).left;

    let mut x = (*column).head;

    if x.is_null() {
        return;
    }

    loop {
        let mut y = (*x).right;
        while y != x {
            (*(*y).up).down = (*y).down;
            (*(*y).down).up = (*y).up;
            (*(*y).column).size -= 1;
            y = (*y).right;
        }
        x = (*x).down;
        if x == (*column).head {
            break;
        }
    }
}

unsafe fn uncover(column: *mut ColumnNode) {
    let mut x = (*column).head;

    if x.is_null() {
        return;
    }

    loop {
        let mut y = (*x).left;
        while y != x {
            (*(*y).up).down = y;
            (*(*y).down).up = y;
            (*(*y).column).size += 1;
            y = (*y).left;
        }
        x = (*x).up;
        if x == (*column).head {
            break;
        }
    }

    (*(*column).left).right = column;
    (*(*column).right).left = column;
}

use std::ptr::null_mut;

unsafe fn add_row(columns: &[*mut ColumnNode], row: &[u8]) {
    let nodes = row
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| {
            if v == 1 {
                let node = Box::into_raw(Box::new(Node {
                    left: null_mut(),
                    right: null_mut(),
                    up: null_mut(),
                    down: null_mut(),
                    column: columns[i],
                }));
                Some(node)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    /* Make horizontal rink */
    for i in 0..nodes.len() {
        let left = nodes[(i - 1) % nodes.len()];
        let this = nodes[i];
        let right = nodes[(i + 1) % nodes.len()];

        (*this).left = left;
        (*this).right = right;
    }

    /* Make vertical rink */
    for &n in nodes.iter() {
        let column = (*n).column;
        let head = (*column).head;
        if head.is_null() {
            (*column).head = n;
            (*n).up = n;
            (*n).down = n;
        } else {
            let last = (*head).up;
            (*head).up = n;
            (*n).down = head;
            (*n).up = last;
            (*last).down = n;
        }
        (*column).size += 1;
    }
}

unsafe fn new_dancing_links(n_columns: u8) -> *mut ColumnNode {
    let nodes = (0..(n_columns + 1))
        .map(|_| {
            let node = Box::into_raw(Box::new(ColumnNode {
                left: null_mut(),
                right: null_mut(),
                head: null_mut(),
                size: 0,
            }));
            node
        })
        .collect::<Vec<_>>();

    for i in 0..nodes.len() {
        let left = nodes[(i - 1) % nodes.len()];
        let this = nodes[i];
        let right = nodes[(i + 1) % nodes.len()];
        (*this).left = left;
        (*this).right = right;
    }

    nodes[0]
}

fn get_columns(root: *mut ColumnNode) -> Vec<*mut ColumnNode> {
    let mut columns = Vec::new();
    let mut current = unsafe { (*root).right };
    while current != root {
        columns.push(current);
        current = unsafe { (*current).right };
    }
    columns
}

fn main() {
    let root = unsafe { new_dancing_links(3) };
    let columns = get_columns(root);
    unsafe {
        add_row(&columns, &[1, 0, 1]);
        add_row(&columns, &[0, 1, 1]);
        add_row(&columns, &[1, 1, 0]);
        add_row(&columns, &[0, 0, 1]);
    }
}

fn solve(root: *mut ColumnNode) {
    if unsafe { (*root).right } == root {
        return;
    }

    let columns = get_columns(root);
    let &column_min_size = columns
        .iter()
        .min_by_key(|&&col| unsafe { (*col).size })
        .unwrap();

    unsafe { cover(column_min_size) };
}
