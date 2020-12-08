use std::io;
use std::io::prelude::*;

fn read_input() -> Vec<String>
{
    return io::stdin().lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
}

struct TreeMap
{
    map: Vec<Vec<char>>,
}

impl TreeMap
{
    fn new(map: Vec<String>) -> TreeMap
    {
        return TreeMap{
            map: map.iter().map(|s| s.chars().collect()).collect(),
        };
    }

    fn height(&self) -> usize
    {
        return self.map.len();
    }

    fn is_tree(&self, x: usize, y: usize) -> bool {

        if y >= self.map.len()
        {
            return false;
        }

        let row = &self.map[y];
        return row[x % row.len()] == '#';
    }
}

fn main() {

    let tree_map = TreeMap::new(read_input());

    println!("part 1: {}", (0..tree_map.height())
        .filter(|i| tree_map.is_tree((*i) * 3, *i))
        .count());

    println!("part 2: {:?}", [(1,1), (3,1), (5,1), (7,1), (1,2)].iter()
        .map(|(xf, yf)|
            (0..(tree_map.height()/yf))
                .filter(|i| tree_map.is_tree(xf * (*i), yf * (*i)))
                .count())
        .fold(1, |product, x| product * x));
}
