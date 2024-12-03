use regex::Regex;
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

fn p1(data: &Vec<String>) -> u64 {
    let line = &data[0];
    let pairs = parse_mul(line);
    pairs.iter().fold(0, |acc, p| acc + (p.0 * p.1))
}

fn p2(data: &Vec<String>) -> u64 {
    let line = &data[0];
    line.split("don't()")
        .enumerate()
        .map(|(i, s)| match i {
            0 => {
                let pairs = parse_mul(s);
                pairs.iter().fold(0, |acc, p| acc + (p.0 * p.1))
            }
            _ => {
                let sections: Vec<&str> = s.split("do()").collect();
                let mut total = 0;
                if sections.len() > 1 {
                    for chunk in sections[1..].iter() {
                        let pairs = parse_mul(chunk);
                        total += pairs.iter().fold(0, |acc, p| acc + (p.0 * p.1))
                    }
                }
                total
            }
        })
        .fold(0, |acc, x| acc + x)
}

fn parse_mul(line: &str) -> Vec<(u64, u64)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut params: Vec<(&str, &str)> = vec![];
    for (_, [one, two]) in re.captures_iter(line).map(|c| c.extract()) {
        params.push((one, two));
    }

    let num_params: Vec<(u64, u64)> = params
        .iter()
        .map(|p| {
            let (f, s) = p;
            let n = match f.parse::<u64>() {
                Ok(v) => v,
                Err(e) => panic!("unable to parse u64 from string! {}", e),
            };
            let m = match s.parse::<u64>() {
                Ok(v) => v,
                Err(e) => panic!("unable to parse u64 from string! {}", e),
            };
            (n, m)
        })
        .collect();

    num_params
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let r = p1(&vec![input.to_string()]);
        assert_eq!(161, r);
    }

    #[test]
    fn test_p2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let r = p2(&vec![input.to_string()]);
        assert_eq!(48, r);
    }

    #[test]
    fn test_parse() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let r = parse_mul(input);
        assert_eq!(vec![(2, 4), (5, 5), (11, 8), (8, 5),], r);
    }
}
