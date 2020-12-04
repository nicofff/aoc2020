use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn main() -> io::Result<()> {
    // Puzzle 1
    // let file = File::open("input.txt")?;
    // let reader = BufReader::new(file);

    // let expenses: Vec<isize> = reader.lines()
    // .map(|line|{
    //     line.unwrap().trim().parse::<isize>().unwrap()
    // })
    // .collect();

    // let expenses2 = expenses.clone();

    // for first_elem in &expenses {
    //     for second_elem in &expenses2 {
    //         if (first_elem + second_elem == 2020) {
    //             println!("{:?}",first_elem*second_elem);
    //             return Ok(());
    //         }
    //     }
    // }
        
    // Puzzle 2

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let expenses: Vec<isize> = reader.lines()
    .map(|line|{
        line.unwrap().trim().parse::<isize>().unwrap()
    })
    .collect();

    let expenses2 = expenses.clone();
    let expenses3 = expenses.clone();

    for first_elem in &expenses {
        for second_elem in &expenses2 {
            for third_element in &expenses3 {
                if (first_elem + second_elem + third_element == 2020) {
                    println!("{:?}",first_elem*second_elem* third_element);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}