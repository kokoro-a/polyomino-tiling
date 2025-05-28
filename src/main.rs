mod dancing_links;

use core::str;

use dancing_links::DancingLinks;

fn main() {
    let mut dl = DancingLinks::from_vecs(vec![vec![1, 0, 0], vec![0, 1, 1], vec![1, 1, 0]]);
    let solutions = dl.solve();
    println!("Solutions: {:?}", solutions);
}

struct PolyominoTiling {
    width: usize,
    height: usize,
    polyominoes: Vec<Polyomino>,
}

struct Polyomino {
    matrix: Vec<Vec<usize>>,
}

impl Polyomino {
    pub fn new(matrix: Vec<Vec<usize>>) -> Self {
        if matrix.is_empty() {
            return Polyomino { matrix };
        }
        let n_col = matrix[0].len();
        if matrix.iter().any(|row| row.len() != n_col) {
            panic!("all rows in the matrix must have the same number of columns.");
        }
        Polyomino { matrix }
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
