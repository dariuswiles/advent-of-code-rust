//! Advent of Code 2020 Day 20
//! https://adventofcode.com/2020/day/20
//!
//! Challenge part 2
//!
//! Given an input file containing square 2D tiles, rotate and/or flip the tiles to join them into
//! a single super-tile. This is analogous to fitting pieces of a jigsaw puzzle together. The
//! borders of tiles are discarded when creating the super-tile. The super-tile is then searched
//! for a given pattern (a sea monster), instances of which may be flipped or rotated. The number
//! of hash signs in the super-tile that are *not* part of a sea monster are counted to give the
//! answer to the challenge.

use std::collections::HashMap;
use std::fs;

const INPUT_FILENAME: &str = "2020_day20_input.txt";
const TILE_SIZE: usize = 10;
const TILE_INPUT_KEYWORD: &str = "Tile "; // The string immediately preceding the tile id

type Direction = usize; // Direction is used for tiles
const TOP: Direction = 0;
const RIGHT: Direction = 1;
const BOTTOM: Direction = 2;
const LEFT: Direction = 3;

type CompassDir = usize; // CompassDir is used for grids
const NORTH: CompassDir = 0;
const EAST: CompassDir = 1;
const SOUTH: CompassDir = 2;
#[allow(dead_code)]
const WEST: CompassDir = 3;

type Id = u16;
type Rotation = usize;
type Flipped = bool; // Indicates the tile is flipped *horizontally*
type Tiles = HashMap<Id, Tile>;
type TileMatches = HashMap<(Id, Direction), (Id, Direction, bool)>;

const SEA_MONSTER: [&str; 3] = [
    &"                  # ",
    &"#    ##    ##    ###",
    &" #  #  #  #  #  #   ",
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

/// A `Tile` stores a single tile, which is a square with a predetermined, constant length. For
/// efficient searching of matching tiles the borders of the tile are stored in `borders`, and
/// reversed (flipped) versions in `borders_flipped`. Borders are stored in the order: top, right,
/// bottom, left. Borders are stored in a clockwise direction, e.g., left-to-right for the top
/// border and right-to-left for the bottom border. This makes comparisons easier when the tile is
/// rotated.
#[derive(Clone, Debug, PartialEq)]
struct Tile {
    id: Id,
    cells: Vec<String>,
    borders: [String; 4],
    borders_flipped: [String; 4],
}

impl Tile {
    fn from_string(input: &str) -> Self {
        let mut lines = input.lines();
        let id_line = lines.next().unwrap();

        if !id_line.starts_with(TILE_INPUT_KEYWORD) {
            panic!("Tile input does not contain expected starting keyword");
        }

        let id = id_line
            .trim_start_matches(TILE_INPUT_KEYWORD)
            .trim_end_matches(':')
            .parse()
            .unwrap();

        let mut cells = Vec::new();
        let mut lines_read = 0;

        loop {
            if let Some(line) = lines.next() {
                if line == "" {
                    if lines_read == TILE_SIZE {
                        break;
                    } else {
                        panic!("Input contained a tile with an unexpected number of rows");
                    }
                }

                if line.len() != TILE_SIZE {
                    panic!("Input contained a tile row with an unexpected number of columns");
                }

                cells.push(line.to_owned());
                lines_read += 1;
            } else {
                break;
            }
        }

        let mut left = String::new();
        let mut right = String::new();

        for row in &cells {
            let mut r_chars = row.chars();
            left.insert(0, r_chars.next().unwrap());
            right.push(r_chars.last().unwrap());
        }

        let bottom: String = cells[TILE_SIZE - 1].chars().rev().collect();

        let borders = [cells[0].to_owned(), right, bottom, left];
        let borders_flipped: [String; 4] = [
            borders[TOP].chars().rev().collect(),
            borders[RIGHT].chars().rev().collect(),
            borders[BOTTOM].chars().rev().collect(),
            borders[LEFT].chars().rev().collect(),
        ];

        Self {
            id,
            cells,
            borders,
            borders_flipped,
        }
    }

    /// Attempts to match all four borders of the current object with non-flipped and flipped
    /// borders of the given Tile. Returns `None` if there are no matches between the two tiles,
    /// otherwise a triplet containing:
    ///     - the border of `self` that matches.
    ///     - the border of `other` that matches.
    ///     - a bool that is true iff the match requires one of the tiles to be flipped.
    ///
    /// NOTE The algorithm used assumes that no tile borders are palindromes, as this requires
    ///      more sophisticated logic that allows tile flips to be optional. An example of a
    ///      palindromic border, that cannot be handled by this code, is "###....###".
    fn find_matching_border(&self, other: &Tile) -> Option<(Direction, Direction, bool)> {
        for self_border_idx in 0..4 {
            for other_border_idx in 0..4 {
                // For the borders of two tiles to match, one must be a reversed version of the
                // other, e.g., "####......" matches "......####". If a match like this is found,
                // it is the simple case where neither of the tiles needs to be flipped.
                if self.borders[self_border_idx] == other.borders_flipped[other_border_idx] {
                    // println!("\tMatched tile {} border {} with tile {} border {}",
                    //     self.id, self_border_idx, other.id, other_border_idx
                    // );

                    return Some((self_border_idx, other_border_idx, false));
                }

                // As above, but this time look for *identical* borders. These still match, but
                // only if one of the tiles is flipped.
                if self.borders[self_border_idx] == other.borders[other_border_idx] {
                    // println!("\tMatched tile {} border {} with *flipped* tile {} border {}",
                    //     self.id, self_border_idx, other.id, other_border_idx
                    // );

                    return Some((self_border_idx, other_border_idx, true));
                }
            }
        }

        None
    }

    /// Returns the contents of column `column`, from top to bottom, as a String. The leftmost
    /// column is 0.
    fn column_to_string(&self, column: usize) -> String {
        let mut result = Vec::new();

        for row in &self.cells {
            result.push(row.chars().nth(column).unwrap());
        }
        result.iter().collect()
    }
}

/// A `GridTile` contains the information required to correctly orient a given tile within a grid
/// of tiles. The tile is first rotated clockwise 90° `rotation` times, then flipped horizontally
/// if `flip` is true.
//
// Flipping exchanges top and bottom borders, so need to be careful if rotating a tile to get one
// of its borders to be at the top or bottom of a `GridTile`, then flipping it.
#[derive(Clone, Debug, Default, PartialEq)]
struct GridTile {
    tile_id: Id,
    rotation: Rotation,
    flip: Flipped,
}

impl GridTile {
    /// Returns the requested `row` of this `GridTile` after accounting for its rotation and
    /// possible flipping.
    fn row_to_string(&self, tiles: &Tiles, row: usize) -> String {
        let max_index = TILE_SIZE - 1;

        match self.rotation {
            0 => {
                let mut r = row;
                if self.flip {
                    r = max_index - r;
                }
                return tiles[&self.tile_id].cells[r].to_string();
            }
            2 => {
                let mut r = max_index - row;
                if self.flip {
                    r = max_index - r;
                }
                return tiles[&self.tile_id].cells[r]
                    .chars()
                    .rev()
                    .collect::<String>()
                    .to_string();
            }
            1 => {
                let mut col = row;

                if self.flip {
                    col = max_index - col;
                }
                return tiles[&self.tile_id]
                    .column_to_string(col)
                    .chars()
                    .rev()
                    .collect::<String>()
                    .to_string();
            }
            3 => {
                let mut col = max_index - row;

                if self.flip {
                    col = max_index - col;
                }
                return tiles[&self.tile_id].column_to_string(col).to_string();
            }
            _ => {
                panic!("GridTile.row_to_string() failed because rotation not in range 0..4");
            }
        }
    }
}

/// A `Grid` contains a 2D layout of `GridTile`s, such that the adjacent borders of all `GridTile`s
/// match. The `Grid` is created empty and provides methods to determine the correct tile for each
/// position. The top-left tile of the `Grid` has (x, y) position (0, 0).
#[derive(Clone, Debug, Default, PartialEq)]
struct Grid {
    tile_grid: HashMap<Position, GridTile>,
}

impl Grid {
    /// Creates and returns a new grid. `corner_id` is the `Id` of the corner tile that has been
    /// chosen to be placed in the top-left corner of the new grid, i.e., at `Position` (0, 0). It
    /// is rotated so that its top and left borders have no matches, and its right and bottom
    /// borders do. A `GridTile` capturing this data is created and added as the first `GridTile`
    /// in the `Grid`.
    ///
    /// # Panics
    ///
    /// Panics if `corner_id` is not a corner tile, i.e., it does not have exactly two borders
    /// matching other tiles.
    fn new(tile_matches: &TileMatches, corner_tile_id: Id) -> Self {
        let corner_top_connects = tile_matches.get(&(corner_tile_id, TOP)) != None;
        let corner_right_connects = tile_matches.get(&(corner_tile_id, RIGHT)) != None;
        let corner_bottom_connects = tile_matches.get(&(corner_tile_id, BOTTOM)) != None;
        let corner_left_connects = tile_matches.get(&(corner_tile_id, LEFT)) != None;

        let mut rotation = usize::MAX;
        if corner_right_connects && corner_bottom_connects {
            rotation = 0;
        };
        if corner_top_connects && corner_right_connects {
            rotation = 1;
        };
        if corner_left_connects && corner_top_connects {
            rotation = 2;
        };
        if corner_bottom_connects && corner_left_connects {
            rotation = 3;
        };

        assert!(
            rotation < usize::MAX,
            "Grid::new() was passed a tile that is not a corner tile"
        );

        let mut tile_grid = HashMap::new();
        tile_grid.insert(
            Position { x: 0, y: 0 },
            GridTile {
                tile_id: corner_tile_id,
                rotation,
                flip: false,
            },
        );

        // print!("New Grid created with top-left tile set to corner tile passed. ");
        // println!("Contents of new Grid:\n{:?}", &tile_grid);

        Self { tile_grid }
    }

    /// Returns which border of the tile at `Position` is at `CompassDir` in this `Grid`. If the
    /// input is (0,0) and EAST, for example, the output could be LEFT because the tile has been
    /// rotated 180 degrees, so its LEFT border is on the EAST edge when the tile is positioned on
    /// this `Grid`. The result takes into account whether the tile is flipped.
    ///
    /// # Panics
    ///
    /// Panics if there is no tile at `Position`.
    fn get_border_for_pos(&self, pos: &Position, dir: CompassDir) -> Direction {
        let grid_tile = &self.tile_grid[(pos)];

        let mut result = (dir as usize + 4 - grid_tile.rotation as usize) % 4;

        if grid_tile.flip && (dir == NORTH || dir == SOUTH) {
            result = (result + 2) % 4;
        }

        result as Direction
    }

    /// Determines the `Id` of the tile that should be placed in a position determined by the `x`
    /// and `y` coordinates passed, and adds it. The determination is performed based on grid tiles
    /// to the west and north of the given position.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///     - there is already a tile at the `x` and `y` coordinates passed.
    ///     - the algorithm fails to find a tile to place at the `x` and `y` coordinates
    ///       passed, or if the tile found does not fit with all tiles in existing grid
    ///       positions.
    ///     - the `x` and `y` coordinates are 0, 0.
    fn add_tile_to_grid(&mut self, tile_matches: &TileMatches, pos: &Position) {
        assert!(
            self.tile_grid.get(&Position { x: pos.x, y: pos.y }) == None,
            "Cannot add tile to grid because position ({}, {}) is occupied",
            pos.x,
            pos.y
        );

        let mut grid_tile_based_on_tile_west = None;
        let mut grid_tile_based_on_tile_north = None;

        if pos.x > 0 {
            grid_tile_based_on_tile_west = self.determine_adjacent_tile(
                tile_matches,
                &Position {
                    x: pos.x - 1,
                    y: pos.y,
                },
                EAST,
            );
        }

        if pos.y > 0 {
            grid_tile_based_on_tile_north = self.determine_adjacent_tile(
                tile_matches,
                &Position {
                    x: pos.x,
                    y: pos.y - 1,
                },
                SOUTH,
            );
        }

        if let Some(tile_west) = grid_tile_based_on_tile_west {
            if let Some(tile_north) = grid_tile_based_on_tile_north {
                if tile_west == tile_north {
                    self.tile_grid.insert(*pos, tile_west);
                } else {
                    panic!(
                        "Cannot determine tile at position ({}, {}) because the tile \
                            to the left joins to:\n{:#?}\n\
                            but the tile above instead joins to \n{:#?}",
                        pos.x, pos.y, tile_west, tile_north
                    );
                }
            } else {
                self.tile_grid.insert(*pos, tile_west);
            }
        } else {
            if let Some(tile_north) = grid_tile_based_on_tile_north {
                self.tile_grid.insert(*pos, tile_north);
            } else {
                panic!(
                    "Cannot determine tile at position ({}, {}) from adjacent tiles",
                    pos.x, pos.y
                );
            }
        }

        // println!("Contents of Grid with addition of tile at ({}, {}):\n{:?}", pos.x, pos.y,
        //     &self.tile_grid
        // );
    }

    /// Based on the tile at the given `Position` (`pos`), determine which tile is in the
    /// adjacent tile in the given `compass` direction. `compass` can be one of NORTH, EAST, SOUTH
    /// or WEST. If the adjacent tile can be determined, returns a new `GridTile` object containing
    /// the tile's `Id`, `rotation` and whether it needs to be flipped.
    fn determine_adjacent_tile(
        &mut self,
        tile_matches: &TileMatches,
        pos: &Position,
        compass_dir: CompassDir,
    ) -> Option<GridTile> {
        if let Some(grid_tile) = self.tile_grid.get(pos) {
            // println!("determine_adjacent_tile called with pos = ({},{}) and compass_dir = {}",
            //     pos.x, pos.y, compass_dir
            // );

            // Determine which border of the tile located at `Position` `pos` is in `compass_dir`.
            let border_in_direction = self.get_border_for_pos(pos, compass_dir);

            // Determine if `tile_matches` contains a match for the tile at `pos` and its border in
            // `compass_dir`. If so, this is the tile we are looking for. Calculate the rotation
            // required to orient its matching border to be next to the `pos` and `compass_dir`
            // passed. For example, if `compass_dir` is EAST and a matching tile is found in that
            // direction, the matching tile needs to be rotated so that its matching border is on
            // the WEST edge when placed on the grid. If the matching tile also needs to be
            // flipped, its rotation is adjusted if `compass_dir` is NORTH or SOUTH (because
            // flipping is performed about the horizontal axis, which swaps top and bottom
            // borders).
            if let Some((adj_tile_id, adj_tile_border, adj_tile_flip)) =
                tile_matches.get(&(grid_tile.tile_id, border_in_direction))
            {
                let adj_rotation;
                let tile_is_flipped = grid_tile.flip ^ adj_tile_flip;
                if (compass_dir == NORTH) || (compass_dir == SOUTH) {
                    if tile_is_flipped {
                        // Need to get matching border to *same* `compass_dir` as that passed,
                        // e.g., if joining to the known tile at its south edge, the new tile's
                        // matching border must be rotated to also be on the south edge. Flipping
                        // will then switch it to the north edge.
                        adj_rotation = (compass_dir + 4 - adj_tile_border) % 4;
                    } else {
                        // Need to get matching border to *opposite* `compass_dir` as that passed,
                        // e.g., if joining to the known tile at its south edge, the new tile's
                        // matching border must be rotated to the north edge.
                        adj_rotation = (compass_dir + 6 - adj_tile_border) % 4;
                    }
                } else {
                    adj_rotation = (compass_dir + 6 - adj_tile_border) % 4;
                }

                return Some(GridTile {
                    tile_id: *adj_tile_id,
                    rotation: adj_rotation,
                    flip: tile_is_flipped,
                });
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    /// Returns a `Vec` of `Strings` containing the contents of the `Grid` in string form, where
    /// each tile is correctly rotated and flipped. `with_borders` determines whether the output
    /// includes the borders of each tile (plus a space between them to improve readability), or
    /// excludes them to create the single super-tile required to complete the challenge.
    fn to_strings(&self, tiles: &Tiles, grid_size: usize, with_borders: bool) -> Vec<String> {
        let mut result = Vec::new();

        if with_borders {
            let blank_tile_string = "____________________"[..TILE_SIZE].to_string() + " ";

            for grid_y in 0..grid_size {
                for tile_y in 0..TILE_SIZE {
                    let mut row_string = "".to_string();
                    for grid_x in 0..grid_size {
                        if let Some(t) = self.tile_grid.get(&Position {
                            x: grid_x,
                            y: grid_y,
                        }) {
                            row_string += &(t.row_to_string(tiles, tile_y).to_owned() + " ");
                        } else {
                            row_string += &blank_tile_string;
                        }
                    }
                    result.push(row_string);
                }
                result.push("".to_string());
            }
        } else {
            let blank_tile_string = "____________________"[..TILE_SIZE - 2].to_string();

            for grid_y in 0..grid_size {
                for tile_y in 1..TILE_SIZE - 1 {
                    let mut row_string = "".to_string();
                    for grid_x in 0..grid_size {
                        if let Some(t) = self.tile_grid.get(&Position {
                            x: grid_x,
                            y: grid_y,
                        }) {
                            row_string += &(t.row_to_string(tiles, tile_y)[1..TILE_SIZE - 1]);
                        } else {
                            row_string += &blank_tile_string;
                        }
                    }
                    result.push(row_string);
                }
            }
        }

        result
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Image {
    image: Vec<Vec<char>>,
}

impl Image {
    /// Creates and returns a new `Image` from a `Vec` of `String`s.
    fn new(image_vec: Vec<String>) -> Self {
        let mut image = Vec::new();

        for row in image_vec {
            image.push(row.chars().collect());
        }

        Image { image }
    }

    /// Searches this `Image` for all instances of the given `pattern`. `pattern` is an array of
    /// strings where only the character '#' is significant; all other characters are ignored. A
    /// match is defined as all '#' characters in `pattern` being present in `Image`, with all the
    /// other characters being ignored. For example, the `pattern` "#  #" matches "#..#", "##.#",
    /// "#.##" and "####" in the `Image`.
    ///
    /// The position of the top-left character of every pattern match is returned within a `Vec`.
    fn find_pattern(&self, pattern: &Pattern) -> Vec<Position> {
        let mut result = Vec::new();

        let pattern_width = pattern.pattern[0].len();
        let pattern_height = pattern.pattern.len();

        let image_width = self.image[0].len();
        let image_height = self.image.len();

        for i_y in 0..=image_height - pattern_height {
            'outer: for i_x in 0..=image_width - pattern_width {
                for p_y in 0..pattern_height {
                    for p_x in 0..pattern_width {
                        if pattern.pattern[p_y][p_x] == '#'
                            && self.image[i_y + p_y][i_x + p_x] != '#'
                        {
                            continue 'outer;
                        }
                    }
                }
                result.push(Position { x: i_x, y: i_y });
            }
        }

        result
    }

    /// Return the number of hash characters in this `Pattern` that have not been exclude by
    /// `mask`. A hash is excluded if its position within `mask` is true.
    fn count_hashes_not_in_mask(&self, mask: &ImageMask) -> u64 {
        assert_eq!(
            self.image.len(),
            mask.mask.len(),
            "Pattern and ImageMask heights must match"
        );
        assert_eq!(
            self.image[0].len(),
            mask.mask[0].len(),
            "Pattern and ImageMask widths must match"
        );

        let mut hash_count = 0;
        for y in 0..self.image.len() {
            for x in 0..self.image[0].len() {
                if self.image[y][x] == '#' && !mask.mask[y][x] {
                    hash_count += 1;
                }
            }
        }

        hash_count
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct ImageMask {
    mask: Vec<Vec<bool>>,
}

impl ImageMask {
    /// Creates and returns an ImageMask with a height and width of `size`.
    fn new(size: usize) -> Self {
        let mut mask = Vec::new();

        for _ in 0..size {
            let mut inner = Vec::new();
            inner.resize(size, false);
            mask.push(inner);
        }

        ImageMask { mask }
    }

    /// Applies `pattern` to this `ImageMask` such that every '#' in `pattern` sets the
    /// corresponding Boolean value in `ImageMask` to true. `pos` is used to position the top-left
    /// corner of `pattern` within `ImageMask`.
    ///
    /// # Panics
    ///
    /// Panics if any part of `pattern` extends beyond the width or height of this `ImageMask`.
    fn set_pattern(&mut self, pattern: &Pattern, pos: &Position) {
        let pattern_width = pattern.pattern[0].len();
        let pattern_height = pattern.pattern.len();

        for p_y in 0..pattern_height {
            for p_x in 0..pattern_width {
                if pattern.pattern[p_y].iter().nth(p_x).unwrap() == &'#' {
                    self.mask[pos.y + p_y][pos.x + p_x] = true;
                }
            }
        }
    }

    /// Convenience function to call `set_pattern` with a Vec of `Position`s, rather than just one.
    /// See `set_pattern` for more details.
    fn set_patterns(&mut self, pattern: &Pattern, pos: &Vec<Position>) {
        for p in pos {
            self.set_pattern(pattern, p);
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Pattern {
    pattern: Vec<Vec<char>>,
}

impl Pattern {
    /// Create and return a new `Pattern` from an array of strings.
    fn new(s: &[&str]) -> Self {
        let mut pattern = Vec::new();

        for row in s {
            pattern.push(row.chars().collect());
        }

        Self { pattern }
    }

    /// Return a new instance of `Pattern` representing this pattern flipped horizontally, i.e.,
    /// the top row becomes the bottom, etc.
    fn flip_horizontally(&self) -> Pattern {
        let mut result = self.pattern.clone();
        result.reverse();

        Pattern { pattern: result }
    }

    /// Return a new instance of `Pattern` representing this pattern rotated 90 degrees clockwise.
    fn rotate_clockwise(&self) -> Pattern {
        let mut result: Vec<Vec<char>> = Vec::new();

        for _ in 0..self.pattern[0].len() {
            result.push(Vec::new());
        }

        for orig_row in self.pattern.iter().rev() {
            for (idx, c) in orig_row.iter().enumerate() {
                result[idx].push(*c);
            }
        }

        Pattern { pattern: result }
    }
}

fn parse_input(input: &str) -> HashMap<Id, Tile> {
    // println!("parse_input called with data \n{}", &input);
    let lines: Vec<&str> = input.lines().collect();

    let mut tiles = HashMap::new();
    let mut tile_start = 0;
    for i in 0..lines.len() {
        if lines[i].starts_with(TILE_INPUT_KEYWORD) {
            tile_start = i;
            // println!("tile_start = {}", tile_start);
        }

        if lines[i] == "" {
            let tile_block = lines[tile_start..i].join("\n");
            // println!("parse_input about to call from_string with data\n{:#?}", &tile_block);

            let tile = Tile::from_string(&tile_block);
            tiles.insert(tile.id, tile);
        }
    }

    if tile_start + TILE_SIZE + 1 == lines.len() {
        let tile_block = lines[tile_start..lines.len()].join("\n");
        // println!("parse_input calling from_string with final block of data\n{:#?}", &tile_block);

        let tile = Tile::from_string(&tile_block);
        tiles.insert(tile.id, tile);
    }

    // println!("parse_input returning\n{:#?}", &tiles);
    tiles
}

/// Returns a `HashMap` containing tile border matches, with the key being the tile `Id` of one
/// matching tile, and `HashMap` value of:
///     - border of the key tile that matches;
///     - tile `Id` of the second matching tile;
///     - border of the second matching tile;
///     - a `bool` indicating whether one border needs to be flipped in order for them to match.
///
/// # Panics
///
/// The code assumes the border of each piece matches either no borders or exactly 1 border of
/// another piece. The former occurs if the border is at the outside edge of the super-tile. If a
/// border matches multiple other borders the code panics as this program is not sufficiently
/// sophisticated to handle this case.
fn find_tile_matches(tiles: &HashMap<Id, Tile>) -> TileMatches {
    let mut matches = HashMap::new();

    let tile_ids = tiles.keys();
    let _tiles_count = tiles.len();

    for tid0 in tile_ids.clone() {
        for tid1 in tile_ids.clone() {
            if tid0 == tid1 {
                continue;
            }

            if let Some((this_border, other_border, flip)) =
                tiles[tid0].find_matching_border(&tiles[tid1])
            {
                matches.insert((*tid0, this_border), (*tid1, other_border, flip));
            }
        }
    }

    matches
}

/// Load tiles from input file, find matching borders and return the product of the ids of the four
/// corner tiles.
fn construct_image(input: &str) -> Image {
    let tiles = parse_input(input);
    let grid_length_f32 = f32::sqrt(tiles.len() as f32);
    if f32::fract(grid_length_f32) > f32::EPSILON * 100.0 {
        panic!(
            "Found {} tiles, which is not a square number so cannot form a square grid",
            tiles.len()
        );
    }
    let grid_length = grid_length_f32 as usize;

    let tile_matches = find_tile_matches(&tiles);
    // println!("List of all matching borders\n{:?}", &tile_matches);

    let tile_match_counts =
        tile_matches
            .keys()
            .fold(HashMap::<Id, usize>::new(), |mut hm, (id, _)| {
                *hm.entry(*id).or_default() += 1;
                hm
            });

    let corners: Vec<Id> = tile_match_counts
        .iter()
        .filter(|(_, &tot)| tot == 2)
        .map(|(&id, _)| id)
        .collect();
    if corners.len() != 4 {
        panic!("Expecting four corners, but found {}", corners.len());
    }
    // println!("Ids of corner tiles are: {:?}", corners);

    let edges: Vec<Id> = tile_match_counts
        .iter()
        .filter(|(_, &tot)| tot == 3)
        .map(|(&id, _)| id)
        .collect();
    if edges.len() != (grid_length - 2) * 4 {
        panic!(
            "Found {} edge tiles (excluding corners), rather than the {} expected",
            edges.len(),
            (grid_length - 2) * 4
        );
    }

    let lowest_corner_tile_id = corners.iter().min().unwrap();
    // println!("Lowest tile id of all corners is {:#?}", lowest_corner_tile_id);

    let mut grid = Grid::new(&tile_matches, *lowest_corner_tile_id);

    // At this point, the top-left corner tile has been placed in the grid of tiles and its
    // orientation is correct. The next task is to determine the correct tile to place to its
    // right, in grid position (1, 0).

    for y in 0..grid_length {
        for x in 0..grid_length {
            if (x == 0) && (y == 0) {
                continue;
            }

            grid.add_tile_to_grid(&tile_matches, &Position { x, y });
        }
    }

    Image::new(grid.to_strings(&tiles, grid_length, false))
}

fn find_monsters(sea: &Image, pattern: &[&str]) -> ImageMask {
    let mut mask = ImageMask::new(sea.image[0].len());

    let mut sm = Pattern::new(pattern);
    let mut smf = sm.flip_horizontally();

    for _ in 0..4 {
        mask.set_patterns(&sm, &sea.find_pattern(&sm));
        mask.set_patterns(&smf, &sea.find_pattern(&smf));

        sm = sm.rotate_clockwise();
        smf = smf.rotate_clockwise();
    }

    mask
}

/// Perform the steps required by the challenge.
fn do_challenge(input: &str, pattern: &[&str]) -> u64 {
    let sea = construct_image(input);

    // for row in sea.image.iter() {
    // println!("{:?}", &row.iter().collect::<String>());
    // }

    let monster_mask = find_monsters(&sea, pattern);

    sea.count_hashes_not_in_mask(&monster_mask)
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let answer = do_challenge(&input_file, &SEA_MONSTER);
    println!(
        "The number of hash signs in the combined set of tiles that are *not* part of a sea \
        monster is {}",
        answer
    );
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    const TEST_SINGLE_TILE: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";

    const TEST_TWO_TILES: &str = "\
Tile 5555:
..##......
#.........
#.........
#.........
#.........
#.........
#.........
#.........
#.........
..#.......

Tile 7777:
##########
#.........
#.........
#.........
#.........
#.........
#.........
#.........
#.........
......##..";

    #[test]
    fn tile_creation() {
        let tile = Tile::from_string(TEST_SINGLE_TILE);

        assert_eq!(tile.cells.len(), TILE_SIZE);
        assert_eq!(tile.cells[0].len(), TILE_SIZE);
        assert_eq!(tile.cells[0], "..##.#..#.");
        assert_eq!(tile.cells[9], "..###..###");
        assert_eq!(tile.cells[9].len(), TILE_SIZE);

        assert_eq!(tile.borders[0], "..##.#..#.");
        assert_eq!(tile.borders[1], "...#.##..#");
        assert_eq!(tile.borders[2], "###..###..");
        assert_eq!(tile.borders[3], ".#..#####.");

        assert_eq!(tile.borders_flipped[0], ".#..#.##..");
        assert_eq!(tile.borders_flipped[1], "#..##.#...");
        assert_eq!(tile.borders_flipped[2], "..###..###");
        assert_eq!(tile.borders_flipped[3], ".#####..#.");
    }

    #[test]
    fn parse_one_tile() {
        let tile = parse_input(TEST_SINGLE_TILE);
        assert_eq!(tile[&2311].cells.len(), TILE_SIZE);
        assert_eq!(tile[&2311].cells[0].len(), TILE_SIZE);
        assert_eq!(tile[&2311].cells[0], "..##.#..#.");
        assert_eq!(tile[&2311].borders[1], "...#.##..#");
    }

    #[test]
    fn match_two_tiles() {
        let tiles = parse_input(TEST_TWO_TILES);

        let matches = find_tile_matches(&tiles);

        println!("find_tile_matches returned\n{:?}", matches);

        let mut expected_result = HashMap::new();
        expected_result.insert((5555, 0), (7777, 2, true));
        expected_result.insert((7777, 2), (5555, 0, true));
        assert_eq!(matches, expected_result);
    }

    #[test]
    fn column_to_string() {
        let tile = parse_input(TEST_SINGLE_TILE);

        assert_eq!(tile[&2311].column_to_string(1), ".#.####.#.".to_string());
        assert_eq!(tile[&2311].column_to_string(8), "#.#.###.##".to_string());
    }

    #[test]
    fn grid_tile_row_to_string() {
        let tile = parse_input(TEST_SINGLE_TILE);
        let gt_rot0 = GridTile {
            tile_id: 2311,
            rotation: 0,
            flip: false,
        };
        let gt_rot1 = GridTile {
            tile_id: 2311,
            rotation: 1,
            flip: false,
        };
        let gt_rot2 = GridTile {
            tile_id: 2311,
            rotation: 2,
            flip: false,
        };
        let gt_rot3 = GridTile {
            tile_id: 2311,
            rotation: 3,
            flip: false,
        };
        let gt_rot0_f = GridTile {
            tile_id: 2311,
            rotation: 0,
            flip: true,
        };
        let gt_rot1_f = GridTile {
            tile_id: 2311,
            rotation: 1,
            flip: true,
        };
        let gt_rot2_f = GridTile {
            tile_id: 2311,
            rotation: 2,
            flip: true,
        };
        let gt_rot3_f = GridTile {
            tile_id: 2311,
            rotation: 3,
            flip: true,
        };

        assert_eq!(gt_rot0.row_to_string(&tile, 3), "####.#...#");
        assert_eq!(gt_rot1.row_to_string(&tile, 2), "###...#..#");
        assert_eq!(gt_rot2.row_to_string(&tile, 4), "###.#...##");
        assert_eq!(gt_rot3.row_to_string(&tile, 9), ".#####..#.");

        assert_eq!(gt_rot0_f.row_to_string(&tile, 7), "#...##..#.");
        assert_eq!(gt_rot1_f.row_to_string(&tile, 0), "#..##.#...");
        assert_eq!(gt_rot2_f.row_to_string(&tile, 5), "###.#...##");
        assert_eq!(gt_rot3_f.row_to_string(&tile, 8), "#.#.###.##");
    }

    #[test]
    fn get_border_for_pos() {
        let pos = Position { x: 0, y: 0 };
        let grid_tile_grid = HashMap::new();
        let mut grid = Grid {
            tile_grid: grid_tile_grid,
        };

        grid.tile_grid.insert(
            pos,
            GridTile {
                tile_id: 2311,
                rotation: 0,
                flip: false,
            },
        );
        assert_eq!(grid.get_border_for_pos(&pos, NORTH), TOP);
        assert_eq!(grid.get_border_for_pos(&pos, EAST), RIGHT);
        assert_eq!(grid.get_border_for_pos(&pos, SOUTH), BOTTOM);
        assert_eq!(grid.get_border_for_pos(&pos, WEST), LEFT);

        grid.tile_grid.insert(
            pos,
            GridTile {
                tile_id: 2311,
                rotation: 1,
                flip: false,
            },
        );
        assert_eq!(grid.get_border_for_pos(&pos, NORTH), LEFT);
        assert_eq!(grid.get_border_for_pos(&pos, EAST), TOP);
        assert_eq!(grid.get_border_for_pos(&pos, SOUTH), RIGHT);
        assert_eq!(grid.get_border_for_pos(&pos, WEST), BOTTOM);

        grid.tile_grid.insert(
            pos,
            GridTile {
                tile_id: 2311,
                rotation: 2,
                flip: false,
            },
        );
        assert_eq!(grid.get_border_for_pos(&pos, NORTH), BOTTOM);
        assert_eq!(grid.get_border_for_pos(&pos, EAST), LEFT);
        assert_eq!(grid.get_border_for_pos(&pos, SOUTH), TOP);
        assert_eq!(grid.get_border_for_pos(&pos, WEST), RIGHT);

        grid.tile_grid.insert(
            pos,
            GridTile {
                tile_id: 2311,
                rotation: 3,
                flip: false,
            },
        );
        assert_eq!(grid.get_border_for_pos(&pos, NORTH), RIGHT);
        assert_eq!(grid.get_border_for_pos(&pos, EAST), BOTTOM);
        assert_eq!(grid.get_border_for_pos(&pos, SOUTH), LEFT);
        assert_eq!(grid.get_border_for_pos(&pos, WEST), TOP);

        grid.tile_grid.insert(
            pos,
            GridTile {
                tile_id: 2311,
                rotation: 0,
                flip: true,
            },
        );
        assert_eq!(grid.get_border_for_pos(&pos, NORTH), BOTTOM);
        assert_eq!(grid.get_border_for_pos(&pos, EAST), RIGHT);
        assert_eq!(grid.get_border_for_pos(&pos, SOUTH), TOP);
        assert_eq!(grid.get_border_for_pos(&pos, WEST), LEFT);

        grid.tile_grid.insert(
            pos,
            GridTile {
                tile_id: 2311,
                rotation: 1,
                flip: true,
            },
        );
        assert_eq!(grid.get_border_for_pos(&pos, NORTH), RIGHT);
        assert_eq!(grid.get_border_for_pos(&pos, EAST), TOP);
        assert_eq!(grid.get_border_for_pos(&pos, SOUTH), LEFT);
        assert_eq!(grid.get_border_for_pos(&pos, WEST), BOTTOM);

        grid.tile_grid.insert(
            pos,
            GridTile {
                tile_id: 2311,
                rotation: 2,
                flip: true,
            },
        );
        assert_eq!(grid.get_border_for_pos(&pos, NORTH), TOP);
        assert_eq!(grid.get_border_for_pos(&pos, EAST), LEFT);
        assert_eq!(grid.get_border_for_pos(&pos, SOUTH), BOTTOM);
        assert_eq!(grid.get_border_for_pos(&pos, WEST), RIGHT);

        grid.tile_grid.insert(
            pos,
            GridTile {
                tile_id: 2311,
                rotation: 3,
                flip: true,
            },
        );
        assert_eq!(grid.get_border_for_pos(&pos, NORTH), LEFT);
        assert_eq!(grid.get_border_for_pos(&pos, EAST), BOTTOM);
        assert_eq!(grid.get_border_for_pos(&pos, SOUTH), RIGHT);
        assert_eq!(grid.get_border_for_pos(&pos, WEST), TOP);
    }

    #[test]
    fn add_tile_to_grid() {
        let tiles = parse_input(TEST_INPUT);
        let matches = find_tile_matches(&tiles);

        println!("{:#?}", &matches);

        // Create a `Grid` by hand so that the first tile exactly matches the test data on the
        // Advent of Code website.
        let mut grid_tile_grid = HashMap::new();
        grid_tile_grid.insert(
            Position { x: 0, y: 0 },
            GridTile {
                tile_id: 1951,
                rotation: 0,
                flip: true,
            },
        );
        let mut grid = Grid {
            tile_grid: grid_tile_grid,
        };

        grid.add_tile_to_grid(&matches, &Position { x: 1, y: 0 });
        assert_eq!(
            grid.tile_grid[&Position { x: 1, y: 0 }],
            GridTile {
                tile_id: 2311,
                rotation: 0,
                flip: true
            }
        );

        grid.add_tile_to_grid(&matches, &Position { x: 2, y: 0 });
        assert_eq!(
            grid.tile_grid[&Position { x: 2, y: 0 }],
            GridTile {
                tile_id: 3079,
                rotation: 0,
                flip: false
            }
        );

        grid.add_tile_to_grid(&matches, &Position { x: 0, y: 1 });
        assert_eq!(
            grid.tile_grid[&Position { x: 0, y: 1 }],
            GridTile {
                tile_id: 2729,
                rotation: 0,
                flip: true
            }
        );

        grid.add_tile_to_grid(&matches, &Position { x: 1, y: 1 });
        assert_eq!(
            grid.tile_grid[&Position { x: 1, y: 1 }],
            GridTile {
                tile_id: 1427,
                rotation: 0,
                flip: true
            }
        );

        grid.add_tile_to_grid(&matches, &Position { x: 2, y: 1 });
        assert_eq!(
            grid.tile_grid[&Position { x: 2, y: 1 }],
            GridTile {
                tile_id: 2473,
                rotation: 1,
                flip: true
            }
        );

        grid.add_tile_to_grid(&matches, &Position { x: 0, y: 2 });
        assert_eq!(
            grid.tile_grid[&Position { x: 0, y: 2 }],
            GridTile {
                tile_id: 2971,
                rotation: 0,
                flip: true
            }
        );

        grid.add_tile_to_grid(&matches, &Position { x: 1, y: 2 });
        assert_eq!(
            grid.tile_grid[&Position { x: 1, y: 2 }],
            GridTile {
                tile_id: 1489,
                rotation: 0,
                flip: true
            }
        );

        grid.add_tile_to_grid(&matches, &Position { x: 2, y: 2 });
        assert_eq!(
            grid.tile_grid[&Position { x: 2, y: 2 }],
            GridTile {
                tile_id: 1171,
                rotation: 2,
                flip: true
            }
        );
    }

    #[test]
    fn find_pattern() {
        let my_image = Image::new(vec![
            "#..##.#".to_string(),
            ".####..".to_string(),
            "...#.##".to_string(),
        ]);

        #[rustfmt::skip]
        let results1 = my_image.find_pattern(&Pattern::new(&[
            " # ",
            "###",
            " # "
        ]));
        assert_eq!(results1, vec![Position { x: 2, y: 0 }]);

        let results2 = my_image.find_pattern(&Pattern::new(&["#  #"]));
        assert_eq!(
            results2,
            vec![
                Position { x: 0, y: 0 },
                Position { x: 3, y: 0 },
                Position { x: 1, y: 1 },
                Position { x: 3, y: 2 }
            ]
        );
    }

    #[test]
    fn image_mask() {
        let mut image_mask = ImageMask::new(5);

        #[rustfmt::skip]
        let pattern = Pattern::new(&[
            "## ",
            "#  ",
            "###"
        ]);
        image_mask.set_pattern(&pattern, &Position { x: 1, y: 2 });

        assert_eq!(image_mask.mask[0], vec![false, false, false, false, false]);
        assert_eq!(image_mask.mask[1], vec![false, false, false, false, false]);
        assert_eq!(image_mask.mask[2], vec![false, true, true, false, false]);
        assert_eq!(image_mask.mask[3], vec![false, true, false, false, false]);
        assert_eq!(image_mask.mask[4], vec![false, true, true, true, false]);
    }


    #[test]
    fn pattern_flip_horizontally() {
        #[rustfmt::skip]
        let pat = Pattern::new(&[
            " # ",
            "# #",
            "###"
        ]);

        let flipped = pat.flip_horizontally();
        assert_eq!(flipped.pattern[0], vec!['#', '#', '#']);
        assert_eq!(flipped.pattern[1], vec!['#', ' ', '#']);
        assert_eq!(flipped.pattern[2], vec![' ', '#', ' ']);
    }

    #[test]
    fn pattern_rotate() {
        #[rustfmt::skip]
        let pat = Pattern::new(&[
            "# # ",
            " ###",
            "  ##"
        ]);

        let rot = pat.rotate_clockwise();
        assert_eq!(rot.pattern.len(), 4, "Incorrect number of rows");
        assert_eq!(
            rot.pattern[0].len(),
            3,
            "Row 0 contains an incorrect number of characters"
        );
        assert_eq!(rot.pattern[0], vec![' ', ' ', '#']);
        assert_eq!(rot.pattern[1], vec![' ', '#', ' ']);
        assert_eq!(rot.pattern[2], vec!['#', '#', '#']);
        assert_eq!(rot.pattern[3], vec!['#', '#', ' ']);
    }

    #[test]
    fn solve_test_puzzle() {
        let answer = do_challenge(&TEST_INPUT, &SEA_MONSTER);
        assert_eq!(answer, 273);
    }
}
