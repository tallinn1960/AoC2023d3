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
    let parsed_schema = std::iter::once("".to_string())
        .chain(schema.lines().map(|r| r.expect("line read failed")))
        .chain(std::iter::once("".to_string()))
        .map(|s| (scan_for_gears(&s, is_symbol_char), scan_for_numbers(&s)))
        .collect::<Vec<_>>();
    parsed_schema.windows(3).for_each(|three_line_group| {
        for (start, end, number) in &three_line_group[1].1 {
            if three_line_group.iter().any(|(gears, _)| {
                gears
                    .iter()
                    .any(|&gear_pos| gear_pos >= *start && gear_pos < *end)
            }) {
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
        .map(|s| (scan_for_gears(&s, is_gear_char), scan_for_numbers(&s)))
        .collect::<Vec<_>>();
    parsed_schema.windows(3).for_each(|three_line_group| {
        for &gear_pos in &three_line_group[1].0 {
            let found = three_line_group
                .iter()
                .flat_map(|line| &line.1)
                .filter_map(|(start, end, number)| {
                    if gear_pos >= *start && gear_pos < *end {
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
    let mut enumerated_chars = s.char_indices();
    let mut result = Vec::new();
    while let Some((i, c)) = enumerated_chars.by_ref().next() {
        if c.is_ascii_digit() {
            let start = i;
            let end = enumerated_chars
                .by_ref()
                .skip_while(|(_, c)| c.is_ascii_digit())
                .next()
                .map(|(i, _)| i)
                .unwrap_or(s.len());
            // this is safe because we know that the slice is a number
            let number = s[start..end].parse::<u32>().ok().unwrap();
            result.push((
                start.saturating_sub(1),
                min(end.saturating_add(1), s.len()),
                number,
            ));
        }
    }
    result
}

fn scan_for_gears<F: Fn(char) -> bool>(s: &str, is_a_match: F) -> Vec<usize> {
    s.char_indices()
        .filter_map(|(i, c)| if is_a_match(c) { Some(i) } else { None })
        .collect()
}

fn is_gear_char(c: char) -> bool {
    c == '*'
}

fn is_symbol_char(c: char) -> bool {
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
