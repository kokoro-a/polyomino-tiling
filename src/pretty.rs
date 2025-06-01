pub fn str_to_matrix(s: Vec<&str>) -> Vec<Vec<usize>> {
    /*
    ## Example

    ```
    let s = vec![
        "..#",
        "###"
    ]

    let matrix = str_to_matrix(s);
    assert_eq!(matrix, vec![vec![0, 0, 1], vec![1, 1, 1]]);
    ```
    */
    s.into_iter()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

pub fn matrix_to_str(matrix: &Vec<Vec<usize>>) -> Vec<String> {
    /*
    ## Example

    ```
    let matrix = vec![vec![0, 0, 1], vec![1, 1, 1]];
    let s = matrix_to_str(&matrix);
    assert_eq!(s, vec!["..#", "###"]);
    ```
    */
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|&x| if x == 0 { '.' } else { '#' })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_matrix() {
        let s = vec!["..#", "###"];
        let matrix = str_to_matrix(s);
        assert_eq!(matrix, vec![vec![0, 0, 1], vec![1, 1, 1]]);
    }

    #[test]
    fn test_matrix_to_str() {
        let matrix = vec![vec![0, 0, 1], vec![1, 1, 1]];
        let s = matrix_to_str(&matrix);
        assert_eq!(s, vec!["..#", "###"]);
    }
}
