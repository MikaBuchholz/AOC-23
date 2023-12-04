use std::collections::HashMap;

use crate::util::read_input;

fn parse_numbers(input: &str) -> Vec<i32> {
    let mut out = vec![];

    input
        .split(' ')
        .filter(|s| !s.is_empty())
        .for_each(|s| out.push(s.parse::<i32>().unwrap()));

    out
}

fn parse_input(input: String) -> Vec<(Vec<i32>, Vec<i32>)> {
    let mut out = vec![];

    let lines = input.lines().collect::<Vec<_>>();
    let numbers: Vec<_> = lines
        .iter()
        .map(|l| l.split(':').collect::<Vec<_>>().remove(1))
        .collect();

    let number_groups: Vec<_> = numbers
        .iter()
        .map(|n| {
            let number_groups = n.split('|').collect::<Vec<_>>();

            (number_groups[0].trim(), number_groups[1].trim())
        })
        .collect();

    for (winner_numbers, my_numbers) in number_groups {
        let parsed_winners = parse_numbers(winner_numbers);
        let my_parsed_numbers = parse_numbers(my_numbers);

        out.push((parsed_winners, my_parsed_numbers))
    }

    out
}

fn count_winners(winning_numbers: &[i32], my_numbers: &[i32]) -> usize {
    my_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count()
}

pub fn aoc_4_part_1() -> std::io::Result<i32> {
    let input = read_input("input4.txt")?;
    let parsed_input = parse_input(input);

    let mut sum = 0;

    for (winner_numbers, my_numbers) in parsed_input {
        let winning_count = count_winners(&winner_numbers, &my_numbers);

        if winning_count == 0 {
            continue;
        }

        let mut points = 1;

        for _ in 0..winning_count - 1 {
            points *= 2;
        }

        sum += points
    }

    Ok(sum)
}

pub fn aoc_4_part_2() -> std::io::Result<i32> {
    let input = read_input("input4.txt")?;
    let parsed_input = parse_input(input);
    let parsed_input: Vec<(usize, Vec<i32>, Vec<i32>)> = parsed_input
        .iter()
        .enumerate()
        .map(|(i, l)| (i, l.0.clone(), l.1.clone()))
        .collect::<Vec<_>>();

    let mut win_lookup = HashMap::new();

    for (card_num, wins, my_nums) in parsed_input.clone() {
        let win_c = count_winners(&wins, &my_nums);
        win_lookup.insert(card_num, win_c);
    }

    let mut win_track = vec![1; parsed_input.len()];

    //creds Ozzy30 for helping
    for (c, ..) in parsed_input.clone() {
        let win_count = win_lookup.get(&c).unwrap();

        if win_count == &0 {
            continue;
        };

        for i in c + 1..=c + win_count {
            win_track[i] += win_track[c];
        }
    }

    Ok(win_track.iter().sum::<i32>())
}
