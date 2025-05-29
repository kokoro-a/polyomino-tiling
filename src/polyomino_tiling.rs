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
            .map(|(piece_id, polyomino)| {
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
            .flatten()
            .collect();

        exact_cover_problem_matrix
    }

    fn decode_dlx_solution(
        &self,
        matrix: &Vec<Vec<usize>>,
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
            .map(|r| decode_one_hot(&r[..self.polyominoes.len()].to_vec()).unwrap())
            .collect();
        let placements: Vec<Vec<Vec<usize>>> = rows
            .iter()
            .map(|r| r[self.polyominoes.len()..].to_vec()) // flattened placements
            .map(|r| {
                (0..(self.height))
                    .map(|i| {
                        let _r = r[(self.width * i)..(self.width * (i + 1))].to_vec();
                        _r
                    })
                    .collect::<Vec<Vec<usize>>>()
            })
            .collect::<Vec<Vec<Vec<usize>>>>();

        let solution: Vec<(usize, Vec<Vec<usize>>)> =
            piece_ids.into_iter().zip(placements.into_iter()).collect();
        return Some(solution);
    }

    pub fn solve(&self) -> Option<Vec<(usize, Vec<Vec<usize>>)>> {
        let matrix = self.encode_into_exact_cover_problem_matrix();
        let mut dlx = DancingLinks::from_vecs(&matrix);
        let dlx_solution = dlx.solve();
        let solution = self.decode_dlx_solution(&matrix, &dlx_solution);
        solution
    }
}

fn rotate(matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n_col = matrix.len();
    let n_row = if n_col > 0 { matrix[0].len() } else { 0 };
    let mut rotated_matrix = vec![vec![0; n_col]; n_row];

    for i in 0..n_col {
        for j in 0..n_row {
            rotated_matrix[j][n_col - 1 - i] = matrix[i][j];
        }
    }
    rotated_matrix
}

fn mirror(matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    matrix
        .iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect()
}

fn get_all_rotations_and_mirrors(matrix: &Vec<Vec<usize>>) -> Vec<Vec<Vec<usize>>> {
    let mirror = mirror(matrix);

    let rotations_self = vec![
        matrix.clone(),
        rotate(matrix),
        rotate(&rotate(matrix)),
        rotate(&rotate(&rotate(matrix))),
    ];

    let rotations_mirror = vec![
        mirror.clone(),
        rotate(&mirror),
        rotate(&rotate(&mirror)),
        rotate(&rotate(&rotate(&mirror))),
    ];

    let mut all = vec![];
    all.extend(rotations_self);
    all.extend(rotations_mirror);
    all
}

fn get_all_placements_without_rotation_nor_mirror(
    matrix: &Vec<Vec<usize>>,
    width: usize,
    height: usize,
) -> Vec<Vec<Vec<usize>>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![vec![vec![0; width]; height]];
    }
    let n_row = matrix.len();
    let n_col = matrix[0].len();

    let mut placements = vec![];
    let matrix = matrix;

    for i in 0..=height - n_row {
        for j in 0..=width - n_col {
            let mut placement = vec![vec![0; width]; height];
            for r in 0..n_row {
                for c in 0..n_col {
                    placement[i + r][j + c] = matrix[r][c];
                }
            }
            placements.push(placement);
        }
    }
    placements
}

fn get_all_placements(
    matrix: &Vec<Vec<usize>>,
    width: usize,
    height: usize,
) -> Vec<Vec<Vec<usize>>> {
    let mut placements = vec![];
    let all_rotations_and_mirrors = get_all_rotations_and_mirrors(matrix);
    for m in all_rotations_and_mirrors {
        placements.extend(get_all_placements_without_rotation_nor_mirror(
            &m, width, height,
        ));
    }
    placements
}

fn flatten(matrix: &Vec<Vec<usize>>) -> Vec<usize> {
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

fn decode_one_hot(encoded: &Vec<usize>) -> Option<usize> {
    encoded
        .iter()
        .enumerate()
        .find(|&(_, &val)| val == 1)
        .map(|(index, _)| index)
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
}
