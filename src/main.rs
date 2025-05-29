mod dancing_links;
mod polyomino_tiling;
mod pretty;
use std::{collections::HashMap, hash::Hash};

use polyomino_tiling::{PolyominoTiling, piece_placements_to_matrix_of_piece_ids};

fn main() {
    let t_mino = vec![
        vec![1, 1, 1], //
        vec![0, 1, 0],
    ];
    let l_mino = vec![
        vec![1, 1, 1], //
        vec![1, 0, 0],
    ];

    let polyominoes = vec![t_mino.clone(), l_mino, t_mino];

    let tiling = PolyominoTiling::new(4, 3, polyominoes);
    let solution = tiling.solve();
    let colors: HashMap<usize, (u8, u8, u8)> = HashMap::from([
        (0, (255, 0, 0)), // Red for T-mino
        (1, (0, 255, 0)), // Green for L-mino
        (2, (0, 0, 255)), // Blue for another T-mino
    ]);
    let solution_pretty = make_solution_pretty(&solution, colors);
    println!("Solution:\n{}", solution_pretty);
}

fn make_solution_pretty(
    solution: &Option<Vec<(usize, Vec<Vec<usize>>)>>,
    colors: HashMap<usize, (u8, u8, u8)>,
) -> String {
    if solution.is_none() {
        return "** NO SOLUTION **\n".to_string();
    }

    let solution = solution.as_ref().unwrap();
    let placement_matrix = piece_placements_to_matrix_of_piece_ids(
        solution,
        solution[0].1[0].len(),
        solution[0].1.len(),
    );

    let mut s = String::new();
    for (row, row_data) in placement_matrix.iter().enumerate() {
        for (col, &piece_id) in row_data.iter().enumerate() {
            let _s: String = if piece_id.is_none() {
                color_str(".", 100, 100, 100)
            } else if let Some(&(r, g, b)) = colors.get(&piece_id.unwrap()) {
                color_str("#", r, g, b)
            } else {
                color_str("#", 255, 255, 255) // Unknown piece
            };
            s.push_str(&_s);
        }
        s.push('\n');
    }
    s.push('\n');
    s
}

fn color_str(text: &str, r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
}
