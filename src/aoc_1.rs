use crate::util::read_input;
use regex::bytes::Regex;

const valid: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

trait StringUtil {
    fn is_spelled(&self) -> bool;
    fn from_spelled(self) -> i32;
}

impl StringUtil for String {
    fn is_spelled(&self) -> bool {
        valid.contains(&self.as_str())
    }

    fn from_spelled(self) -> i32 {
        match self.as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => unreachable!(), //is_spelled should ensure this to be never reached
        }
    }
}

pub fn aoc_1() -> std::io::Result<()> {
    let input = read_input("input.txt")?;
    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();

    let mut sum = 0;

    for line in lines {
        let len = line.len();
        let s_bytes = line.as_bytes();

        let mut spelled_out_reserved = vec![0; len];
        let mut out = String::new();

        let mut pats = vec![];

        for pat in valid {
            let matches = line.match_indices(pat).collect::<Vec<_>>();

            if matches.is_empty() {
                continue;
            }

            pats.push(matches);
        }

        //println!("{pats:?}");

        for matches in pats {
            let (start, str) = matches[0];
            let end = start + str.len();

            let num = str.to_string().from_spelled();

            spelled_out_reserved[start..end].copy_from_slice(&vec![num; str.len()])
        }

        println!("{:?}", spelled_out_reserved);

        for index in 0..len {
            if spelled_out_reserved[index] != 0 {
                let str = &format!("{}", spelled_out_reserved[index]);
                out.push_str(str);
                break;
            }

            if s_bytes[index].is_ascii_digit() {
                out.push(s_bytes[index] as char);
                break;
            }
        }

        for index in (0..len).rev() {
            if spelled_out_reserved[index] != 0 {
                let str = &format!("{}", spelled_out_reserved[index]);
                out.push_str(str);
                break;
            }

            if s_bytes[index].is_ascii_digit() {
                out.push(s_bytes[index] as char);
                break;
            }
        }

        //println!("{helper}: {out} {:?}", spelled_out_reserved);
        if out.is_empty() {
            continue;
        }
        sum += out.parse::<i32>().unwrap();
    }

    println!("{sum}");

    Ok(())
}
