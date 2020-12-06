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
    
        // For cases where seq_length > INDEX_COUNT it would be sufficient to
        // only check the first index because the increment that updates the
        // first index to its max value is the final increment. In the case
        // where seq_length == INDEX_COUNT however the first value emitted is
        // also the final/only value so the first index is already at its max.
        // To handle this case we check last index as well because the initial
        // state of the struct has the last index set to 1 less than it's min
        // (and in this case max) value.
        if self.curr[0] >= base_max &&
            self.curr[INDEX_COUNT - 1] >= base_max + INDEX_COUNT - 1
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
    let mut start: [usize; INDEX_COUNT] = [0; INDEX_COUNT];
    for i in 0..INDEX_COUNT
    {
        start[i] = i;
    }
    
    // Start in a state where the increment algorithm will increment to the
    // first value. We could start on the first value and have the iterator impl
    // return the pre-increment value instead, but it's easier to detect that
    // curr IS final value and return None than to detect/track that the final
    // value has already been returned.
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
            println!("part 1: {}", input[x] * input[y]);
        }
    }

    for [x,y,z] in index_permutations::<3>(input.len())
    {
        if input[x] + input[y] + input[z] == SUM
        {
            println!("part 2: {}", input[x] * input[y] * input[z]);
        }
    }
}