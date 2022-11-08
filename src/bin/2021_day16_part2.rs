//! Advent of Code 2021 Day 16
//! https://adventofcode.com/2021/day/16
//!
//! Challenge part 2
//!
//! Read a hexadecimal string from a file, parse it into its components and build a hierarchy of
//! packets to represent it. Calculations are performed on the packets based on their type,
//! resulting in a single number for the outermost packet which is the answer to part 2 of the
//! challenge.

use std::fs;

const INPUT_FILENAME: &str = "2021_day16_input.txt";

#[derive(Clone, Debug, PartialEq)]
enum PacketData {
    Literal(u64),
    Operator(Vec<Packet>),
}

/// Holds an array of bits, created from a hexadecimal string. Allows individual or groups of bits
/// to be retrieved using their index.
#[derive(Debug)]
struct BitBuffer {
    bit_vec: Vec<u8>,
}

impl BitBuffer {
    /// Returns a new BitBuffer containing the bit representation of the hexadecimal string passed.
    fn new(s: &str) -> Self {
        let s_len = s.len();
        assert!(s_len % 2 == 0);

        let mut bit_vec = Vec::new();

        for i in (0..s_len).step_by(2) {
            let s_slice = &s[i..i + 2];
            bit_vec.push(u8::from_str_radix(s_slice, 16).unwrap());
        }

        Self { bit_vec }
    }

    /// Returns the `nth` bit in this `BitBuffer`.
    fn nth(&self, bit_pos: usize) -> u8 {
        (self.bit_vec[bit_pos / 8] >> (7 - (bit_pos % 8))) & 1
    }

    /// Returns a `u64` containing a contiguous set of bits from this `BitBuffer` starting at
    /// `bit_start` and `bit_length` bits long. The maximum length is 32 bits. The output is
    /// contained in the least significant bits.
    fn get_bits(&self, bit_start: usize, bit_length: usize) -> u64 {
        assert!(bit_length <= 32);

        let mut result = 0;
        for i in bit_start..bit_start + bit_length {
            result <<= 1;
            result |= self.nth(i) as u64;
        }

        result
    }
}

/// Stores a packet and its associated data. A packet can contain sub-packets.
#[derive(Clone, Debug, PartialEq)]
struct Packet {
    version: u8,
    packet_type: u8,
    data: PacketData,
}

impl Packet {
    /// Returns a new `Packet` representing a parsed version of the hexadecimal data passed.
    fn new(input: &str) -> Self {
        let buffer = BitBuffer::new(input);

        let mut buffer_pos = 0;
        Packet::parse_packet(&buffer, &mut buffer_pos)
    }

    /// Returns a packet created from the data in `buffer` starting at `buffer_pos`. `buffer_pos`
    /// is modified to refer to the first bit of data not consumed during the creation of the
    /// returned object.
    fn parse_packet(buffer: &BitBuffer, buffer_pos: &mut usize) -> Packet {
        let version = buffer.get_bits(*buffer_pos, 3) as u8;
        *buffer_pos += 3;
        let packet_type = buffer.get_bits(*buffer_pos, 3) as u8;
        *buffer_pos += 3;

        match packet_type {
            4 => {
                // Literal value
                let literal = Packet::parse_literal(&buffer, buffer_pos);

                return Self {
                    version,
                    packet_type,
                    data: PacketData::Literal(literal),
                };
            }

            _ => {
                // Operator
                return Self {
                    version,
                    packet_type,
                    data: PacketData::Operator(Packet::parse_operator(buffer, buffer_pos)),
                };
            }
        }
    }

    /// Returns a literal object created from the data in `buffer` starting at `buffer_pos`.
    /// `buffer_pos` is modified to refer to the first bit of data not consumed during the creation
    /// of the returned object.
    fn parse_literal(buffer: &BitBuffer, buffer_pos: &mut usize) -> u64 {
        // println!("parse_literal entered with buffer_pos = {}", buffer_pos);
        let mut literal = 0;
        let mut more_data = true;

        while more_data {
            let literal_group = buffer.get_bits(*buffer_pos, 5);
            literal <<= 4;
            literal += literal_group & 0xF;
            more_data = (literal_group >> 4) == 1;
            *buffer_pos += 5;
        }
        // println!("parse_literal returning literal {} and buffer_pos of {}", literal, buffer_pos);
        literal
    }

    /// Returns an operator object created from the data in `buffer` starting at `buffer_pos`.
    /// `buffer_pos` is modified to refer to the first bit of data not consumed during the creation
    /// of the returned object.
    fn parse_operator(buffer: &BitBuffer, buffer_pos: &mut usize) -> Vec<Packet> {
        // println!("Entering parse_operator with buffer_pos = {}", buffer_pos);

        let mut sub_packets = Vec::new();

        if buffer.nth(*buffer_pos) == 0 {
            // Length type ID: next 15-bits = sub-pkt length in bits
            *buffer_pos += 1;

            let sub_packet_len = buffer.get_bits(*buffer_pos, 15) as usize;
            *buffer_pos += 15;
            // println!("Operator contains {} bits of sub-packets", sub_packet_len);
            let sub_packet_end = *buffer_pos + sub_packet_len;

            // println!("Entering loop with buffer_pos = {}, sub_packet_end = {}", buffer_pos, sub_packet_end);

            while *buffer_pos < sub_packet_end {
                sub_packets.push(Packet::parse_packet(&buffer, buffer_pos));
            }
        } else {
            // Length type ID: next 11-bits = number of sub-packets
            *buffer_pos += 1;

            let sub_packet_count = buffer.get_bits(*buffer_pos, 11) as usize;
            *buffer_pos += 11;
            // println!("Operator contains {} sub-packets", sub_packet_count);
            // println!("Entering loop with buffer_pos = {}", buffer_pos);

            for _ in 0..sub_packet_count {
                sub_packets.push(Packet::parse_packet(&buffer, buffer_pos));
            }
        }
        sub_packets
    }
}

/// Returns the result of performing the operation specified in the given packet's type on the
/// contents of the packet.
fn evaluate_packet(p: &Packet) -> u64 {
    if p.packet_type == 4 {
        if let PacketData::Literal(l) = p.data {
            return l;
        } else {
            panic!(
                "Packet type is literal, but data is not literal for packet {:#?}",
                p
            );
        }
    }

    if p.packet_type <= 7 {
        let mut sub_packet_data = Vec::new();

        if let PacketData::Operator(sub_packets) = &p.data {
            for sub_packet in sub_packets {
                sub_packet_data.push(evaluate_packet(sub_packet) as u64);
            }
        } else {
            panic!(
                "Packet contents do not match packet type for packet {:#?}",
                &p.data
            );
        }

        match p.packet_type {
            0 => {
                return sub_packet_data.iter().sum();
            }
            1 => {
                return sub_packet_data.iter().product();
            }
            2 => {
                return *sub_packet_data.iter().min().unwrap();
            }
            3 => {
                return *sub_packet_data.iter().max().unwrap();
            }
            5 => {
                // Greater than
                assert_eq!(sub_packet_data.len(), 2);
                if sub_packet_data[0] > sub_packet_data[1] {
                    return 1;
                } else {
                    return 0;
                }
            }
            6 => {
                // Less than
                assert_eq!(sub_packet_data.len(), 2);
                if sub_packet_data[0] < sub_packet_data[1] {
                    return 1;
                } else {
                    return 0;
                }
            }
            7 => {
                // Equals
                assert_eq!(sub_packet_data.len(), 2);
                if sub_packet_data[0] == sub_packet_data[1] {
                    return 1;
                } else {
                    return 0;
                }
            }
            _ => {
                panic!("Unrecognized packet type for packet {:#?}", p);
            }
        }
    } else {
        panic!("Unrecognized packet type for packet {:#?}", p);
    }
}

fn main() {
    let input_file = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");

    let answer = evaluate_packet(&Packet::new(&input_file.lines().next().unwrap()));
    println!("The sum of all versions is {}", answer);
}

// Test using data from the examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PACKET_LITERAL: &str = "D2FE28";
    const TEST_PACKET_AS_BITS: [u8; 3] = [0b1101_0010, 0b1111_1110, 0b0010_1000];

    const TEST_PACKET_OP_ID0: &str = "38006F45291200";
    const TEST_PACKET_OP_ID1: &str = "EE00D40C823060";
    const TEST_PACKET_OP_OP_OP: &str = "8A004A801A8002F478";

    const TEST_PACKET_SUM: &str = "C200B40A82";
    const TEST_PACKET_PRODUCT: &str = "04005AC33890";
    const TEST_PACKET_MIN: &str = "880086C3E88112";
    const TEST_PACKET_MAX: &str = "CE00C43D881120";
    const TEST_PACKET_GT: &str = "D8005AC2A8F0";
    const TEST_PACKET_LT: &str = "F600BC2D8F";
    const TEST_PACKET_EQ: &str = "9C005AC2F8F0";
    const TEST_PACKET_FULL: &str = "9C0141080250320F1802104A08";

    #[test]
    fn test_bitbuffer() {
        let bb = BitBuffer::new(&TEST_PACKET_LITERAL);
        assert_eq!(bb.bit_vec[0], TEST_PACKET_AS_BITS[0]);
        assert_eq!(bb.bit_vec[1], TEST_PACKET_AS_BITS[1]);
        assert_eq!(bb.bit_vec[2], TEST_PACKET_AS_BITS[2]);
    }

    #[test]
    fn test_bb_nth() {
        let bb = BitBuffer::new(&TEST_PACKET_LITERAL);
        assert_eq!(bb.nth(0), 1);
        assert_eq!(bb.nth(1), 1);
        assert_eq!(bb.nth(2), 0);
        assert_eq!(bb.nth(8), 1);
        assert_eq!(bb.nth(15), 0);
        assert_eq!(bb.nth(16), 0);
        assert_eq!(bb.nth(23), 0);
    }

    #[test]
    fn test_bb_get_bits() {
        let bb = BitBuffer::new(&TEST_PACKET_LITERAL);

        let bits0 = bb.get_bits(0, 8);
        assert_eq!(bits0, TEST_PACKET_AS_BITS[0] as u64);

        let bits1 = bb.get_bits(4, 8);
        assert_eq!(bits1, 0b0010_1111);
    }

    #[test]
    fn test_parse_literal_packet() {
        let p = Packet::new(&TEST_PACKET_LITERAL);

        assert_eq!(p.version, 6);
        assert_eq!(p.packet_type, 4);
        assert_eq!(p.data, PacketData::Literal(2021));
    }

    #[test]
    fn test_parse_op0() {
        let p = Packet::new(&TEST_PACKET_OP_ID0);

        assert_eq!(
            p,
            Packet {
                version: 1,
                packet_type: 6,
                data: PacketData::Operator(vec![
                    Packet {
                        version: 6,
                        packet_type: 4,
                        data: PacketData::Literal(10)
                    },
                    Packet {
                        version: 2,
                        packet_type: 4,
                        data: PacketData::Literal(20)
                    },
                ])
            }
        );
    }

    #[test]
    fn test_parse_op1() {
        let p = Packet::new(&TEST_PACKET_OP_ID1);

        assert_eq!(
            p,
            Packet {
                version: 7,
                packet_type: 3,
                data: PacketData::Operator(vec![
                    Packet {
                        version: 2,
                        packet_type: 4,
                        data: PacketData::Literal(1)
                    },
                    Packet {
                        version: 4,
                        packet_type: 4,
                        data: PacketData::Literal(2)
                    },
                    Packet {
                        version: 1,
                        packet_type: 4,
                        data: PacketData::Literal(3)
                    },
                ])
            }
        );
    }

    #[test]
    fn test_parse_op_op_op() {
        let p = Packet::new(&TEST_PACKET_OP_OP_OP);

        assert_eq!(
            p,
            Packet {
                version: 4,
                packet_type: 2,
                data: PacketData::Operator(vec![Packet {
                    version: 1,
                    packet_type: 2,
                    data: PacketData::Operator(vec![Packet {
                        version: 5,
                        packet_type: 2,
                        data: PacketData::Operator(vec![Packet {
                            version: 6,
                            packet_type: 4,
                            data: PacketData::Literal(15)
                        },])
                    }])
                }])
            }
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(evaluate_packet(&Packet::new(&TEST_PACKET_SUM)), 3);
    }

    #[test]
    fn test_product() {
        assert_eq!(evaluate_packet(&Packet::new(&TEST_PACKET_PRODUCT)), 54);
    }

    #[test]
    fn test_min() {
        assert_eq!(evaluate_packet(&Packet::new(&TEST_PACKET_MIN)), 7);
    }

    #[test]
    fn test_max() {
        assert_eq!(evaluate_packet(&Packet::new(&TEST_PACKET_MAX)), 9);
    }

    #[test]
    fn test_gt() {
        assert_eq!(evaluate_packet(&Packet::new(&TEST_PACKET_GT)), 1);
    }

    #[test]
    fn test_lt() {
        assert_eq!(evaluate_packet(&Packet::new(&TEST_PACKET_LT)), 0);
    }

    #[test]
    fn test_eq() {
        assert_eq!(evaluate_packet(&Packet::new(&TEST_PACKET_EQ)), 0);
    }

    #[test]
    fn test_full() {
        assert_eq!(evaluate_packet(&Packet::new(&TEST_PACKET_FULL)), 1);
    }
}
