use std::{collections::HashMap, future::IntoFuture};

use crate::util::read_input;

trait Util {
    fn is_symbol(&self) -> bool;
}

impl Util for char {
    fn is_symbol(&self) -> bool {
        self != &'.' && !self.is_numeric()
    }
}

pub fn aoc_3_part_1() -> std::io::Result<i32> {
    let input = read_input("input3.txt")?;
    let lines: Vec<&str> = input.lines().collect();

    let mut indexes = vec![];
    let mut number = String::new();

    let mut sum = 0;

    for (row, line) in lines.clone().into_iter().enumerate() {
        for (column, token) in line.char_indices() {
            if token.is_numeric() {
                number.push(token);
                indexes.push(column);
            }

            if !token.is_numeric() && !number.is_empty() {
                let parsed_number = number.parse::<i32>().unwrap();

                let last = indexes.last().unwrap();
                let first = indexes.first().unwrap();

                if let Some(left) = first.checked_sub(1) {
                    //left first digit in number
                    if (lines[row].as_bytes()[left] as char).is_symbol() {
                        sum += parsed_number;
                        number.clear();
                        indexes.clear();
                        continue;
                    }
                }

                if last + 1 < line.len() {
                    //right of last digit in number
                    if (lines[row].as_bytes()[last + 1] as char).is_symbol() {
                        sum += parsed_number;
                        number.clear();
                        indexes.clear();
                        continue;
                    }
                }

                for index in indexes.clone() {
                    if let Some(last_row_index) = row.checked_sub(1) {
                        //char above each digit in number
                        let char_above = lines[last_row_index].as_bytes()[index] as char;

                        if char_above.is_symbol() {
                            sum += parsed_number;
                            break;
                        }

                        if let Some(left_side_index) = index.checked_sub(1) {
                            //top left corner of first digit
                            let top_left =
                                lines[last_row_index].as_bytes()[left_side_index] as char;

                            if top_left.is_symbol() {
                                sum += parsed_number;
                                break;
                            }
                        }

                        if index + 1 < line.len() {
                            //top right corner of last digit
                            let top_right = lines[last_row_index].as_bytes()[index + 1] as char;

                            if top_right.is_symbol() {
                                sum += parsed_number;
                                break;
                            }
                        }
                    }

                    if row + 1 < lines.len() {
                        //bellow each digit in number
                        let char_bellow = lines[row + 1].as_bytes()[index] as char;

                        if char_bellow.is_symbol() {
                            sum += parsed_number;
                            break;
                        }

                        if let Some(left_side_index) = index.checked_sub(1) {
                            //bottom left of first digit
                            let bottom_left = lines[row + 1].as_bytes()[left_side_index] as char;

                            if bottom_left.is_symbol() {
                                sum += parsed_number;
                                break;
                            }
                        }

                        if index + 1 < line.len() {
                            //bottom right of last digit
                            let bottom_right = lines[row + 1].as_bytes()[index + 1] as char;

                            if bottom_right.is_symbol() {
                                sum += parsed_number;
                                break;
                            }
                        }
                    }
                }

                number.clear();
                indexes.clear()
            }
        }
    }

    Ok(sum)
}

fn neighbours(row: usize, col: usize, len: usize, line_len: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    for i in (row.saturating_sub(1))..(usize::min(row + 2, len)) {
        for j in (col.saturating_sub(1))..(usize::min(col + 2, line_len)) {
            if i != row || j != col {
                neighbours.push((i, j));
            }
        }
    }

    neighbours
}

pub fn aoc_3_part_2() -> std::io::Result<()> {
    let input = read_input("input3.txt")?;
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;

    let mut num_indexes = vec![];

    let mut indexes = vec![];

    for (row, line) in lines.iter().enumerate() {
        for (col, token) in line.chars().enumerate() {
            if token.is_numeric() {
                indexes.push(col)
            } else if !indexes.is_empty() {
                let num: String = indexes
                    .iter()
                    .map(|i| lines[row].as_bytes()[*i] as char)
                    .collect();

                match num.parse::<i32>() {
                    Ok(_) => {
                        num_indexes.push((row, indexes.clone()));
                        indexes.clear()
                    }
                    Err(_) => {
                        println!("{num}");
                        indexes.clear()
                    }
                }
            }
        }
    }

    let mut gear_numbers = vec![];
    for (row, line) in lines.iter().enumerate() {
        for (col, token) in line.chars().enumerate() {
            if token == '*' {
                let neighbours = neighbours(row, col, lines.len(), line.len());
                let mut seen = vec![];

                for (row, col) in neighbours {
                    let adjecent_nums = num_indexes
                        .iter()
                        .filter(|x| x.0 == row && x.1.contains(&col))
                        .collect::<Vec<_>>();

                    if !adjecent_nums.is_empty() && !seen.contains(&adjecent_nums) {
                        seen.push(adjecent_nums.clone());

                        let (row, indexes) = &adjecent_nums[0];

                        let num: String = indexes
                            .iter()
                            .map(|i| lines[*row].as_bytes()[*i] as char)
                            .collect();

                        match num.parse::<i32>() {
                            Ok(num) => gear_numbers.push(num),
                            Err(_) => {
                                println!("{row}:{num}");
                                println!("{:?}", adjecent_nums);
                            }
                        };
                    }
                }
            }
        }

        if gear_numbers.len() == 2 {
            sum += gear_numbers[0] * gear_numbers[1];
        }
        gear_numbers.clear()
    }

    Ok(())
}
