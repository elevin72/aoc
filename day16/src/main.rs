use std::collections::HashMap;

const INPUT: &str = "020D78804D397973DB5B934D9280CC9F43080286957D9F60923592619D3230047C0109763976295356007365B37539ADE687F333EA8469200B666F5DC84E80232FC2C91B8490041332EB4006C4759775933530052C0119FAA7CB6ED57B9BBFBDC153004B0024299B490E537AFE3DA069EC507800370980F96F924A4F1E0495F691259198031C95AEF587B85B254F49C27AA2640082490F4B0F9802B2CFDA0094D5FB5D626E32B16D300565398DC6AFF600A080371BA12C1900042A37C398490F67BDDB131802928F5A009080351DA1FC441006A3C46C82020084FC1BE07CEA298029A008CCF08E5ED4689FD73BAA4510C009981C20056E2E4FAACA36000A10600D45A8750CC8010989716A299002171E634439200B47001009C749C7591BD7D0431002A4A73029866200F1277D7D8570043123A976AD72FFBD9CC80501A00AE677F5A43D8DB54D5FDECB7C8DEB0C77F8683005FC0109FCE7C89252E72693370545007A29C5B832E017CFF3E6B262126E7298FA1CC4A072E0054F5FBECC06671FE7D2C802359B56A0040245924585400F40313580B9B10031C00A500354009100300081D50028C00C1002C005BA300204008200FB50033F70028001FE60053A7E93957E1D09940209B7195A56BCC75AE7F18D46E273882402CCD006A600084C1D8ED0E8401D8A90BE12CCF2F4C4ADA602013BC401B8C11360880021B1361E4511007609C7B8CA8002DC32200F3AC01698EE2FF8A2C95B42F2DBAEB48A401BC5802737F8460C537F8460CF3D953100625C5A7D766E9CB7A39D8820082F29A9C9C244D6529C589F8C693EA5CD0218043382126492AD732924022CE006AE200DC248471D00010986D17A3547F200CA340149EDC4F67B71399BAEF2A64024B78028200FC778311CC40188AF0DA194CF743CC014E4D5A5AFBB4A4F30C9AC435004E662BB3EF0";

const SUM: u32 = 0;
const PRODUCT: u32 = 1;
const MINIMUM: u32 = 2;
const MAXIMUM: u32 = 3;
const LITERAL: u32 = 4;
const GREATER_THAN: u32 = 5;
const LESS_THAN: u32 = 6;
const EQUALS: u32 = 7;

const OFFSET_BITLENGTH: usize = 22;
const OFFSET_COUNT: usize = 18;

fn main() {
    println!("Hello, world!");

    println!("{}", star31(String::from(INPUT)));

    println!("{}", star32(String::from(INPUT)));
}

struct PacketReturn {
    version_sum: u32,
    ends_at: usize,
    value: u64,
}

fn star31(mut input: String) -> u32 {
    input = to_binary(input);
    parse_packet(&input).version_sum
}

fn star32(mut input: String) -> u64 {
    input = to_binary(input);
    parse_packet(&input).value
}

fn parse_packet(input: &str) -> PacketReturn {
    let mut packet_version = to_decimal(&input[0..3]) as u32;
    let packet_type = to_decimal(&input[3..6]) as u32;

    if packet_type == LITERAL {
        let mut value = String::new();
        let mut idx = 6;
        loop {
            let leading_bit = &input[idx..idx + 1];
            value += &input[idx + 1..idx + 5];
            if leading_bit == "0" {
                return PacketReturn {
                    version_sum: packet_version,
                    ends_at: idx + 5,
                    value: to_decimal(&value),
                };
            }
            idx += 5;
        }
    }

    let mut offset = 0;
    let mut children_values = vec![];

    if to_decimal(&input[6..7]) == 0 {
        // children by bit length
        let children_length = to_decimal(&input[7..22]) as usize;
        offset = OFFSET_BITLENGTH;

        loop {
            let PacketReturn {
                version_sum,
                ends_at,
                value,
            } = parse_packet(&input[offset..]);
            packet_version += version_sum;
            offset += ends_at;
            children_values.push(value);

            if offset == OFFSET_BITLENGTH + children_length {
                break;
            }
        }
    } else {
        // children by number
        let children_count = to_decimal(&input[7..18]);
        offset = OFFSET_COUNT;

        for _ in 0..children_count {
            let PacketReturn {
                version_sum,
                ends_at,
                value,
            } = parse_packet(&input[offset..]);
            packet_version += version_sum;
            offset += ends_at;
            children_values.push(value);
        }
        //             let children_count = to_decimal(&input[7..18]);
        //             let mut offset = 18;
        //             for _ in 0..children_count {
        //                 let child = parse_packet1(&input[offset..]);
        //                 packet_version += child.version_sum;
        //                 offset += child.ends_at;
        //             }
        //             return PacketReturn {
        //                 version_sum: packet_version,
        //                 ends_at: offset,
        //                 value: 0,
        //             };
        //         }
        //     }
    }

    let value: u64 = match packet_type {
        SUM => children_values.iter().sum::<u64>(),
        PRODUCT => children_values.iter().product::<u64>(),
        MINIMUM => *children_values.iter().min().unwrap(),
        MAXIMUM => *children_values.iter().max().unwrap(),
        GREATER_THAN => {
            if children_values[0] > children_values[1] {
                1
            } else {
                0
            }
        }
        LESS_THAN => {
            if children_values[0] < children_values[1] {
                1
            } else {
                0
            }
        }
        EQUALS => {
            if children_values[0] == children_values[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("damn"),
    };

    return PacketReturn {
        version_sum: packet_version,
        ends_at: offset,
        value,
    };
}

// for part 1
// fn parse_packet1(input: &str) -> PacketReturn {
//     let mut packet_version = to_decimal(&input[0..3]);
//     let packet_type = to_decimal(&input[3..6]);
//     if packet_type == 4 {
//         let mut _value = 0;
//         let mut next_chunk = 6;
//         loop {
//             let start = &input[next_chunk..next_chunk + 1];
//             _value += to_decimal(&input[next_chunk + 1..next_chunk + 5]);
//             if start == "0" {
//                 return PacketReturn {
//                     version_sum: packet_version,
//                     ends_at: next_chunk + 5,
//                     value: 0,
//                 };
//             }
//             next_chunk += 5;
//         }
//     } else {
//         let length_type_id = to_decimal(&input[6..7]);
//         if length_type_id == 0 {
//             let children_length = to_decimal(&input[7..22]) as usize;
//             let mut offset = 22;
//             loop {
//                 let child = parse_packet1(&input[offset..]);
//                 packet_version += child.version_sum;
//                 offset += child.ends_at;
//                 if offset == 22 + children_length {
//                     return PacketReturn {
//                         version_sum: packet_version,
//                         ends_at: offset,
//                         value: 0,
//                     };
//                 }
//             }
//         } else {
//             // == 1
//             let children_count = to_decimal(&input[7..18]);
//             let mut offset = 18;
//             for _ in 0..children_count {
//                 let child = parse_packet1(&input[offset..]);
//                 packet_version += child.version_sum;
//                 offset += child.ends_at;
//             }
//             return PacketReturn {
//                 version_sum: packet_version,
//                 ends_at: offset,
//                 value: 0,
//             };
//         }
//     }
// }

fn to_decimal(bits: &str) -> u64 {
    bits.chars()
        .rev()
        .fold((0, 0), |(mut val, place), c| {
            val = val + (u64::pow(2, place) * c.to_digit(10).unwrap() as u64);
            (val, place + 1)
        })
        .0
}

fn to_binary(input: String) -> String {
    let values = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);
    input.chars().map(|c| values[&c]).collect()
}

#[cfg(test)]
mod tests {
    // use test::Bencher;

    use super::*;

    #[test]
    fn test1_part1() {
        let input = String::from("8A004A801A8002F478");
        let g = star31(input);
        assert_eq!(g, 16);
    }

    // #[test]
    // fn test2_part1() {
    //     let input = String::from("620080001611562C8802118E34");
    //     let g = star26(input);
    //     assert_eq!(g, 12);
    // }

    // #[test]
    // fn test3_part1() {
    //     let input = String::from("C0015000016115A2E0802F182340");
    //     let g = star26(input);
    //     assert_eq!(g, 23);
    // }

    // #[test]
    // fn test4_part1() {
    //     let input = String::from("A0016C880162017C3686B18A3D4780");
    //     let g = star26(input);
    //     assert_eq!(g, 31);
    // }
}
