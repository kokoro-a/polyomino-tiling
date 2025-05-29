mod dancing_links;
mod polyomino_tiling;
mod pretty;
use polyomino_tiling::PolyominoTiling;

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
    let solution_pretty = solution.as_ref().map(|s| {
        s.iter()
            .map(|(piece_id, placement)| {
                let placement_str = pretty::matrix_to_str(placement);
                format!("Piece {}:\n{}", piece_id, placement_str.join("\n"))
            })
            .collect::<Vec<String>>()
            .join("\n")
    });
    println!(
        "Solution:\n{}",
        solution_pretty.unwrap_or_else(|| "No solution found".to_string())
    );
}
