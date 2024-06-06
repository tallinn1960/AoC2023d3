use std::{collections::HashSet, io::BufRead};

struct PartNumber {
    value: i64,
    points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(row: i64, col: i64, ch: char) -> Self {
        let points = HashSet::from([
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ]);
        Self {
            value: ch.to_digit(10).unwrap() as i64,
            points,
        }
    }

    fn add_digit(&mut self, row: i64, col: i64, ch: char) {
        self.value = self.value * 10 + ch.to_digit(10).unwrap() as i64;
        self.points
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)])
    }
}

pub fn p1<F: BufRead>(f: F) -> i64 {
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut syms = HashSet::new();
    for (row, line) in f.lines().enumerate() {
        let line = line.expect("failed to read line");
        let mut current_number: Option<PartNumber> = None;
        for (col, ch) in line.char_indices() {
            if ch.is_ascii_digit() {
                if let Some(ref mut part_number) = current_number {
                    part_number.add_digit(row as i64, col as i64, ch);
                } else {
                    current_number = Some(PartNumber::new(row as i64, col as i64, ch));
                }
            } else {
                if let Some(part_number) = current_number.take() {
                    part_numbers.push(part_number);
                };
                if ch != '.' {
                    syms.insert((row as i64, col as i64));
                }
            }
        }
    }
    part_numbers
        .iter()
        .filter(|part_number| part_number.points.intersection(&syms).next().is_some())
        .map(|part_number| part_number.value)
        .sum()
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
}
