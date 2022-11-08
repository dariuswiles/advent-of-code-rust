//! Advent of Code 2021 Day 18
//! https://adventofcode.com/2021/day/18
//!
//! Challenge part 1
//!
//! Parse an input file of numbers in "Snailfish" format, one number per line, and add them
//! together to find the answer to the challenge.

use std::fmt::{Display, Error, Formatter};
use std::fs;

const INPUT_FILENAME: &str = "2021_day18_input.txt";

type Int = u8;

#[derive(Debug)]
struct ExplodeData<'a> {
    node_to_explode: Option<&'a mut Number>,
    nearest_left: Option<&'a mut Number>,
    nearest_right: Option<&'a mut Number>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Number {
    Regular(Int),
    Compound {
        left: Box<Number>,
        right: Box<Number>,
    },
}

impl Number {
    fn new(input: &str) -> Self {
        let mut chars: Vec<char> = input.chars().collect();
        parse_number(&mut chars)
    }

    /// Searches this object for the first explode action that is required, if any. If
    /// required, the modifications explained in the challenge are made and `true` is returned.
    /// Otherwise, `false` is returned and no changes are made. From the challenge, the
    /// modifications apply to "any pair ... nested inside four pairs" and changes are made to
    /// the leftmost such pair.
    ///
    /// "To explode a pair, the pair's left value is added to the first regular number to the
    /// left of the exploding pair (if any), and the pair's right value is added to the first
    /// regular number to the right of the exploding pair (if any). Exploding pairs will always
    /// consist of two regular numbers. Then, the entire exploding pair is replaced with the
    /// regular number 0."
    fn explode(&mut self) -> bool {
        let mut explode_data = ExplodeData {
            node_to_explode: None,
            nearest_left: None,
            nearest_right: None,
        };

        // println!("explode: Data before changes {}", &self);
        Self::explode_recurse(self, 0, &mut explode_data);

        if explode_data.node_to_explode.is_none() {
            // println!("explode: no compound Number needs exploding.");
            return false;
        }

        if let Number::Compound { left, right } = explode_data.node_to_explode.as_ref().unwrap() {
            if let Some(Number::Regular(nl)) = explode_data.nearest_left {
                if let Number::Regular(explode_left) = **left {
                    // print!("explode: Changing the nearest Regular number to the left of the exploding node. ");
                    // println!("Its current value is: {}. About to add {}", nl, explode_left);
                    *nl += explode_left;
                };
            }

            if let Some(Number::Regular(nl)) = explode_data.nearest_right {
                if let Number::Regular(explode_right) = **right {
                    // print!("explode: Changing the nearest Regular number to the right of the exploding node. ");
                    // println!("Its current value is: {}. About to add {}", nl, explode_right);
                    *nl += explode_right;
                };
            }
        }

        *explode_data.node_to_explode.unwrap() = Number::Regular(0);

        // println!("explode: Data after changes {}", &self);
        true
    }

    /// Recursively walks the node of Numbers starting at `node` looking for any Number that is
    /// "nested inside four pairs" of parent Numbers. `depth` is used to track the current depth of
    /// recursion. If a node needs exploding, updates `explode_data` to point to the node. The
    /// nearest number to the left and the nearest number to the right are also tracking in this
    /// data.
    fn explode_recurse<'a, 'b>(
        node: &'a mut Number,
        depth: usize,
        explode_data: &'b mut ExplodeData<'a>,
    ) {
        // Implementation note: this causes borrow problems if included in 'match' statement below.
        if let Number::Compound { .. } = node {
            if depth == 4 {
                // println!("explode: At nest level {}, reached criteria to perform an explode \
                // operation", depth
                // );
                explode_data.node_to_explode = Some(node);
                return;
            }
        }

        match node {
            Number::Compound { left, right } => {
                // println!("    The element is a compound Number", depth );
                // println!("    Recursing into Left nested Number");
                if explode_data.node_to_explode.is_none() {
                    Self::explode_recurse(left, depth + 1, explode_data);
                } else {
                    Self::explode_recurse(left, 0, explode_data);
                }

                if explode_data.node_to_explode.is_some() && explode_data.nearest_right.is_some() {
                    return;
                }

                // println!("    Recursing into Right nested Number");
                if explode_data.node_to_explode.is_none() {
                    Self::explode_recurse(right, depth + 1, explode_data);
                } else {
                    Self::explode_recurse(right, 0, explode_data);
                }
            }
            Number::Regular(_reg) => {
                // println!("    The element is regular Number {}", _reg);
                if explode_data.node_to_explode.is_none() {
                    explode_data.nearest_left = Some(node);
                } else {
                    explode_data.nearest_right = Some(node);
                }
            }
        }
    }

    /// Searches this object for the first split action that is required, if any, i.e., the first
    /// Regular Number which is "10 or greater". If such an action is required, replaces the Number
    /// with a Compound Number where:
    ///     the left element is the original number divided by two and rounded down, and
    ///     the right element is the original number divided by two and rounded up.
    ///
    /// Returns true if a split action is performed, false otherwise.
    fn split(&mut self) -> bool {
        if let Some(node_to_split) = Self::split_recurse(self) {
            if let Number::Regular(existing) = node_to_split {
                *node_to_split = Number::Compound {
                    left: Box::new(Number::Regular(*existing / 2)),
                    right: Box::new(Number::Regular((*existing as f32 / 2.0 + 0.5) as Int)),
                };

                return true;
            } else {
                panic!("Internal error: split() expected a Regular Number");
            }
        }

        false
    }

    /// Recursively walks the node of Numbers starting at `node` looking for any Regular Number
    /// greater or equal to 10. If found, the node holding this Number is returned.
    fn split_recurse(node: &mut Number) -> Option<&mut Number> {
        match node {
            Number::Compound { left, right } => {
                let search_left = Self::split_recurse(left);

                if search_left.is_some() {
                    return search_left;
                }

                return Self::split_recurse(right);
            }
            Number::Regular(reg) => {
                if *reg >= 10 {
                    return Some(node);
                } else {
                    return None;
                }
            }
        }
    }

    /// Reduces a snailfish Number using explodes and splits until no more changes are required.
    fn reduce(&mut self) {
        let mut changes_made = true;

        while changes_made {
            // println!("{}", self);
            changes_made = self.explode();
            if changes_made {
                continue;
            }

            changes_made = self.split();
        }
    }

    /// Returns the addition of two Sailfish `Number`s following the challenge criteria. The
    /// return value is a new compound `Number` composed of the the `Numbers` passed in. The output
    /// is not "reduced", and this operation should be performed separately after the add.
    #[must_use]
    #[allow(dead_code)]
    fn add_no_reduce(self, n: Number) -> Self {
        Self::Compound {
            left: Box::new(self),
            right: Box::new(n),
        }
    }

    /// Returns the addition of two Sailfish `Number`s following the challenge criteria. The
    /// return value is the result of creating a new compound `Number` composed of the the
    /// `Numbers` passed in, then "reducing" Number.
    #[must_use]
    fn add(self, n: Number) -> Self {
        let mut result = Self::Compound {
            left: Box::new(self),
            right: Box::new(n),
        };

        result.reduce();
        result
    }

    /// Returns the magnitude of Self.
    fn magnitude(&self) -> u32 {
        Self::magnitude_recurse(&self)
    }

    /// Recursively walks the tree of Numbers starting at `node` and returns a single magnitude
    /// representing the entire tree.
    fn magnitude_recurse(node: &Number) -> u32 {
        match node {
            Number::Compound { left, right } => {
                3 * Self::magnitude_recurse(left) + 2 * Self::magnitude_recurse(right)
            }
            Number::Regular(reg) => *reg as u32,
        }
    }

    /// Internal routine to be called recursively to write a Snailfish number. Should only be
    /// called by Self::fmt().
    fn fmt_recurse(node: &Number, f: &mut Formatter<'_>) {
        match node {
            Number::Compound { left, right } => {
                write!(f, "[").unwrap();
                Self::fmt_recurse(left, f);
                write!(f, ",").unwrap();
                Self::fmt_recurse(right, f);
                write!(f, "]").unwrap();
            }
            Number::Regular(n) => {
                write!(f, "{}", n).unwrap();
            }
        }
    }
}

/// Writes a Snailfish number in text form.
impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Self::fmt_recurse(&self, f);

        Ok(())
    }
}

/// Returns a snailfish `Number` (consisting of left and right sides) based on the input provided.
/// All parsed elements are removed from the input.
///
/// # Panics
///
/// Panics if the input is not in the format specified in the challenge.
fn parse_number(chars: &mut Vec<char>) -> Number {
    let left;
    let right;

    let mut c = chars.remove(0);
    assert_eq!(c, '[');

    c = chars[0];
    if c.is_digit(10) {
        c = chars.remove(0);
        left = Box::new(Number::Regular(c.to_digit(10).unwrap() as u8));
    } else {
        assert_eq!(c, '[');
        left = Box::new(parse_number(chars));
    }

    c = chars.remove(0);
    assert_eq!(c, ',');

    c = chars[0];
    if c.is_digit(10) {
        c = chars.remove(0);
        right = Box::new(Number::Regular(c.to_digit(10).unwrap() as u8));
    } else {
        assert_eq!(c, '[');
        right = Box::new(parse_number(chars));
    }

    c = chars.remove(0);
    assert_eq!(c, ']');

    Number::Compound { left, right }
}

/// Processes `input`, consisting of one snailfish Number per line, adding the result of each
/// number with the next and returning the result.
fn add_input(input: &str) -> Number {
    let mut sub_total: Option<Number> = None;

    for line in input.lines() {
        if line == "" {
            continue;
        }

        if let Some(st) = sub_total {
            sub_total = Some(st.add(Number::new(line)));
        } else {
            sub_total = Some(Number::new(line));
        }

        // println!("sub_total = {:?}", sub_total);
    }

    sub_total.unwrap()
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let result = add_input(&input_file);
    println!(
        "Iteratively adding all Snailfish numbers in the input gives a magnitude of {}",
        result.magnitude()
    );
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    // Tests over one line of input

    const TEST_INPUT_0: &str = "[1,2]";
    const TEST_INPUT_1: &str = "[[1,2],3]";
    const TEST_INPUT_2: &str = "[9,[8,7]]";
    const TEST_INPUT_3: &str = "[[1,9],[8,5]]";
    const TEST_INPUT_4: &str = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
    const TEST_INPUT_5: &str = "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]";
    const TEST_INPUT_6: &str = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";

    #[test]
    fn test_parse_number() {
        let mut chars: Vec<char> = "[3,4]".chars().collect();

        let result = parse_number(&mut chars);
        println!("{:?}", result);

        assert_eq!(
            result,
            Number::Compound {
                left: Box::new(Number::Regular(3)),
                right: Box::new(Number::Regular(4))
            }
        );
    }

    #[test]
    fn test_new_number() {
        assert_eq!(
            Number::new(&TEST_INPUT_0),
            Number::Compound {
                left: Box::new(Number::Regular(1)),
                right: Box::new(Number::Regular(2))
            }
        );
        assert_eq!(
            Number::new(&TEST_INPUT_1),
            Number::Compound {
                left: Box::new(Number::Compound {
                    left: Box::new(Number::Regular(1)),
                    right: Box::new(Number::Regular(2))
                }),
                right: Box::new(Number::Regular(3))
            }
        );
        assert_eq!(
            Number::new(&TEST_INPUT_2),
            Number::Compound {
                left: Box::new(Number::Regular(9)),
                right: Box::new(Number::Compound {
                    left: Box::new(Number::Regular(8)),
                    right: Box::new(Number::Regular(7))
                })
            }
        );
        assert_eq!(
            Number::new(&TEST_INPUT_3),
            Number::Compound {
                left: Box::new(Number::Compound {
                    left: Box::new(Number::Regular(1)),
                    right: Box::new(Number::Regular(9))
                }),
                right: Box::new(Number::Compound {
                    left: Box::new(Number::Regular(8)),
                    right: Box::new(Number::Regular(5))
                })
            }
        );

        assert_eq!(
            Number::new(&TEST_INPUT_4),
            Number::Compound {
                left: Box::new(Number::Compound {
                    left: Box::new(Number::Compound {
                        left: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(1)),
                            right: Box::new(Number::Regular(2))
                        }),
                        right: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(3)),
                            right: Box::new(Number::Regular(4))
                        })
                    }),
                    right: Box::new(Number::Compound {
                        left: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(5)),
                            right: Box::new(Number::Regular(6))
                        }),
                        right: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(7)),
                            right: Box::new(Number::Regular(8))
                        })
                    }),
                }),
                right: Box::new(Number::Regular(9))
            }
        );

        assert_eq!(
            Number::new(&TEST_INPUT_5),
            Number::Compound {
                left: Box::new(Number::Compound {
                    left: Box::new(Number::Compound {
                        left: Box::new(Number::Regular(9)),
                        right: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(3)),
                            right: Box::new(Number::Regular(8))
                        })
                    }),
                    right: Box::new(Number::Compound {
                        left: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(0)),
                            right: Box::new(Number::Regular(9))
                        }),
                        right: Box::new(Number::Regular(6))
                    })
                }),
                right: Box::new(Number::Compound {
                    left: Box::new(Number::Compound {
                        left: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(3)),
                            right: Box::new(Number::Regular(7))
                        }),
                        right: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(4)),
                            right: Box::new(Number::Regular(9))
                        }),
                    }),
                    right: Box::new(Number::Regular(3))
                }),
            }
        );

        assert_eq!(
            Number::new(&TEST_INPUT_6),
            Number::Compound {
                left: Box::new(Number::Compound {
                    left: Box::new(Number::Compound {
                        left: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(1)),
                            right: Box::new(Number::Regular(3))
                        }),
                        right: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(5)),
                            right: Box::new(Number::Regular(3))
                        })
                    }),
                    right: Box::new(Number::Compound {
                        left: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(1)),
                            right: Box::new(Number::Regular(3))
                        }),
                        right: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(8)),
                            right: Box::new(Number::Regular(7))
                        })
                    })
                }),
                right: Box::new(Number::Compound {
                    left: Box::new(Number::Compound {
                        left: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(4)),
                            right: Box::new(Number::Regular(9))
                        }),
                        right: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(6)),
                            right: Box::new(Number::Regular(9))
                        })
                    }),
                    right: Box::new(Number::Compound {
                        left: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(8)),
                            right: Box::new(Number::Regular(2))
                        }),
                        right: Box::new(Number::Compound {
                            left: Box::new(Number::Regular(7)),
                            right: Box::new(Number::Regular(3))
                        })
                    })
                }),
            }
        );
    }

    #[test]
    fn test_explode0() {
        let mut input0 = Number::new("[[[[[9,8],1],2],3],4]");
        assert!(input0.explode());
        assert_eq!(input0, Number::new("[[[[0,9],2],3],4]"));
    }

    #[test]
    fn test_explode1() {
        let mut input1 = Number::new("[7,[6,[5,[4,[3,2]]]]]");
        assert!(input1.explode());
        assert_eq!(input1, Number::new("[7,[6,[5,[7,0]]]]"));
    }

    #[test]
    fn test_explode2() {
        let mut input2 = Number::new("[[6,[5,[4,[3,2]]]],1]");
        assert!(input2.explode());
        assert_eq!(input2, Number::new("[[6,[5,[7,0]]],3]"));
    }

    #[test]
    fn test_explode3() {
        let mut input3 = Number::new("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        assert!(input3.explode());
        assert_eq!(input3, Number::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
    }

    #[test]
    fn test_explode4() {
        let mut input4 = Number::new("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert!(input4.explode());
        assert_eq!(input4, Number::new("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
    }

    #[test]
    fn test_split() {
        let mut input0 = Number::Regular(10);
        assert!(input0.split());
        assert_eq!(input0, Number::new("[5,5]"));

        let mut input1 = Number::Regular(11);
        assert!(input1.split());
        assert_eq!(input1, Number::new("[5,6]"));

        let mut input2 = Number::Regular(12);
        assert!(input2.split());
        assert_eq!(input2, Number::new("[6,6]"));
    }

    #[test]
    fn test_reduce() {
        let mut reduce0 = Number::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        reduce0.reduce();
        assert_eq!(reduce0, Number::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn test_addition_without_reduce() {
        let a = Number::new("[1,2]");
        let b = Number::new("[[3,4],5]");

        let c = a.add_no_reduce(b);
        assert_eq!(c, Number::new("[[1,2],[[3,4],5]]"));
    }

    #[test]
    fn test_add() {
        let num0 = Number::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let num1 = Number::new("[1,1]");

        let result = num0.add(num1);
        assert_eq!(result, Number::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    #[should_panic]
    fn test_missing_close_bracket() {
        Number::new("[1,2");
    }

    // Tests over multiple lines of input

    const TEST_MULTI_LINE_0: &str = "\
[1,1]
[2,2]
[3,3]
[4,4]
";

    const TEST_MULTI_LINE_1: &str = "\
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
";

    const TEST_MULTI_LINE_2: &str = "\
[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
";

    const TEST_MULTI_LINE_3: &str = "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
";

    #[test]
    fn test_multi_line0() {
        let result = add_input(&TEST_MULTI_LINE_0);
        assert_eq!(result, Number::new("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
    }

    #[test]
    fn test_multi_line1() {
        let result = add_input(&TEST_MULTI_LINE_1);
        assert_eq!(result, Number::new("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
    }

    #[test]
    fn test_multi_line2() {
        let result = add_input(&TEST_MULTI_LINE_2);
        assert_eq!(result, Number::new("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    }

    #[test]
    fn test_multi_line3() {
        let result = add_input(&TEST_MULTI_LINE_3);
        assert_eq!(
            result,
            Number::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Number::new("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(
            Number::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
            1384
        );
        assert_eq!(
            Number::new("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(),
            445
        );
        assert_eq!(
            Number::new("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(),
            791
        );
        assert_eq!(
            Number::new("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(),
            1137
        );
        assert_eq!(
            Number::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
            3488
        );
    }

    // Complete test, exercising all functions required to find the challenge answer.

    const TEST_FULL: &str = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_all_functions() {
        let result = add_input(&TEST_FULL);
        assert_eq!(result.magnitude(), 4140);
    }
}
