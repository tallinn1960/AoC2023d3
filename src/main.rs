use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader, Seek},
};

fn main() {
    let f = File::open("input.txt").expect("can't open file");
    let mut f = BufReader::new(f);
    let result = p1(&mut f);
    println!("{}", result);
    f.rewind().expect("rewind failed");
    let result = p2(&mut f);
    println!("{}", result);
}

fn p1<F: BufRead>(schema: &mut F) -> u32 {
    let mut result = 0;
    // append empty lines to the beginning and end of the schema
    let schema: Vec<String> = std::iter::once("".to_string())
        .chain(schema.lines().map(|r| r.expect("line read failed")))
        .chain(std::iter::once("".to_string()))
        .collect();
    schema.windows(3).for_each(|three_line_group| {
        for (start, end, number) in scan_for_numbers(&three_line_group[1]) {
            // construct a string from start..end of previous line,
            // this line, and next line
            let s0 = &three_line_group[0][min(start, three_line_group[0].len())..min(end, three_line_group[0].len())];
            let s1 = &three_line_group[1][start..end];
            let s2 = &three_line_group[2][min(start, three_line_group[2].len())..min(end, three_line_group[2].len())];
            // check for gear characters in the strings
            if s0
                .chars()
                .chain(s1.chars())
                .chain(s2.chars())
                .any(|c| is_gear_char(c))
            {
                result += number;
            }
        }
    });
    result
}

fn p2<F: BufRead>(schema: &mut F) -> u32 {
    let mut result = 0;
    let parsed_schema = std::iter::once("".to_string())
        .chain(schema.lines().map(|r| r.expect("line read failed")))
        .chain(std::iter::once("".to_string()))
        .map(|s| {
            let numbers = scan_for_numbers(&s);
            (s, numbers)
        })
        .collect::<Vec<_>>();
    parsed_schema.windows(3).for_each(|three_line_group| {
        for pos in scan_for_gears(&three_line_group[1].0) {
            let found = three_line_group.iter()
                .flat_map(|line| &line.1)
                .filter_map(|(start, end, number)| {
                    if pos >= *start && pos < *end {
                        Some(*number)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if found.len() > 1 {
                result += found.iter().fold(1, |acc, &x| acc * x);
            }
        }
    });
    result
}

// scan for consecutive digits in a string, record the start and end index of each sequence
// including possibly lead and trailing char, and the number found
fn scan_for_numbers(s: &str) -> Vec<(usize, usize, u32)> {
    let mut result = Vec::new();
    let mut start = None;
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() {
            if start.is_none() {
                start = Some(i);
            }
        } else {
            if let Some(start) = start {
                // this can't fail because we know that the slice from start..i is all digits
                let number = (&s[start..i]).parse::<u32>().ok().unwrap();
                let start = start.saturating_sub(1);
                let end = i.saturating_add(1).min(s.len());
                result.push((start, end, number));
            }
            start = None;
        }
    }
    if let Some(start) = start {
        // this can't fail because we know that the slice from start.. is all digits
        let number = (&s[start..]).parse::<u32>().ok().unwrap();
        let start = start.saturating_sub(1);
        result.push((start, s.len(), number));
    }
    result
}

fn scan_for_gears(s: &str) -> Vec<usize> {
    let mut result = Vec::new();
    for (i, c) in s.char_indices() {
        if is_gear_char(c) {
            result.push(i);
        }
    }
    result
}

fn is_gear_char(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
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
    #[test]
    fn test_p2() {
        let mut f = BufReader::new(INPUT.as_bytes());
        assert_eq!(p2(&mut f), 467835);
    }
}
