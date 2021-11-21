pub fn process_row(row: Vec<u8>) -> Vec<u8> {
    let original_length = row.len();
    let mut row: Vec<u8> = row.into_iter().filter(|i| i > &0).collect();
    for i in 0..(row.len() - 1) {
        if row[i] == row[i + 1] {
            row[i] += 1;
            row[i + 1] = 0;
        }
    }
    let mut row: Vec<u8> = row.into_iter().filter(|i| i > &0).collect();
    row.resize(original_length, 0);
    row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_row_correctly() {
        assert_eq!(process_row(vec![0, 1, 1, 1]), vec![2, 1, 0, 0]);
        assert_eq!(process_row(vec![1, 1, 1, 1]), vec![2, 2, 0, 0]);
        assert_eq!(process_row(vec![1, 0, 0, 1]), vec![2, 0, 0, 0]);
        assert_eq!(process_row(vec![2, 0, 2, 1]), vec![3, 1, 0, 0]);
    }
}
