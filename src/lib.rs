use std::{cmp::min, io::BufRead};

pub fn p1<F: BufRead>(schema: &mut F) -> u32 {
    let mut result = 0;
    let parsed_schema = parser(schema, |s| {
        (scan_for_gears(s, is_symbol_char), scan_for_numbers(s))
    })
    .collect::<Vec<_>>();
    for three_line_group in parsed_schema.windows(3) {
        for (start, end, number) in &three_line_group[1].1 {
            if three_line_group.iter().any(|(gears, _)| {
                gears
                    .iter()
                    .any(|&gear_pos| gear_pos >= *start && gear_pos < *end)
            }) {
                result += number;
            }
        }
    }
    result
}

pub fn p2<F: BufRead>(schema: &mut F) -> u32 {
    let mut result = 0;
    let parsed_schema = parser(schema, |s| {
        (scan_for_gears(s, is_gear_char), scan_for_numbers(s))
    })
    .collect::<Vec<_>>();
    for three_line_group in parsed_schema.windows(3) {
        for &gear_pos in &three_line_group[1].0 {
            let mut count = 0;
            let product = three_line_group
                .iter()
                .flat_map(|line| &line.1)
                .filter_map(|(start, end, number)| {
                    if gear_pos >= *start && gear_pos < *end {
                        Some(*number)
                    } else {
                        None
                    }
                })
                .inspect(|_| count += 1)
                .product::<u32>();
            if count > 1 {
                result += product;
            }
        }
    }
    result
}

fn parser<'a, F, E, P>(schema: &'a mut F, line_parser: P) -> impl Iterator<Item = E> + 'a
where
    F: BufRead,
    P: Fn(&str) -> E + 'a,
    E: Default + 'a,
{
    // We want every parsed result of a line to have a predecessor and a successor
    // thus we add default results as the first and last element.
    // We use default result elements so the line_parser does not need to handle
    // empty lines.
    std::iter::once(E::default())
        .chain(
            schema
                .lines()
                .map(|r| r.expect("line read failed"))
                .map(move |s| line_parser(&s)),
        )
        .chain(std::iter::once(E::default()))
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
                .find(|(_, c)| !c.is_ascii_digit())
                .map(|(i, _)| i)
                .unwrap_or(s.len());
            // the slice is a number, but let's protect against numbers
            // which are too big for u32, which we ignore
            if let Ok(number) = s[start..end].parse::<u32>() {
                result.push((
                    start.saturating_sub(1),
                    min(end.saturating_add(1), s.len()),
                    number,
                ));
            };
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
    use std::io::BufReader;

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
