use std::error::Error;
use std::fmt;
use utils::{parse_ints, read_file};

#[derive(Debug, Clone)]
struct WrongCountError {
    expected: u64,
    found: usize,
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for WrongCountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "wrong number of elements found! expected {} found {}",
            self.expected, self.found
        )
    }
}

impl Error for WrongCountError {}

fn main() -> Result<(), Box<dyn Error>> {
    let raw_data = read_file("data.txt")?;
    let data = match convert_data(&raw_data) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    let one = p1(&data);
    println!("Part 1: {}", one);

    Ok(())
}

fn convert_data(data: &Vec<String>) -> Result<Vec<Vec<u64>>, WrongCountError> {
    let mut nums: Vec<Vec<u64>> = vec![vec![0, 0]; data.len()];
    for (i, line) in data.iter().enumerate() {
        let vals = parse_ints(line);
        nums[i] = vals;
    }

    Ok(nums)
}

fn allow_one(row: &Vec<i64>) -> usize {
    return 1
}


fn determine_incrementor(row: &Vec<u64>) -> usize {
    if row[0] < row[1] {
        for val in 1..row.len() {
            if row[val - 1] >= row[val] {
                return 0;
            }
            if (row[val] - row[val-1]) > 3 {
                return 0;
            }
        }
        return 1;
    } else {
        for val in 1..row.len() {
            if row[val - 1] <= row[val] {
                return 0;
            }
            if (row[val-1] - row[val]) > 3 {
                return 0;
            }
        }
        return 1;
    };
}

fn p1(rows: &Vec<Vec<u64>>) -> usize {
    let count = rows.iter().fold(0, |acc, row| {
        let inc = determine_incrementor(row);
        acc + inc
    });

    count
}

fn p2(rows: &Vec<Vec<i64>>) -> usize {
    let count = rows.iter().fold(0, |acc, row| {
        let inc = allow_one(row);
        acc + inc
    });

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let data = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 9],
            vec![1, 3, 2, 4, 9],
            vec![8, 6, 4, 4, 9],
            vec![1, 3, 6, 7, 9],
        ];

        let v = p1(&data);
        assert_eq!(v, 2);
    }

    #[test]
    fn test_p2() {
        let data = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 9],
            vec![1, 3, 2, 4, 9],
            vec![8, 6, 4, 4, 9],
            vec![1, 3, 6, 7, 9],
        ];

        let v = p2(&data);
        assert_eq!(v, 4);
    }
}
