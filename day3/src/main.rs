use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;
use std::num::ParseIntError;
use std::fs;


#[derive(Debug,PartialEq)]
enum Tile {
    Tree,
    Empty
}

#[derive(Debug)]
struct ForestMap {
    tiles: Vec<Vec<Tile>>,
}

impl FromStr for ForestMap {

    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles : Vec<Vec<Tile>> = Vec::new();
        for line in s.lines(){
            let mut line_vec : Vec<Tile> = Vec::new();
            for c in line.chars(){
                line_vec.push(
                    match c {
                        '#' => Tile::Tree,
                        '.' => Tile::Empty,
                        _ => unreachable!()
                    }
                )
            }
            tiles.push(line_vec);
        }
        Ok(
            ForestMap {
                tiles
            }
        )
    }
}

impl ForestMap {

    fn iter(&self,slope_x:usize,slope_y:usize) -> ForestSlopeIterator {
        ForestSlopeIterator {
            map: self,
            current_x:0,
            current_y:0,
            slope_x,
            slope_y
        }
    }
}

struct ForestSlopeIterator<'a> {
    map: &'a ForestMap,
    current_x: usize,
    current_y: usize,
    slope_x: usize,
    slope_y: usize
}

impl<'a> Iterator for ForestSlopeIterator<'a> {
    type Item = &'a Tile;
    fn next(&mut self) -> Option<&'a Tile> {
        self.current_x += self.slope_x;
        self.current_y += self.slope_y;
        if self.current_y >= self.map.tiles.len() {
            return None;
        }
        let row = &self.map.tiles[self.current_y];
        let tile = &row[self.current_x % row.len()];
        Some(tile)
    }
}


fn main() -> io::Result<()> {
    let map: ForestMap = ForestMap::from_str(&fs::read_to_string("input.txt")?).unwrap();
    
    //Puzzle 1
    // let tree_count: usize = map.iter(3, 1)
    // .filter(|tile| tile == &&Tile::Tree )
    // .count();
    // println!("{:?}",tree_count);
    //Ok(())

    // puzzle 2
    let slopes = vec![(1,1),(3,1),(5,1),(7,1),(1,2)];

    let mut result = 1;
    for slope in slopes {
        let tree_count: usize = map.iter(slope.0, slope.1)
        .filter(|tile| tile == &&Tile::Tree )
        .count();
        println!("{},{},{}",slope.0,slope.1,tree_count);
        result *= tree_count;
    }

    println!("{}",result);
    Ok(())
}