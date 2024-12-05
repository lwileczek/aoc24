use std::error::Error;
use utils::read_file;

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_file("data.txt")?;
    let ans = p1(&data);
    println!("Problem 1: {}", ans);

    let second_answer = p2(&data);
    println!("Problem 2: {}", second_answer);
    Ok(())
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn find_next_char(m: &Vec<Vec<&str>>, ch: &str, idx: (i64, i64), mv: (i64, i64)) -> i64 {
    let pos = (idx.0 + mv.0, idx.1 + mv.1);
    if m[pos.0 as usize][pos.1 as usize] == ch {
        let v = match ch {
            "M" => find_next_char(m, "A", pos, mv),
            "A" => find_next_char(m, "S", pos, mv),
            "S" => 1 as i64,
            _ => 0 as i64,
        };
        return v;
    }
    return 0;
}

fn p1(data: &Vec<String>) -> u64 {
    let n0 = data.iter().fold(0, |acc, line| {
        let fwd = line.matches("XMAS").count();
        let bkwd = line.matches("SAMX").count();
        acc + fwd + bkwd
    });

    let matrix: Vec<Vec<&str>> = data.iter().map(|line| line.split("").collect()).collect();
    let transposed_matrix = transpose(matrix.clone());
    let transposed_data: Vec<String> = transposed_matrix
        .iter()
        .map(|row| row.iter().fold(String::new(), |acc, v| acc + v))
        .collect();

    let n1 = transposed_data.iter().fold(0, |acc, line| {
        let fwd = line.matches("XMAS").count();
        let bkwd = line.matches("SAMX").count();
        acc + fwd + bkwd
    });

    let n2 = matrix.iter().enumerate().fold(0, |acc, (row, line)| {
        let c = line.iter().enumerate().fold(0, |sacc, (col, ch)| {
            let mut v = 0;
            if *ch == "X" {
                //check up
                if row > 2 {
                    //check up left
                    if col > 2 {
                        v = v + find_next_char(&matrix, "M", (row as i64, col as i64), (-1, -1));
                    }
                    //check up right
                    if col < (line.len() - 3) {
                        v = v + find_next_char(&matrix, "M", (row as i64, col as i64), (-1, 1));
                    }
                }

                //check down
                if row < matrix.len() - 3 {
                    //check down left
                    if col > 2 {
                        v = v + find_next_char(&matrix, "M", (row as i64, col as i64), (1, -1));
                    }
                    //check down right
                    if col < (line.len() - 3) {
                        v = v + find_next_char(&matrix, "M", (row as i64, col as i64), (1, 1));
                    }
                }
            }

            sacc + v
        });
        acc + c
    });

    (n0 + n1 + (n2 as usize)) as u64
}

fn is_mas(x: &str, y: &str) -> bool {
    (x == "S" || x == "M") && (y == "S" || y == "M") && x != y
}

fn p2(data: &Vec<String>) -> u64 {
    let left = |x| x - 1;
    let right = |x: usize| x + 1;
    let above = |x| x - 1;
    let below = |x: usize| x + 1;

    let matrix: Vec<Vec<&str>> = data.iter().map(|line| line.split("").collect()).collect();
    matrix.iter().enumerate().fold(0, |acc, (row, line)| {
        let c = line.iter().enumerate().fold(0, |sacc, (col, ch)| {
            if row == 0 || row == matrix.len() - 1 || col == 0 || col == line.len() {
                return sacc;
            }
            let mut v = 0;
            if *ch == "A" {
                let ul = matrix[left(row)][above(col)];
                let br = matrix[right(row)][below(col)];

                let ur = matrix[right(row)][above(col)];
                let bl = matrix[left(row)][below(col)];

                if is_mas(ul, br) && is_mas(ur, bl) {
                    v = 1;
                }
            }

            sacc + v
        });
        acc + c as u64
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let data: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();
        let r = p1(&data);
        assert_eq!(18, r);
    }

    #[test]
    fn test_p2() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let data: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();
        let r = p2(&data);
        assert_eq!(9, r);
    }
}
