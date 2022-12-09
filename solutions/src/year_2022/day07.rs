use std::collections::HashMap;

use aoc::*;

#[derive(Debug, Date)]
#[date(year = 2022, day = 7)]
pub struct Day07;

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> AocResult {
        let sizes = dir_sizes(input);

        let sum: usize = sizes
            .into_iter()
            .map(|(_, size)| size)
            .filter(|size| *size < 10_0000)
            .sum();

        Ok(Box::new(sum))
    }

    fn part_two(&self, input: &str) -> AocResult {
        let sizes = dir_sizes(input);
        let free = 70_000_000 - sizes.get(&vec!["/"]).unwrap_or(&0);

        let smallest_dir_to_delete = dir_sizes(input)
            .into_iter()
            .map(|(_, size)| size)
            .filter(|size| free + size >= 30_000_000)
            .min()
            .unwrap_or_default();

        Ok(Box::new(smallest_dir_to_delete))
    }
}

fn dir_sizes(input: &str) -> HashMap<Vec<&str>, usize> {
    let mut dirs = vec![];
    let mut sizes = HashMap::new();
    for line in input.lines() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "cd", ".."] => {
                dirs.pop();
            }
            ["$", "cd", dir] => dirs.push(dir),
            [maybe_size, _] => {
                if let Ok(size) = maybe_size.parse::<usize>() {
                    for i in 0..dirs.len() {
                        *sizes.entry(dirs[0..=i].to_vec()).or_insert(0) += size;
                    }
                }
            }
            _ => (),
        }
    }
    sizes
}

#[test]
fn test() {
    let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    assert_solution!(Day07.part_one, input, "95437");
    assert_solution!(Day07.part_two, input, "24933642");
}
