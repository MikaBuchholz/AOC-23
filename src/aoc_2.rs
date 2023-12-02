use std::cmp;

use crate::util::read_input;

pub fn aoc_2() -> std::io::Result<i32> {
    let input = read_input("input2.txt")?;
    let games: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let mut sum = 0;
    //uncomment comments for part 1 of task
    /*'game:*/
    for game in games {
        let segments = game.split(':').collect::<Vec<&str>>();
        //let single_game_segments = segments[0].split(' ').collect::<Vec<&str>>();

        //let current_game = single_game_segments[1].parse::<i32>().unwrap();

        let sets = &segments[1..][0]
            .split([';'])
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        let mut green = 0;
        let mut red = 0;
        let mut blue = 0;

        for set in sets {
            let set_parts: Vec<_> = set.split(',').map(|s| s.trim()).collect();
            let set_parts: Vec<Vec<_>> = set_parts.iter().map(|v| v.split(' ').collect()).collect();
            let set_parts: Vec<(i32, &str)> = set_parts
                .iter()
                .map(|v| (v[0].parse::<i32>().unwrap(), v[1]))
                .collect();

            for (count, color) in set_parts {
                match color {
                    //comment out cmp::max(...) for part one of task
                    "green" => green = cmp::max(count, green), /* count */
                    "red" => red = cmp::max(count, red),       /* count */
                    "blue" => blue = cmp::max(count, blue),    /* count */
                    _ => unreachable!(),
                }

                //comment out for part 1 of task
                /*  if red > 12 || green > 13 ||  blue > 14 {
                    continue 'game;
                }
                */
            }
        }
        //switch to current_game for part 1;
        sum += green * red * blue /* current_game */;
    }

    Ok(sum)
}
