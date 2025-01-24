//! Advent of Code 2024 Day 09
//! https://adventofcode.com/2024/day/9
//!
//! Challenge part 2
//!
//! The input is a sequence of numbers that represent files and empty space stored on disk blocks.
//! The challenge requires the disk blocks to be "compacted" by rearranging them so that each file,
//! starting with the last, is moved to the first block big enough to contain the entire file
//! starting at the beginning of the disk. When compaction is complete, a checksum is generated by
//! summing a value calculated for each used block from the block's position and the id of the file
//! it contains.

use std::cmp::Ordering;
use std::fs;

const INPUT_FILENAME: &str = "2024_day09_input.txt";

type FileId = u16;

#[derive(Clone, Copy, Debug, PartialEq)]
struct DiskBlock {
    size: u16,
    content: Option<FileId>,
}

impl DiskBlock {
    /// Returns a newly created `DiskBlock` containing the given size and data content.
    fn new(size: u16, content: Option<FileId>) -> Self {
        Self { size, content }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    println!(
        "The checksum over all compacted files is {}",
        do_challenge(&input)
    );
}

/// Reads the input into a disk map, compacts the files in this disk map following the process in
/// the challenge, and generates a checksum that is then returned.
fn do_challenge(input: &str) -> u64 {
    let mut disk_map = parse_input(input);
    compact_disk_map(&mut disk_map);
    generate_checksum(&disk_map)
}

/// Returns a `Vec` where each element is a disk block. The input is a sequence of numbers where
/// the numbers at odd positions represent files that use that number of contiguous disk blocks. The
/// numbers at even blocks represent unused (i.e., empty) disk blocks that are that number of
/// contiguous disk blocks in size.
///
/// Based on the input, each block is either in use by a file or empty. Empty is represented as
/// `None`, and used as `Some(FileId)`, where `FileId` is a numeric value given to a file, with the
/// `first file in the input being 0, the second 1, etc.
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Vec<DiskBlock> {
    let mut disk_map = Vec::new();

    let first_line: &str = input
        .lines()
        .collect::<Vec<_>>()
        .first()
        .expect("Error parsing input");

    for (index, c) in first_line.chars().enumerate() {
        let size =
            u16::try_from(c.to_digit(10).expect("Input must only consist of digits")).unwrap();

        let file_id = (index / 2) as FileId;

        if index % 2 == 0 {
            disk_map.push(DiskBlock::new(size, Some(file_id)));
        } else {
            disk_map.push(DiskBlock::new(size, None));
        }
    }

    disk_map
}

/// Compacts the given `disk_map` by looping through all files starting with the last and moving
/// a file in its entirety to the first empty block in the `disk_map` that is big enough to
/// accommodate it. If no such block exists, the file is not moved from its initial position.
///
/// # Panics
///
/// Panics if the given disk_map is empty.
fn compact_disk_map(disk_map: &mut Vec<DiskBlock>) {
    let highest_block_id = disk_map
        .last()
        .expect("Cannot compact an empty disk")
        .content
        .unwrap();

    for block_id in (0..=highest_block_id).rev() {
        let block_index = disk_map
            .iter()
            .position(|block| block.content == Some(block_id))
            .unwrap();
        let block_size = disk_map[block_index].size;

        for disk_index in 0..block_index {
            if let (free_size, None) = (disk_map[disk_index].size, disk_map[disk_index].content) {
                match free_size.cmp(&block_size) {
                    Ordering::Less => {}
                    Ordering::Greater => {
                        swap_blocks_and_merge(disk_map, disk_index, block_index);
                        break;
                    }
                    Ordering::Equal => {
                        swap_blocks_and_merge(disk_map, disk_index, block_index);
                        break;
                    }
                }
            }
        }
    }
}

// Swaps the data in the disk block at index `source` with the same size of free space at index
// `destination`. If the free space is bigger, a new block containing the remainder of the free
// space is inserted after the data block is swapped. All modified blocks containing free space
// are merged with adjacent free blocks.
//
// # Panics
//
// Panics if:
//     - `source` and `destination` indexes are the same
//     - the `source` block contains free space, not data
//     - the `destination` block contains data, not free space
//     - the `destination` block does not contain enough free space for the data block
fn swap_blocks_and_merge(disk_map: &mut Vec<DiskBlock>, destination: usize, source: usize) {
    assert!(source != destination, "Cannot swap a block with itself");
    assert!(
        disk_map[source].content.is_some(),
        "Only disk block containing data can be moved"
    );
    assert!(
        disk_map[destination].content.is_none(),
        "A disk block can only be moved to an empty block"
    );
    assert!(
        disk_map[destination].size >= disk_map[source].size,
        "Destination is too small"
    );

    let source_size = disk_map[source].size;
    let destination_size = disk_map[destination].size;

    disk_map.swap(destination, source);
    disk_map[source].size = source_size;

    // The `source` block has been replaced with an empty block. If the block following it is also
    // empty, merge the two.
    if source + 1 < disk_map.len() && disk_map[source + 1].content.is_none() {
        disk_map[source].size += disk_map[source + 1].size;
        disk_map.remove(source + 1);
    }

    // The `source` block has been replaced with an empty block. If the block preceding it is also
    // empty, merge the two.
    if source > 0 && disk_map[source - 1].content.is_none() {
        disk_map[source - 1].size += disk_map[source].size;
        disk_map.remove(source);
    }

    let partial_block = destination_size - source_size;
    if partial_block > 0 {
        if destination + 1 < disk_map.len() && disk_map[destination + 1].content.is_none() {
            disk_map[destination + 1].size += partial_block;
        } else {
            disk_map.insert(destination + 1, DiskBlock::new(partial_block, None));
        }
    }
}

/// Returns a checksum for the given `disk_map`. This is the sum from multiplying each non-empty
/// block's file id by its position in the `disk_map` (with the first block being 0).
fn generate_checksum(disk_map: &[DiskBlock]) -> u64 {
    disk_map
        .iter()
        .flat_map(|block| vec![block.content; block.size as usize])
        .enumerate()
        .map(|(position, data)| position as u64 * data.unwrap_or(0) as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_parse_input_short() {
        let disk_map = parse_input("12345");

        assert_eq!(
            vec![
                DiskBlock::new(1, Some(0)),
                DiskBlock::new(2, None),
                DiskBlock::new(3, Some(1)),
                DiskBlock::new(4, None),
                DiskBlock::new(5, Some(2)),
            ],
            disk_map
        );
    }

    #[test]
    fn test_parse_input() {
        let disk_map = parse_input(INPUT);

        assert_eq!(
            vec![
                DiskBlock::new(2, Some(0)),
                DiskBlock::new(3, None),
                DiskBlock::new(3, Some(1)),
                DiskBlock::new(3, None),
                DiskBlock::new(1, Some(2)),
                DiskBlock::new(3, None),
                DiskBlock::new(3, Some(3)),
                DiskBlock::new(1, None),
                DiskBlock::new(2, Some(4)),
                DiskBlock::new(1, None),
                DiskBlock::new(4, Some(5)),
                DiskBlock::new(1, None),
                DiskBlock::new(4, Some(6)),
                DiskBlock::new(1, None),
                DiskBlock::new(3, Some(7)),
                DiskBlock::new(1, None),
                DiskBlock::new(4, Some(8)),
                DiskBlock::new(0, None),
                DiskBlock::new(2, Some(9)),
            ],
            disk_map
        );
    }

    #[test]
    fn test_compact_disk_map_short() {
        let mut disk_map = parse_input("12345");
        compact_disk_map(&mut disk_map);

        assert_eq!(
            vec![
                DiskBlock::new(1, Some(0)),
                DiskBlock::new(2, None),
                DiskBlock::new(3, Some(1)),
                DiskBlock::new(4, None),
                DiskBlock::new(5, Some(2)),
            ],
            disk_map
        );
    }

    #[test]
    fn test_swap_blocks_and_merge_0() {
        let mut disk_map = vec![
            DiskBlock::new(5, Some(0)), // Index 0
            DiskBlock::new(1, None),    // Index 1
            DiskBlock::new(5, Some(1)), // Index 2
            DiskBlock::new(5, None),    // Index 3
            DiskBlock::new(5, Some(2)), // Index 4
            DiskBlock::new(9, None),    // Index 5
            DiskBlock::new(5, Some(3)), // Index 6
        ];

        swap_blocks_and_merge(&mut disk_map, 3, 6);

        assert_eq!(
            vec![
                DiskBlock::new(5, Some(0)), // Index 0
                DiskBlock::new(1, None),    // Index 1
                DiskBlock::new(5, Some(1)), // Index 2
                DiskBlock::new(5, Some(3)), // Index 3. Data copied here
                DiskBlock::new(5, Some(2)), // Index 4
                DiskBlock::new(14, None),   // Index 5
            ],
            disk_map
        );
    }

    #[test]
    fn test_swap_blocks_and_merge_1() {
        let mut disk_map = vec![
            DiskBlock::new(5, Some(0)), // Index 0
            DiskBlock::new(9, None),    // Index 1
            DiskBlock::new(5, Some(1)), // Index 2
            DiskBlock::new(5, None),    // Index 3
            DiskBlock::new(3, Some(2)), // Index 4
            DiskBlock::new(9, None),    // Index 5
            DiskBlock::new(5, Some(3)), // Index 6
        ];

        swap_blocks_and_merge(&mut disk_map, 1, 4);

        assert_eq!(
            vec![
                DiskBlock::new(5, Some(0)), // Index 0
                DiskBlock::new(3, Some(2)), // Index 1. Data copied here
                DiskBlock::new(6, None),    // Index 2. Remainder of block size 9
                DiskBlock::new(5, Some(1)), // Index 3. Was index 2
                DiskBlock::new(17, None),   // Index 4. Was indexes 3 and 5 plus gap of size 3
                DiskBlock::new(5, Some(3)), // Index 5.
            ],
            disk_map
        );
    }

    #[test]
    fn test_compact_disk_map() {
        let mut disk_map = parse_input(INPUT);
        compact_disk_map(&mut disk_map);

        assert_eq!(
            vec![
                DiskBlock::new(2, Some(0)),
                DiskBlock::new(2, Some(9)),
                DiskBlock::new(1, Some(2)),
                DiskBlock::new(3, Some(1)),
                DiskBlock::new(3, Some(7)),
                DiskBlock::new(1, None),
                DiskBlock::new(2, Some(4)),
                DiskBlock::new(1, None),
                DiskBlock::new(3, Some(3)),
                DiskBlock::new(4, None),
                DiskBlock::new(4, Some(5)),
                DiskBlock::new(1, None),
                DiskBlock::new(4, Some(6)),
                DiskBlock::new(5, None),
                DiskBlock::new(4, Some(8)),
                DiskBlock::new(2, None),
            ],
            disk_map
        );
    }

    #[test]
    fn test_do_challenge() {
        assert_eq!(2858, do_challenge(INPUT));
    }
}
