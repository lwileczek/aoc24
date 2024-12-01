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
    let data = read_file("data.txt")?;
    let (left, right) = match convert_data(&data) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    let one = p1(&left, &right);
    println!("Part 1: {}", one);

    let two = p2(&left, &right);
    println!("Part 2: {}", two);

    Ok(())
}

fn convert_data(data: &Vec<String>) -> Result<(Vec<u64>, Vec<u64>), WrongCountError> {
    let mut nums: Vec<Vec<u64>> = vec![vec![0, 0]; data.len()];
    for (i, line) in data.iter().enumerate() {
        let vals = parse_ints(line);
        nums[i] = vals;
    }

    let mut left: Vec<u64> = vec![0; data.len()];
    let mut right: Vec<u64> = vec![0; data.len()];

    for (i, pair) in nums.iter().enumerate() {
        if pair.len() != 2 {
            return Err(WrongCountError {
                expected: 2,
                found: pair.len(),
            });
        }

        left[i] = pair[0];
        right[i] = pair[1];
    }

    left.sort();
    right.sort();

    Ok((left, right))
}

fn p1(left: &Vec<u64>, right: &Vec<u64>) -> i64 {
    let sum = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (lv, rv)| acc + i64::abs(*lv as i64 - *rv as i64));

    sum
}

//TODO: check if current value and last value are the same
//since we sorted, check if the right value is larger than last value, and exit
fn p2(left: &Vec<u64>, right: &Vec<u64>) -> u64 {
    let prod = left.iter().fold(0, |acc, lv| {
        acc + (lv * right.iter().filter(|x| **x == *lv).count() as u64)
    });

    prod
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let mut lv = vec![3, 4, 2, 1, 3, 3];
        let mut rv = vec![4, 3, 5, 3, 9, 3];
        lv.sort();
        rv.sort();

        let v = p1(&lv, &rv);
        assert_eq!(v, 11);
    }

    #[test]
    fn test_p2() {
        let mut lv = vec![3, 4, 2, 1, 3, 3];
        let mut rv = vec![4, 3, 5, 3, 9, 3];
        lv.sort();
        rv.sort();

        let v = p2(&lv, &rv);
        assert_eq!(v, 31);
    }
}
