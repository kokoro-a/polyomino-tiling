mod dancing_links;
mod polyomino_tiling;
mod pretty;
use env_logger;
use polyomino_tiling::{PolyominoTiling, piece_placements_to_matrix_of_piece_ids};
use pretty::str_to_matrix;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mino_names: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    match katamino(mino_names) {
        Ok(solution) => {
            let colors: HashMap<usize, (u8, u8, u8)> = HashMap::from([
                (0, (255, 0, 0)),      // Red
                (1, (0, 255, 0)),      // Green
                (2, (0, 0, 255)),      // Blue
                (3, (255, 255, 0)),    // Yellow
                (4, (255, 0, 255)),    // Magenta
                (5, (0, 255, 255)),    // Cyan
                (6, (192, 192, 192)),  // Silver
                (7, (128, 0, 128)),    // Purple
                (8, (255, 165, 0)),    // Orange
                (9, (128, 128, 128)),  // Gray
                (10, (255, 192, 203)), // Pink
                (11, (0, 128, 0)),     // Green
            ]);
            let solution_pretty = make_solution_pretty(&solution, colors);
            println!("{}", solution_pretty);
        }
        Err(_) => {
            eprintln!("Error: Invalid polyomino name");
        }
    }
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

fn katamino(mino_names: Vec<&str>) -> Result<Option<Vec<(usize, Vec<Vec<usize>>)>>, ()> {
    let mino_dict = HashMap::from([
        (
            "L",
            str_to_matrix(vec![
                "###", //
                "#..", //
                "#..",
            ]),
        ),
        (
            "l",
            str_to_matrix(vec![
                "####", //
                "#...",
            ]),
        ),
        (
            "I",
            str_to_matrix(vec![
                "#####", //
            ]),
        ),
        (
            "C",
            str_to_matrix(vec![
                "##", //
                "#.", //
                "##",
            ]),
        ),
        (
            "S",
            str_to_matrix(vec![
                ".##", //
                ".#.", //
                "##.",
            ]),
        ),
        (
            "s",
            str_to_matrix(vec![
                ".###", //
                "##..",
            ]),
        ),
        (
            "X",
            str_to_matrix(vec![
                ".#.", //
                "###", //
                ".#.",
            ]),
        ),
        (
            "F",
            str_to_matrix(vec![
                "##.", //
                ".##", //
                ".#.",
            ]),
        ),
        (
            "T",
            str_to_matrix(vec![
                "###", //
                ".#.", //
                ".#.",
            ]),
        ),
        (
            "t",
            str_to_matrix(vec![
                "####", //
                ".#..",
            ]),
        ),
        (
            "M",
            str_to_matrix(vec![
                ".##", //
                "##.", //
                "#..",
            ]),
        ),
        (
            "b",
            str_to_matrix(vec![
                "#.", //
                "##", //
                "##",
            ]),
        ),
    ]);
    let minos: Vec<Vec<Vec<usize>>> = mino_names
        .iter()
        .map(|&name| mino_dict.get(name).ok_or(()))
        .collect::<Result<Vec<_>, ()>>()?
        .iter()
        .map(|&v| v.clone())
        .collect();

    let problem = PolyominoTiling::new(minos.len(), 5, minos);
    let solution = problem.solve();

    Ok(solution)
}

mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let _ = env_logger::try_init();
        let mino_l3x3 = str_to_matrix(vec![
            "###", //
            "#..", //
            "#..",
        ]);
        let mino_l2x4: Vec<Vec<usize>> = str_to_matrix(vec![
            "####", //
            "#...",
        ]);
        let mino_i = str_to_matrix(vec![
            "#####", //
        ]);
        let mino_c = str_to_matrix(vec![
            "###", //
            "#..", //
            "###",
        ]);
        let mino_s3x3 = str_to_matrix(vec![
            ".##", //
            ".#.", //
            "##.",
        ]);
        let mino_s2x4 = str_to_matrix(vec![
            "###.", //
            "..##", //
        ]);
        let mino_cross = str_to_matrix(vec![
            ".#.", //
            "###", //
            ".#.",
        ]);
        let mino_f = str_to_matrix(vec![
            "##.", //
            ".##", //
            ".#.",
        ]);
        let mino_t3x3 = str_to_matrix(vec![
            "###", //
            ".#.", //
            ".#.", //
        ]);
        let mino_t2x4 = str_to_matrix(vec![
            "####", //
            ".#..", //
        ]);
        let mino_m = str_to_matrix(vec![
            ".##", //
            "##.", //
            "#..",
        ]);
        let mino_b = str_to_matrix(vec![
            ".#", //
            "##", //
            "##",
        ]);

        let polyominoes = vec![
            mino_l3x3, mino_t3x3, mino_b, mino_m, mino_s3x3, mino_t2x4, mino_l2x4, mino_s2x4,
        ];

        let tiling = PolyominoTiling::new(8, 5, polyominoes);
        let solution = tiling.solve();
        let colors: HashMap<usize, (u8, u8, u8)> = HashMap::from([
            (0, (255, 0, 0)),     // Red
            (1, (0, 255, 0)),     // Green
            (2, (0, 0, 255)),     // Blue
            (3, (255, 255, 0)),   // Yellow
            (4, (255, 0, 255)),   // Magenta
            (5, (0, 255, 255)),   // Cyan
            (6, (192, 192, 192)), // Silver
            (7, (128, 0, 128)),   // Purple
        ]);
        let solution_pretty = make_solution_pretty(&solution, colors);
        println!("Solution:\n{}", solution_pretty);
    }
}
