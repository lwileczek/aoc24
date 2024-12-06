use std::fs;

fn main() -> std::io::Result<()> {
    let raw = fs::read_to_string("data.txt")?;
    let contents: &str = &raw;
    parse(contents);
    println!("Hello, world!");

    Ok(())
}

struct Data {
    x: Vec<u64>,
    y: Vec<u64>,
    order: Vec<Vec<u64>>,
}

fn p1(data: &Data) {
    data.order.iter().filter(|row| {
        //check rows
    })
}

fn parse(s: &str) -> Data {
    let parts: Vec<&str> = s.split("\n\n").collect();

    let rules = parts[0];
    let actions = parts[1];

    let mut x: Vec<u64> = vec![];
    let mut y: Vec<u64> = vec![];

    let nums: Vec<Vec<u64>> = rules
        .split("\n")
        .map(|line| {
            let x: Vec<u64> = line
                .split("|")
                .map(|val| val.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            x
        })
        .collect();

    for pair in nums {
        if pair.len() >= 2 {
            x.push(pair[0]);
            y.push(pair[1]);
        }
    }

    let order: Vec<Vec<u64>> = actions
        .split("\n")
        .map(|section| {
            section
                .split(",")
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    Data { x, y, order }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let actions = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ];
        let x = vec![
            47, 97, 97, 97, 75, 61, 75, 29, 97, 53, 61, 97, 61, 47, 75, 97, 47, 75, 47, 75, 53,
        ];
        let y = vec![
            53, 13, 61, 47, 29, 13, 53, 13, 29, 29, 53, 53, 29, 13, 47, 75, 61, 61, 29, 13, 13,
        ];

        let r = parse(input);
        assert_eq!(actions, r.order);
        assert_eq!(x, r.x);
        assert_eq!(y, r.y);
    }
}
