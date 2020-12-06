#![feature(min_const_generics)]

use std::io::prelude::*;
use std::io;

fn read_input() -> Vec<i64>
{
    return io::stdin().lock()
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
}

struct IndexPermutationSeq<const INDEX_COUNT: usize>
{
    seq_length: usize,
    curr: [usize; INDEX_COUNT],
}

impl<const INDEX_COUNT: usize> Iterator for IndexPermutationSeq<INDEX_COUNT>
{
    type Item = [usize; INDEX_COUNT];
    
    fn next(&mut self) -> Option<[usize; INDEX_COUNT]>
    {
        if self.seq_length < INDEX_COUNT
        {
            return None;
        }
    
        let base_max = self.seq_length - INDEX_COUNT;
    
        if self.curr[0] >= base_max
        {
            return None;
        }
        
        for i in 1..INDEX_COUNT
        {
    		if self.curr[i] >= i + base_max
    		{
    			self.curr[i - 1] += 1;
    			for j in i..INDEX_COUNT
    			{
    				self.curr[j] = self.curr[j - 1] + 1;
    			}
    			return Some(self.curr);
    		}
    	}
    	
    	self.curr[INDEX_COUNT - 1] += 1;
        return Some(self.curr);
    }
}

fn index_permutations<const INDEX_COUNT: usize>(seq_length: usize)
    -> impl Iterator<Item=[usize; INDEX_COUNT]>
{
    // todo(ts): When seq_length == INDEX_COUNT the iterator should return a single
    // element of [0..INDEX_COUNT] but returns an empty iterator instead.

    let mut start: [usize; INDEX_COUNT] = [0; INDEX_COUNT];
    for i in 0..INDEX_COUNT
    {
        start[i] = i;
    }
    
    start[INDEX_COUNT - 1] -= 1;
    
    return IndexPermutationSeq::<INDEX_COUNT>{ seq_length, curr: start };
}

const SUM: i64 = 2020;

fn main()
{

    let input = read_input();
    for [x,y] in index_permutations::<2>(input.len())
    {
        if input[x] + input[y] == SUM
        {
            println!("{}", input[x] * input[y]);
        }
    }
}