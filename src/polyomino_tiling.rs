use log::{debug, info};

use crate::dancing_links::DancingLinks;

pub struct PolyominoTiling {
    pub width: usize,
    pub height: usize,
    pub polyominoes: Vec<Vec<Vec<usize>>>,
}

impl PolyominoTiling {
    pub fn new(width: usize, height: usize, polyominoes: Vec<Vec<Vec<usize>>>) -> Self {
        PolyominoTiling {
            width,
            height,
            polyominoes,
        }
    }

    fn encode_into_exact_cover_problem_matrix(&self) -> Vec<Vec<usize>> {
        let n_pieces = self.polyominoes.len();
        let exact_cover_problem_matrix: Vec<Vec<usize>> = self
            .polyominoes
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(|(piece_id, polyomino)| {
                let placements = get_all_placements(&polyomino, self.width, self.height);
                let placements_flattened: Vec<Vec<usize>> = placements
                    .iter()
                    .map(|placement| flatten(placement))
                    .collect();
                let one_hot_encoded_piece_id = encode_one_hot(n_pieces, piece_id);
                let placements_with_piece_id: Vec<Vec<usize>> = placements_flattened
                    .into_iter()
                    .map(|placement| {
                        let mut row = one_hot_encoded_piece_id.clone();
                        row.extend(placement);
                        row
                    })
                    .collect();
                placements_with_piece_id
            })
            .collect();

        exact_cover_problem_matrix
    }

    fn decode_dlx_solution(
        &self,
        matrix: &[Vec<usize>],
        dlx_solution: &Option<Vec<usize>>,
    ) -> Option<Vec<(usize, Vec<Vec<usize>>)>> {
        if (dlx_solution).is_none() {
            return None;
        }
        let dlx_solution = (dlx_solution).as_ref().unwrap();
        let rows: Vec<Vec<usize>> = dlx_solution
            .iter()
            .map(|&index| matrix[index].clone())
            .collect();
        let piece_ids: Vec<usize> = rows
            .iter()
            .map(|r| decode_one_hot(&r[..self.polyominoes.len()]).unwrap())
            .collect();
        let placements: Vec<Vec<Vec<usize>>> = rows
            .iter()
            .map(|r| r[self.polyominoes.len()..].to_vec()) // flattened placements
            .map(|r| {
                (0..(self.height))
                    .map(|i| r[(self.width * i)..(self.width * (i + 1))].to_vec())
                    .collect::<Vec<Vec<usize>>>()
            })
            .collect::<Vec<Vec<Vec<usize>>>>();

        let solution: Vec<(usize, Vec<Vec<usize>>)> =
            piece_ids.into_iter().zip(placements).collect();
        Some(solution)
    }

    pub fn solve(&self) -> Option<Vec<(usize, Vec<Vec<usize>>)>> {
        if !self.is_board_size_eq_to_number_of_cells_of_polyominoes() {
            info!(
                "Board size does not match the total number of cells in polyominoes. \
                width={}, height={}, total_cells={}",
                self.width,
                self.height,
                self.polyominoes
                    .iter()
                    .map(|polyomino| polyomino.iter().flatten().sum::<usize>())
                    .sum::<usize>()
            );
            return None;
        }
        let matrix = self.encode_into_exact_cover_problem_matrix();
        debug!(
            "problem reduced into exact cover problem matrix: {:?}",
            matrix
        );
        let mut dlx =
            DancingLinks::from_vecs(&matrix, self.width * self.height + self.polyominoes.len());
        let dlx_solution = dlx.solve();
        self.decode_dlx_solution(&matrix, &dlx_solution)
    }

    fn is_board_size_eq_to_number_of_cells_of_polyominoes(&self) -> bool {
        let total_cells: usize = self
            .polyominoes
            .iter()
            .map(|polyomino| polyomino.iter().flatten().sum::<usize>())
            .sum();
        debug!(
            "width={}, height={}, total_cells={}",
            self.width, self.height, total_cells
        );
        total_cells == self.width * self.height
    }
}

fn rotate(matrix: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n_col = matrix.len();
    let n_row = if n_col > 0 { matrix[0].len() } else { 0 };
    let mut rotated_matrix = vec![vec![0; n_col]; n_row];

    for (i, _) in matrix.iter().enumerate().take(n_col) {
        for (j, _) in (0..n_row).enumerate() {
            rotated_matrix[j][n_col - 1 - i] = matrix[i][j];
        }
    }
    rotated_matrix
}

fn mirror(matrix: &[Vec<usize>]) -> Vec<Vec<usize>> {
    matrix
        .iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect()
}

fn get_all_rotations_and_mirrors(matrix: &[Vec<usize>]) -> Vec<Vec<Vec<usize>>> {
    let mirror = mirror(matrix);

    let rotation_1 = rotate(matrix);
    let rotation_2 = rotate(&rotation_1);
    let rotation_3 = rotate(&rotation_2);
    let rotations_self = vec![matrix.to_vec(), rotation_1, rotation_2, rotation_3];

    let mirror_rotation_1 = rotate(&mirror);
    let mirror_rotation_2 = rotate(&mirror_rotation_1);
    let mirror_rotation_3 = rotate(&mirror_rotation_2);
    let rotations_mirror = vec![
        mirror,
        mirror_rotation_1,
        mirror_rotation_2,
        mirror_rotation_3,
    ];

    let mut all = vec![];
    all.extend(rotations_self);
    all.extend(rotations_mirror);
    all
}

fn get_all_placements_without_rotation_nor_mirror(
    matrix: &[Vec<usize>],
    width: usize,
    height: usize,
) -> Vec<Vec<Vec<usize>>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![vec![vec![0; width]; height]];
    }
    let piece_height = matrix.len();
    let piece_width = matrix[0].len();

    let mut placements = vec![];

    if height < piece_height || width < piece_width {
        info!(
            "piece is larger than board. height={}, width={}, piece_height={}, piece_width={}",
            height, width, piece_height, piece_width
        );
        return placements;
    }
    for i in 0..=height - piece_height {
        for j in 0..=width - piece_width {
            let mut placement = vec![vec![0; width]; height];
            for r in 0..piece_height {
                for c in 0..piece_width {
                    placement[i + r][j + c] = matrix[r][c];
                }
            }
            placements.push(placement);
        }
    }
    placements
}

fn get_all_placements(matrix: &[Vec<usize>], width: usize, height: usize) -> Vec<Vec<Vec<usize>>> {
    let mut placements = vec![];
    let all_rotations_and_mirrors = get_all_rotations_and_mirrors(matrix);
    for m in all_rotations_and_mirrors {
        placements.extend(get_all_placements_without_rotation_nor_mirror(
            &m, width, height,
        ));
    }
    debug!(
        "Found {} placements for piece with dimensions {}x{}",
        placements.len(),
        matrix.len(),
        if matrix.is_empty() {
            0
        } else {
            matrix[0].len()
        }
    );
    placements
}

fn flatten(matrix: &[Vec<usize>]) -> Vec<usize> {
    matrix.iter().flat_map(|row| row.iter()).cloned().collect()
}

fn encode_one_hot(len: usize, index: usize) -> Vec<usize> {
    let mut vec = vec![0; len];
    if index < len {
        vec[index] = 1;
    } else {
        panic!("Index out of bounds for one-hot encoding");
    }
    vec
}

fn decode_one_hot(encoded: &[usize]) -> Option<usize> {
    encoded
        .iter()
        .enumerate()
        .find(|&(_, &val)| val == 1)
        .map(|(index, _)| index)
}

pub fn piece_placements_to_matrix_of_piece_ids(
    piece_placements: &[(usize, Vec<Vec<usize>>)],
    width: usize,
    height: usize,
) -> Vec<Vec<Option<usize>>> {
    /*
    ## Example
    ```rust
    let solution = vec![
        (0, vec![vec![1, 0, 0], vec![0, 1, 1]]),
        (1, vec![vec![0, 1, 0], vec![1, 0, 0]]),

    let matrix = solution_to_matrix_of_piece_ids(&solution);
    assert_eq!(
        matrix,
        vec![
            vec![Some(0), Some(1), None],
            vec![Some(1), Some(0), Some(0)],
        ]
    );
    ```
    */

    let mut matrix: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
    for (piece_id, placement) in piece_placements {
        for (i, row) in placement.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                if value == 1 {
                    matrix[i][j] = Some(*piece_id);
                }
            }
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten() {
        let matrix = vec![
            vec![1, 0, 0], //
            vec![0, 1, 1], //
        ];
        let flattened = flatten(&matrix);
        assert_eq!(flattened, vec![1, 0, 0, 0, 1, 1]);
    }
    #[test]
    fn test_rotate_matrix() {
        let matrix = vec![
            vec![1, 0, 0], //
            vec![1, 1, 1], //
        ];
        let rotated = rotate(&matrix);
        assert_eq!(
            rotated,
            vec![
                vec![1, 1], //
                vec![1, 0], //
                vec![1, 0]  //
            ]
        );
    }

    #[test]
    fn test_mirror_matrix() {
        let matrix = vec![
            vec![1, 0, 0], //
            vec![0, 1, 0], //
        ];
        let mirrored = mirror(&matrix);
        assert_eq!(
            mirrored,
            vec![
                vec![0, 0, 1], //
                vec![0, 1, 0]  //
            ]
        );
    }

    #[test]
    fn test_get_all_rotations_and_mirrors() {
        let matrix = vec![
            vec![1, 0, 0], //
            vec![0, 1, 0], //
        ];
        let mut actual = get_all_rotations_and_mirrors(&matrix);
        let mut expected = vec![
            vec![
                vec![1, 0, 0], //
                vec![0, 1, 0], //
            ],
            vec![
                vec![0, 1], //
                vec![1, 0], //
                vec![0, 0], //
            ],
            vec![
                vec![0, 1, 0], //
                vec![0, 0, 1], //
            ],
            vec![
                vec![0, 0], //
                vec![0, 1], //
                vec![1, 0], //
            ],
            vec![
                vec![0, 0, 1], //
                vec![0, 1, 0], //
            ],
            vec![
                vec![1, 0], //
                vec![0, 1], //
                vec![0, 0], //
            ],
            vec![
                vec![0, 1, 0], //
                vec![1, 0, 0], //
            ],
            vec![
                vec![0, 0], //
                vec![1, 0], //
                vec![0, 1], //
            ],
        ];
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_all_placement_without_rotation_nor_mirror() {
        let matrix = vec![
            vec![1, 0, 0], //
            vec![0, 1, 1], //
        ];
        let mut actual = get_all_placements_without_rotation_nor_mirror(&matrix, 4, 3);
        let mut expected: Vec<Vec<Vec<usize>>> = vec![
            vec![
                vec![1, 0, 0, 0], //
                vec![0, 1, 1, 0], //
                vec![0, 0, 0, 0], //
            ],
            vec![
                vec![0, 1, 0, 0], //
                vec![0, 0, 1, 1], //
                vec![0, 0, 0, 0], //
            ],
            vec![
                vec![0, 0, 0, 0], //
                vec![1, 0, 0, 0], //
                vec![0, 1, 1, 0], //
            ],
            vec![
                vec![0, 0, 0, 0], //
                vec![0, 1, 0, 0], //
                vec![0, 0, 1, 1], //
            ],
        ];
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_piece_placements_to_matrix_of_piece_ids() {
        let piece_placements = vec![
            (0, vec![vec![1, 0, 0], vec![0, 1, 1]]),
            (1, vec![vec![0, 1, 0], vec![1, 0, 0]]),
        ];
        let width = 3;
        let height = 2;
        let matrix = piece_placements_to_matrix_of_piece_ids(&piece_placements, width, height);
        assert_eq!(
            matrix,
            vec![
                vec![Some(0), Some(1), None],
                vec![Some(1), Some(0), Some(0)],
            ]
        );
    }

    #[test]
    fn test_is_board_size_eq_to_number_of_cells_of_polyominoes() {
        _ = env_logger::builder().is_test(true).try_init();
        let polyominoes = vec![
            vec![vec![1, 0], vec![0, 1]], // 2 cells
            vec![vec![1, 1, 1]],          // 3 cells
        ];
        let tiling = PolyominoTiling::new(2, 2, polyominoes);
        assert!(!tiling.is_board_size_eq_to_number_of_cells_of_polyominoes());

        let polyominoes = vec![
            vec![vec![1, 0], vec![0, 1]], // 2 cells
            vec![vec![1, 1]],             // 2 cells
        ];
        let tiling = PolyominoTiling::new(2, 2, polyominoes);
        assert!(tiling.is_board_size_eq_to_number_of_cells_of_polyominoes());
    }

    #[test]
    fn test_solve_polyomino_tiling_no_solution() {
        _ = env_logger::builder().is_test(true).try_init();
        let polyominos = vec![vec![
            vec![1, 1, 1], //
            vec![0, 1, 0], //
            vec![0, 1, 0], //
        ]];
        let tiling = PolyominoTiling::new(1, 5, polyominos);
        let solution = tiling.solve();
        assert!(
            solution.is_none(),
            "Expected no solution for mismatched board size"
        );
    }
}
