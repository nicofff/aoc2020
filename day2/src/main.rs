use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;


#[derive(Debug)]
struct PasswordSample {
    min: usize,
    max: usize,
    letter: char,
    password: String
}

impl FromStr for PasswordSample {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 13-15 p: pppppppvppppxxppp

        let parts: Vec<&str> = s.split(' ').collect();
        let min_max: Vec<usize> = parts[0].split('-')
            .map(|line|{
                line.trim().parse::<usize>().unwrap()
            }).take(2).collect();

        let min = min_max[0];
        let max = min_max[1];
        let letter = parts[1].chars().nth(0).unwrap();
        let password = parts[2].to_owned();

        Ok(PasswordSample {
            min,
            max,
            letter,
            password
        })
    }

}

fn main() -> io::Result<()> {
    //Puzzle 1
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let valid_passwords = reader.lines()
    .map(|line|{
        let line = line.unwrap();
        PasswordSample::from_str(&line).unwrap()
    })
    .filter(|pass|{
        let ocurrences = pass.password.matches(pass.letter).count();
        return ocurrences >= pass.min && ocurrences <= pass.max
    }).count();

    println!("{:?}",valid_passwords);

    // Puzzle 2

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let valid_passwords = reader.lines()
    .map(|line|{
        let line = line.unwrap();
        PasswordSample::from_str(&line).unwrap()
    })
    .filter(|pass|{
        let first_match = pass.password.chars().nth(pass.min-1).unwrap();
        let second_match = pass.password.chars().nth(pass.max-1).unwrap();
        let mut matches = 0;
        matches += if first_match == pass.letter {
            1
        }else {
            0
        };

        matches += if second_match == pass.letter {
            1
        }else {
            0
        };

        matches == 1
    }).count();

    println!("{:?}",valid_passwords);

    Ok(())
}