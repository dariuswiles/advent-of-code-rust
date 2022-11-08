//! Advent of Code 2021 Day 20
//! https://adventofcode.com/2021/day/20
//!
//! Challenge part 2
//!
//! Display the number of light pixels after enhancing a given image with a given enhancement
//! algorithm 50 times.

use std::collections::HashSet;
use std::fmt;
use std::fs;

const INPUT_FILENAME: &str = "2021_day20_input.txt";
const IMAGE_ENHANCEMENT_LEN: usize = 512;
const DARK: char = '.';
const LIGHT: char = '#';
const ENHANCEMENT_ITERATIONS: usize = 50;

type PositionInt = i32;

/// `Position` is a tuple of (row, column).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position(PositionInt, PositionInt);

#[derive(Clone, Debug, PartialEq)]
struct ImageEnhancementAlgorithm {
    data: Vec<char>,
}

impl ImageEnhancementAlgorithm {
    fn from_string(input: &str) -> Self {
        let data: Vec<char> = input.chars().collect();
        assert_eq!(data.len(), IMAGE_ENHANCEMENT_LEN);
        Self { data }
    }
}

/// Holds a representation of an image. Individual pixels are identified by their `Position`,
/// consisting of a row and column. The first character in the input used to create an `Image`
/// is position (0, 0). Rows and columns are allowed to be negative to accommodate the challenge
/// requirement that an image can expand in all directions as processing is performed.
#[derive(Clone, Debug, PartialEq)]
struct Image {
    light_pixels: HashSet<Position>,
    initial_size: usize,
    enhancement_count: usize,
}

impl Image {
    /// Returns an `Image` object created from the `input` provided.
    ///
    /// # Panics
    ///
    /// Panics if the input contains unrecognized characters or is not square, i.e., the number of
    /// lines does not exactly match the number of characters on every line.
    fn from_string(input: &str) -> Self {
        let mut light_pixels = HashSet::new();
        let mut size = None;
        let mut row_num: usize = 0;

        for line in input.lines() {
            if line == "" {
                continue;
            }

            let chars: Vec<char> = line.chars().collect();

            if let Some(line_length) = size {
                if chars.len() != line_length {
                    panic!("All image data lines must be the same length, but are not.");
                }
            } else {
                size = Some(chars.len());
            }

            for (col_num, pixel) in chars.iter().enumerate() {
                match pixel {
                    &LIGHT => {
                        light_pixels
                            .insert(Position(row_num as PositionInt, col_num as PositionInt));
                    }
                    &DARK => {}
                    _ => {
                        panic!(
                            "Row {} of the image contains unknown character '{}'",
                            row_num, pixel
                        )
                    }
                };
            }

            row_num += 1;
        }

        if row_num != size.unwrap() {
            panic!("Image data must be square, but is not.");
        }

        Self {
            light_pixels,
            initial_size: size.unwrap(),
            enhancement_count: 0,
        }
    }

    /// Returns a tuple containing two `Position`s. The first holds the lowest row number with a
    /// light pixel, and the lowest column number with a light pixel. The second is similar but for
    /// the highest row and column. This gives the limits of all light pixels.
    fn get_light_pixel_limits(&self) -> (Position, Position) {
        let mut row_min = PositionInt::MAX;
        let mut row_max = PositionInt::MIN;
        let mut col_min = PositionInt::MAX;
        let mut col_max = PositionInt::MIN;

        for p in &self.light_pixels {
            row_min = row_min.min(p.0);
            row_max = row_max.max(p.0);
            col_min = col_min.min(p.1);
            col_max = col_max.max(p.1);
        }

        (Position(row_min, col_min), Position(row_max, col_max))
    }

    /// Returns a number that is the binary representation of the 3x3 grid centered on `p`. Each
    /// pixel in this image is considered a binary '1' if light, or '0' if dark. As there are 9
    /// pixels, the range of the output is 0..=512. `outside_char` is the default value that should
    /// be used for pixels outside the square of pixels that have been explicitly enhanced so far.
    /// The boundary of this square is determined by the `Image`'s initial size and the number of
    /// times it has been enhanced, both of which are stored in its fields.
    fn get_3x3(&self, p: &Position, outside_char: char) -> usize {
        let Position(row, col) = *p;
        let mut output = 0;
        let init_size = self.initial_size as PositionInt;
        let iteration = self.enhancement_count as PositionInt;

        for r in row - 1..=row + 1 {
            for c in col - 1..=col + 1 {
                output <<= 1;

                if r < -iteration
                    || r >= init_size + iteration
                    || c < -iteration
                    || c >= init_size + iteration
                {
                    if outside_char == LIGHT {
                        output += 1;
                    }
                    continue;
                }

                if self.light_pixels.get(&Position(r, c)).is_some() {
                    output += 1;
                }
            }
        }
        output
    }

    /// Returns the "enhanced" value of the pixel at `Position` 'p', following the steps in the
    /// challenge. `outside_char` is the default value that should be used for pixels outside the
    /// square of pixels that have been explicitly enhanced so far.
    fn enhance_pixel(
        &self,
        p: &Position,
        algo: &ImageEnhancementAlgorithm,
        outside_char: char,
    ) -> char {
        algo.data[self.get_3x3(p, outside_char)]
    }

    /// Returns a new, enhanced version of this image.
    fn enhance(&self, algo: &ImageEnhancementAlgorithm) -> Self {
        let mut light_pixels = HashSet::new();
        let iteration = self.enhancement_count as PositionInt;

        // Determine if the pixels outside the image we have enhanced so far are light or dark.
        // These extend to infinity in all directions. If index 0 of the
        // ImageEnhancementAlgorithm` is DARK, the outside pixels stay dark every iteration. If it
        // is LIGHT, all outside pixels switch to LIGHT on the first iteration. If the *last* pixel
        // is also LIGHT, the outside pixels remain light for all further iterations. If its DARK,
        // outside pixels are LIGHT on odd iterations and DARK on even iterations.
        let mut outside = DARK;
        if algo.data[0] == LIGHT {
            if algo.data[IMAGE_ENHANCEMENT_LEN - 1] == LIGHT {
                outside = LIGHT;
            } else {
                if self.enhancement_count % 2 == 1 {
                    outside = LIGHT;
                }
            }
        }

        for row in -iteration - 1..=self.initial_size as PositionInt + iteration {
            for col in -iteration - 1..=self.initial_size as PositionInt + iteration {
                let p = Position(row, col);

                if self.enhance_pixel(&p, algo, outside) == LIGHT {
                    light_pixels.insert(p);
                }
            }
        }

        Self {
            light_pixels,
            initial_size: self.initial_size,
            enhancement_count: self.enhancement_count + 1,
        }
    }

    /// Runs the image enhancement algorithm `iteration` times and returns a new `Image` containing
    /// the result.
    fn enhance_repeatedly(&self, algo: &ImageEnhancementAlgorithm, iterations: usize) -> Image {
        let mut current: Self = self.clone();
        for _ in 0..iterations {
            current = current.enhance(&algo);
        }
        current
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (Position(top, left), Position(bottom, right)) = self.get_light_pixel_limits();

        for row in top..=bottom {
            for col in left..=right {
                let p = Position(row, col);

                if self.light_pixels.get(&p).is_some() {
                    let _ = write!(f, "#");
                } else {
                    let _ = write!(f, ".");
                }
            }
            let _ = writeln!(f);
        }
        writeln!(f)
    }
}

/// Returns `ImageEnhancementAlgorithm` and `Image` objects as a tuple, representing the data
/// passed in `input`.
fn parse_input(input: &str) -> (ImageEnhancementAlgorithm, Image) {
    let lines: Vec<&str> = input.lines().collect();
    let enhancement = ImageEnhancementAlgorithm::from_string(lines.first().unwrap());

    let image = Image::from_string(&lines[1..].join("\n"));

    (enhancement, image)
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let (enhancement, image0) = parse_input(&input_file);
    let image50 = image0.enhance_repeatedly(&enhancement, ENHANCEMENT_ITERATIONS);

    println!(
        "The enhanced image has {} light pixels",
        image50.light_pixels.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_enhancement_from_string() {
        let enhancement = ImageEnhancementAlgorithm::from_string(
            &TEST_INPUT.lines().collect::<Vec<&str>>().first().unwrap(),
        );

        assert_eq!(enhancement.data.len(), IMAGE_ENHANCEMENT_LEN);
        assert_eq!(enhancement.data[0..2], vec!['.', '.']);
        assert_eq!(enhancement.data[74..76], vec!['#', '.']);
        assert_eq!(enhancement.data[148..150], vec!['.', '#']);
        assert_eq!(enhancement.data[510..512], vec!['.', '#']);
    }

    #[test]
    fn test_image_from_string() {
        let image = Image::from_string(&TEST_INPUT.lines().collect::<Vec<&str>>()[1..].join("\n"));

        assert_eq!(image.light_pixels.len(), 10);
        assert!(image.light_pixels.contains(&Position(0, 0)));
        assert!(image.light_pixels.contains(&Position(0, 3)));
        assert!(image.light_pixels.contains(&Position(1, 0)));
        assert!(image.light_pixels.contains(&Position(2, 0)));
        assert!(image.light_pixels.contains(&Position(2, 1)));
        assert!(image.light_pixels.contains(&Position(2, 4)));
        assert!(image.light_pixels.contains(&Position(3, 2)));
        assert!(image.light_pixels.contains(&Position(4, 2)));
        assert!(image.light_pixels.contains(&Position(4, 3)));
        assert!(image.light_pixels.contains(&Position(4, 4)));
    }

    #[test]
    #[should_panic]
    fn test_malformed_image() {
        Image::from_string("###\n...\n#\n");
    }

    #[test]
    fn test_image_limits() {
        let image = Image::from_string(&TEST_INPUT.lines().collect::<Vec<&str>>()[1..].join("\n"));

        let limits = image.get_light_pixel_limits();

        assert_eq!(limits.0, Position(0, 0));
        assert_eq!(limits.1, Position(4, 4));
    }

    #[test]
    fn test_get_3x3() {
        let image = Image::from_string(&TEST_INPUT.lines().collect::<Vec<&str>>()[1..].join("\n"));

        assert_eq!(image.get_3x3(&Position(2, 2), DARK), 34);
    }

    #[test]
    fn test_get_3x3_outside_dark() {
        let image = Image::from_string(&TEST_INPUT.lines().collect::<Vec<&str>>()[1..].join("\n"));

        assert_eq!(image.get_3x3(&Position(200, 200), DARK), 0);
    }

    #[test]
    fn test_get_3x3_outside_light() {
        let image = Image::from_string(&TEST_INPUT.lines().collect::<Vec<&str>>()[1..].join("\n"));

        assert_eq!(
            image.get_3x3(&Position(200, 200), LIGHT),
            IMAGE_ENHANCEMENT_LEN - 1
        );
    }

    #[test]
    fn test_enhance_pixel() {
        let (enhancement, image) = parse_input(&TEST_INPUT);
        let result = image.enhance_pixel(&Position(2, 2), &enhancement, DARK);

        assert_eq!(result, LIGHT);
    }

    #[test]
    fn test_enhance_1() {
        let (enhancement, image0) = parse_input(&TEST_INPUT);
        let image1 = image0.enhance(&enhancement);

        assert_eq!(image1.light_pixels.len(), 24);
        assert!(image1.light_pixels.contains(&Position(-1, 0)));
        assert!(image1.light_pixels.contains(&Position(-1, 1)));
        assert!(image1.light_pixels.contains(&Position(-1, 3)));
        assert!(image1.light_pixels.contains(&Position(-1, 4)));
        assert!(image1.light_pixels.contains(&Position(0, -1)));
        assert!(image1.light_pixels.contains(&Position(0, 2)));
        assert!(image1.light_pixels.contains(&Position(0, 4)));
        assert!(image1.light_pixels.contains(&Position(1, -1)));
        assert!(image1.light_pixels.contains(&Position(1, 0)));
        assert!(image1.light_pixels.contains(&Position(1, 2)));
        assert!(image1.light_pixels.contains(&Position(1, 5)));
        assert!(image1.light_pixels.contains(&Position(2, -1)));
        assert!(image1.light_pixels.contains(&Position(2, 0)));
        assert!(image1.light_pixels.contains(&Position(2, 1)));
        assert!(image1.light_pixels.contains(&Position(2, 2)));
        assert!(image1.light_pixels.contains(&Position(2, 5)));
        assert!(image1.light_pixels.contains(&Position(3, 0)));
        assert!(image1.light_pixels.contains(&Position(3, 3)));
        assert!(image1.light_pixels.contains(&Position(3, 4)));
        assert!(image1.light_pixels.contains(&Position(4, 1)));
        assert!(image1.light_pixels.contains(&Position(4, 2)));
        assert!(image1.light_pixels.contains(&Position(4, 5)));
        assert!(image1.light_pixels.contains(&Position(5, 2)));
        assert!(image1.light_pixels.contains(&Position(5, 4)));
    }

    #[test]
    fn test_enhance_2() {
        let (enhancement, image0) = parse_input(&TEST_INPUT);
        let image2 = image0.enhance(&enhancement).enhance(&enhancement);

        assert_eq!(image2.light_pixels.len(), 35);
        assert!(image2.light_pixels.contains(&Position(-2, 5)));
        assert!(image2.light_pixels.contains(&Position(-1, -1)));
        assert!(image2.light_pixels.contains(&Position(-1, 2)));
        assert!(image2.light_pixels.contains(&Position(-1, 4)));
        assert!(image2.light_pixels.contains(&Position(0, -2)));
        assert!(image2.light_pixels.contains(&Position(0, 0)));
        assert!(image2.light_pixels.contains(&Position(0, 4)));
        assert!(image2.light_pixels.contains(&Position(0, 5)));
        assert!(image2.light_pixels.contains(&Position(0, 6)));
        assert!(image2.light_pixels.contains(&Position(1, -2)));
        assert!(image2.light_pixels.contains(&Position(1, 2)));
        assert!(image2.light_pixels.contains(&Position(1, 3)));
        assert!(image2.light_pixels.contains(&Position(1, 5)));
        assert!(image2.light_pixels.contains(&Position(2, -2)));
        assert!(image2.light_pixels.contains(&Position(2, 4)));
        assert!(image2.light_pixels.contains(&Position(2, 6)));
        assert!(image2.light_pixels.contains(&Position(3, -1)));
        assert!(image2.light_pixels.contains(&Position(3, 1)));
        assert!(image2.light_pixels.contains(&Position(3, 2)));
        assert!(image2.light_pixels.contains(&Position(3, 3)));
        assert!(image2.light_pixels.contains(&Position(3, 4)));
        assert!(image2.light_pixels.contains(&Position(3, 5)));
        assert!(image2.light_pixels.contains(&Position(4, 0)));
        assert!(image2.light_pixels.contains(&Position(4, 2)));
        assert!(image2.light_pixels.contains(&Position(4, 3)));
        assert!(image2.light_pixels.contains(&Position(4, 4)));
        assert!(image2.light_pixels.contains(&Position(4, 5)));
        assert!(image2.light_pixels.contains(&Position(4, 6)));
        assert!(image2.light_pixels.contains(&Position(5, 1)));
        assert!(image2.light_pixels.contains(&Position(5, 2)));
        assert!(image2.light_pixels.contains(&Position(5, 4)));
        assert!(image2.light_pixels.contains(&Position(5, 5)));
        assert!(image2.light_pixels.contains(&Position(6, 2)));
        assert!(image2.light_pixels.contains(&Position(6, 3)));
        assert!(image2.light_pixels.contains(&Position(6, 4)));
    }

    #[test]
    fn test_enhance_repeatedly() {
        let (enhancement, image0) = parse_input(&TEST_INPUT);
        let image2 = image0.enhance(&enhancement).enhance(&enhancement);
        let repeated = image0.enhance_repeatedly(&enhancement, 2);

        assert_eq!(image2, repeated);
    }
}
