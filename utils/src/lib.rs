use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};

///parse a vector of integers from a string by extracting all of the base 10 digits
///and creating a new number each time any other character is found
pub fn parse_ints(line: &str) -> Vec<u64> {
    let charz: Vec<char> = line.chars().collect();
    let mut ans: Vec<u64> = Vec::new();
    let mut pos = 0;
    while pos < line.len() {
        if charz[pos].is_digit(10) {
            let mut end = pos + 1;
            while end < line.len() && line.chars().nth(end).unwrap().is_digit(10) {
                end += 1;
            }
            let n = match line[pos..end].parse::<u64>() {
                Ok(v) => v,
                Err(e) => panic!("unable to parse u64 from string! {}", e),
            };

            ans.push(n);
            pos = end;
            continue;
        }
        pos = pos + 1
    }
    ans
}

///Read a file and put each line as an entry into a vector
pub fn read_file(file_name: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut v: Vec<String> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        v.push(line);
    }

    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_integers() {
        struct TestCase<'a> {
            input: &'a str,
            output: Vec<u64>,
        }

        let test_cases = vec![
            TestCase {
                input: "1",
                output: vec![1],
            },
            TestCase {
                input: "12",
                output: vec![12],
            },
            TestCase {
                input: "123",
                output: vec![123],
            },
            TestCase {
                input: "123.3",
                output: vec![123, 3],
            },
            TestCase {
                input: " 123.3",
                output: vec![123, 3],
            },
            TestCase {
                input: ": 123.3",
                output: vec![123, 3],
            },
            TestCase {
                input: ": 123.3 5",
                output: vec![123, 3, 5],
            },
            TestCase {
                input: ":thing42  and 24",
                output: vec![42, 24],
            },
        ];

        for tc in test_cases.iter() {
            let result = parse_ints(tc.input);
            assert_eq!(result, tc.output);
        }
    }
}
