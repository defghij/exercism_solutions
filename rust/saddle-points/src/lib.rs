/// s is a saddle point if s >= all a in the row of s and s <= all b in the column of s.
/// Sufficient to find min of each column, max of each row, and check if the indicies overlap.
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_pts: Vec<(usize, usize)> = Vec::new();

    let maxes: Vec<(usize, usize)> = get_row_maxes(input);
    let mins:  Vec<(usize, usize)> = get_col_mins(input);

    maxes.iter().for_each(|m| {
        if mins.contains(m) {
            saddle_pts.push(*m);
        }
    });

    saddle_pts
}

fn get_col_mins(matrix: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut mins: Vec<(usize, usize)> = Vec::new();

    for c in 0..matrix[0].len() {
        let mut col_elements: Vec<u64>            = Vec::new();
        let mut indices:      Vec<(usize, usize)> = Vec::new();

        for r in 0..matrix.len() {
            indices.push((r, c));
            col_elements.push(matrix[r][c]);
        }

        let min = col_elements.iter().min().unwrap();
        col_elements.iter()
                .enumerate()
                .for_each(|(idx, elem)| {
                    if elem == min {
                        mins.push(indices[idx]);
                    }
                });
    }

    mins
}

fn get_row_maxes(matrix: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut maxes: Vec<(usize, usize)> = Vec::new();

    matrix.iter().enumerate().for_each(|(r, row)| {
        let max = row.iter().max();

        row.iter().enumerate().for_each(|(c, elem)| {
            if elem == max.unwrap() {
                maxes.push((r, c));
            }
        })
    });

    maxes
}
