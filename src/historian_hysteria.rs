use crate::challenge::Challenge;
use std::collections::HashMap;

pub struct HistorianHysteria {
    name: &'static str,
}

impl HistorianHysteria {
    pub fn new() -> Self {
        HistorianHysteria {
            name: "historian_hysteria",
        }
    }
}

impl Challenge for HistorianHysteria {
    type Input = (Vec<i32>, Vec<i32>);
    type Output = i32;

    fn name(&self) -> &'static str {
        self.name
    }

    fn parse(&self, input: &[u8]) -> Self::Input {
        let input = String::from_utf8_lossy(input);

        let mut first_location_list: Vec<i32> = Vec::new();
        let mut second_location_list: Vec<i32> = Vec::new();

        input.lines().for_each(|line| {
            let mut locations = line.split_whitespace();
            first_location_list.push(locations.next().unwrap().parse().unwrap());
            second_location_list.push(locations.next().unwrap().parse().unwrap());
        });

        (first_location_list, second_location_list)
    }

    fn solve_part_1(&self, input: &Self::Input) -> Self::Output {
        let mut first_location_list = input.0.to_vec();
        let mut second_location_list = input.1.to_vec();

        if first_location_list.len() != second_location_list.len() {
            panic!("Both lists must have the same length");
        }

        first_location_list.sort();
        second_location_list.sort();

        first_location_list
            .iter()
            .zip(second_location_list.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    fn solve_part_2(&self, input: &Self::Input) -> Self::Output {
        let first_location_list = input.0.to_vec();
        let second_location_list = input.1.to_vec();

        let second_list_frequencies =
            second_location_list
                .iter()
                .fold(HashMap::new(), |mut acc, &item| {
                    *acc.entry(item).or_insert(0) += 1;
                    acc
                });

        first_location_list
            .iter()
            .map(|item| item * second_list_frequencies.get(item).unwrap_or(&0))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test_parse() {
        let input = b"3 4\n4 3\n2 5\n1 3\n3 9\n3 3\n";
        assert_eq!(
            HistorianHysteria::new().parse(input),
            (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
        );
    }

    #[test]
    fn example_test_solve_part_1() {
        let first_location_list = vec![3, 4, 2, 1, 3, 3];
        let second_location_list = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(
            HistorianHysteria::new()
                .solve_part_1(&(first_location_list.to_vec(), second_location_list.to_vec())),
            11
        );
    }

    #[test]
    fn example_test_solve_part_2() {
        let first_location_list = vec![3, 4, 2, 1, 3, 3];
        let second_location_list = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(
            HistorianHysteria::new()
                .solve_part_2(&(first_location_list.to_vec(), second_location_list.to_vec())),
            31
        );
    }
}
