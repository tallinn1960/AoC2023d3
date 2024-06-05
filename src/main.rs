use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("input.txt").expect("can't open file");
    let mut f = BufReader::new(f);
    let result = p1(&mut f);
    println!("{}", result);
}

fn p1<F: BufRead>(schema: &mut F) -> u32 {
    let mut result = 0;
    // append empty lines to the beginning and end of the schema
    let a: Vec<String> = std::iter::once("".to_string())
        .chain(schema.lines().map(|r| r.expect("line read failed")))
        .chain(std::iter::once("".to_string()))
        .collect();
    a.windows(3).for_each(|w| {
        for (start, end, number) in scan_digits(&w[1]) {
            // construct a string from start..end of previous line,
            // this line, and next line
            let s0 = &w[0][min(start, w[0].len())..min(end, w[0].len())];
            let s1 = &w[1][min(start, w[1].len())..min(end, w[1].len())];
            let s2 = &w[2][min(start, w[2].len())..min(end, w[2].len())];
            let s = format!("{}{}{}", s0, s1, s2);
            // check for special chars in s (i.e. any char that is not . or digit)
            if s.chars().any(|c| !c.is_ascii_digit() && c != '.') {
                result += number;
            }
        }
    });
    result
}

// scan for consecutive digits in a string, record the start and end index of each sequence
// including possibly lead and trailing char, and the number found
fn scan_digits(s: &str) -> Vec<(usize, usize, u32)> {
    let mut result = Vec::new();
    let mut start = None;
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() {
            if start.is_none() {
                start = Some(i);
            }
        } else {
            if let Some(start) = start {
                let number = u32::from_str_radix(&s[start..i], 10).unwrap();
                let start = start.saturating_sub(1);
                let end = i.saturating_add(1).min(s.len());
                result.push((start, end, number));
            }
            start = None;
        }
    }
    if let Some(start) = start {
        let number = u32::from_str_radix(&s[start..], 10).unwrap();
        let start = start.saturating_sub(1);
        result.push((start, s.len(), number));
    }
    result
}

fn _p2<F: BufRead>(schema: &mut F) -> u32 {
    let mut result = 0;
    // append empty lines to the beginning and end of the schema
    let a: Vec<String> = std::iter::once("".to_string())
        .chain(schema.lines().map(|r| r.expect("line read failed")))
        .chain(std::iter::once("".to_string()))
        .collect();
    a.windows(3).for_each(|w| {
        for (start, end, number) in scan_digits(&w[1]) {
            // construct a string from start..end of previous line,
            // this line, and next line
            let s0 = &w[0][min(start, w[0].len())..min(end, w[0].len())];
            let s1 = &w[1][min(start, w[1].len())..min(end, w[1].len())];
            let s2 = &w[2][min(start, w[2].len())..min(end, w[2].len())];
            let s = format!("{}{}{}", s0, s1, s2);
            // check for special chars in s (i.e. any char that is not . or digit)
            if s.chars().any(|c| !c.is_ascii_digit() && c != '.') {
                result += number;
            }
        }
    });
    result
}

fn _scan_for_gears(s: &str) -> Vec<usize> {
    let mut result = Vec::new();
    for (i, c) in s.char_indices() {
        if c != '.' && !c.is_ascii_digit() {
            result.push(i);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_p1() {
        let mut f = BufReader::new(INPUT.as_bytes());
        assert_eq!(p1(&mut f), 4361);
    }
}
