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

    let two = p2(&data);
    println!("Part 2: {}", two);

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

fn slice_without_index(slice: &Vec<u64>, k: usize) -> Vec<u64> {
    let before = &slice[..k];
    let after = if slice.len() >= (k + 1) {
        &slice[(k + 1)..]
    } else {
        &[0; 0]
    };
    [before, after].concat()
}

fn determine_incrementor(row: &Vec<u64>, exit: bool) -> bool {
    if row[0] < row[1] && row[0] >= row[row.len() - 1]
        || row[0] > row[1] && row[0] <= row[row.len() - 1]
    {
        if exit {
            return false;
        } else {
            let without_last = slice_without_index(row, 0);
            let without_current = slice_without_index(row, 1);
            return determine_incrementor(&without_last, true)
                || determine_incrementor(&without_current, true);
        }
    }

    if row[0] < row[1] {
        for idx in 1..row.len() {
            if row[idx - 1] >= row[idx] || (row[idx] - row[idx - 1]) > 3 {
                if exit {
                    return false;
                } else {
                    let without_last = slice_without_index(row, idx - 1);
                    let without_current = slice_without_index(row, idx);
                    let combine = determine_incrementor(&without_last, true)
                        || determine_incrementor(&without_current, true);
                    if !combine {
                        println!("full: {:?}", row);
                        println!("missing: {} - {:?}", idx - 1, without_last);
                        println!("missing: {} - {:?}\n\n", idx, without_current);
                    }

                    return combine;
                }
            }
        }
    } else {
        for idx in 1..row.len() {
            if row[idx - 1] <= row[idx] || (row[idx - 1] - row[idx]) > 3 {
                if exit {
                    return false;
                } else {
                    let without_last = slice_without_index(row, idx - 1);
                    let without_current = slice_without_index(row, idx);
                    let combine = determine_incrementor(&without_last, true)
                        || determine_incrementor(&without_current, true);
                    if !combine {
                        println!("full: {:?}", row);
                        println!("missing: {} - {:?}", idx - 1, without_last);
                        println!("missing: {} - {:?}\n\n", idx, without_current);
                    }

                    return combine;
                }
            }
        }
    };

    return true;
}

fn p1(rows: &Vec<Vec<u64>>) -> usize {
    let count = rows.iter().fold(0, |acc, row| {
        let valid = determine_incrementor(row, true);
        if valid {
            return acc + 1;
        }
        acc
    });

    count
}

fn p2(rows: &Vec<Vec<u64>>) -> usize {
    let count = rows.iter().fold(0, |acc, row| {
        let valid = determine_incrementor(row, false);
        if valid {
            return acc + 1;
        }
        acc
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
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
            vec![85, 85, 89, 90, 91],
        ];

        let v = p1(&data);
        assert_eq!(v, 2);
    }

    #[test]
    fn test_p2() {
        let data = vec![
            vec![7, 6, 4, 2, 1],              // safe
            vec![1, 2, 7, 8, 9],              // unsafe
            vec![9, 7, 6, 2, 1],              // unsafe
            vec![1, 3, 2, 4, 5],              // safe without 3
            vec![8, 6, 4, 4, 1],              // safe without 4
            vec![1, 3, 6, 7, 9],              // safe
            vec![48, 55, 58, 59, 62, 63, 63], // unsafe
            vec![48, 55, 58, 59, 62, 63, 65], //safe without 0
            // From reddit
            vec![48, 46, 47, 49, 51, 54, 56], // safe
            vec![1, 1, 2, 3, 4, 5],           // safe
            vec![1, 2, 3, 4, 5, 5],           // safe
            vec![5, 1, 2, 3, 4, 5],           // safe
            vec![1, 4, 3, 2, 1],              // safe
            vec![1, 6, 7, 8, 9],              // safe
            vec![1, 2, 3, 4, 3],              // safe
            vec![9, 8, 7, 6, 7],              // safe
        ];

        let v = p2(&data);
        assert_eq!(v, 13);
    }
}
