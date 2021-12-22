use crate::lib;

const HEADER_LEN: usize = 6;
const LITERAL_VALUE_LEN: usize = 5;
const SUB_PACKET_TYPE_0_LEN: usize = 15;
const SUB_PACKET_TYPE_1_LEN: usize = 11;
const SUB_PACKET_TYPE_0_END: usize = HEADER_LEN + 1 + SUB_PACKET_TYPE_0_LEN;
const SUB_PACKET_TYPE_1_END: usize = HEADER_LEN + 1 + SUB_PACKET_TYPE_1_LEN;

#[derive(Debug)]
enum PacketType {
    SUM,
    PROD,
    MIN,
    MAX,
    LIT,
    GT,
    LT,
    EQ,
}

impl PacketType {
    pub fn from_u8(i: u8) -> Result<PacketType, String> {
        match i {
            0 => Ok(PacketType::SUM),
            1 => Ok(PacketType::PROD),
            2 => Ok(PacketType::MIN),
            3 => Ok(PacketType::MAX),
            4 => Ok(PacketType::LIT),
            5 => Ok(PacketType::GT),
            6 => Ok(PacketType::LT),
            7 => Ok(PacketType::EQ),
            _ => return Err("unknown packet type".to_string()),
        }
    }

    pub fn eval(&self, p: &Packet) -> usize {
        let iter = p.sub_packets.iter();
        match self {
            PacketType::LIT => p.value,
            PacketType::SUM => iter.fold(0, |acc, c| acc + c.value()),
            PacketType::PROD => iter.fold(1, |acc, c| acc * c.value()),
            PacketType::MIN => iter.map(|p| p.value()).min().unwrap_or_default(),
            PacketType::MAX => iter.map(|p| p.value()).max().unwrap_or_default(),
            PacketType::GT => (p.sub_packets[0].value() > p.sub_packets[1].value()) as usize,
            PacketType::LT => (p.sub_packets[0].value() < p.sub_packets[1].value()) as usize,
            PacketType::EQ => (p.sub_packets[0].value() == p.sub_packets[1].value()) as usize,
        }
    }
}

#[derive(Debug)]
struct Header {
    version: u8,
    r#type: PacketType,
}

impl Header {
    pub fn from_bits(bits: &[bool]) -> Result<Header, String> {
        if bits.len() < HEADER_LEN {
            Err("not enough bits to parse header".to_string())
        } else {
            Ok(Header {
                version: lib::bits_to_usize(&bits[0..HEADER_LEN / 2]) as u8,
                r#type: PacketType::from_u8(
                    lib::bits_to_usize(&bits[HEADER_LEN / 2..HEADER_LEN]) as u8
                )?,
            })
        }
    }
}

#[derive(Debug)]
struct Packet {
    header: Header,
    length: usize,
    sub_packets: Vec<Packet>,
    value: usize,
}

impl Packet {
    pub fn from_hex(s: &str) -> Result<Packet, String> {
        Packet::from_bits(&hex_to_bits(s))
    }

    pub fn from_bits(bits: &[bool]) -> Result<Packet, String> {
        let header = Header::from_bits(&bits)?;

        let mut length = HEADER_LEN;
        let mut sub_packets: Vec<Packet> = vec![];
        let mut value: usize = 0;

        match header.r#type {
            PacketType::LIT => {
                // Literal value
                let mut v_bits: Vec<bool> = vec![];
                for i in (HEADER_LEN..bits.len()).step_by(LITERAL_VALUE_LEN) {
                    length += LITERAL_VALUE_LEN;
                    for j in i + 1..i + LITERAL_VALUE_LEN {
                        v_bits.push(bits[j]);
                    }
                    if !bits[i] {
                        break;
                    }
                }
                value = lib::bits_to_usize(&v_bits);
            }
            _ => {
                // Sub-packets

                length += 1;

                if bits[HEADER_LEN] {
                    // 11 bits, number of sub-packets
                    length += SUB_PACKET_TYPE_1_LEN;
                    while sub_packets.len()
                        < lib::bits_to_usize(&bits[HEADER_LEN + 1..SUB_PACKET_TYPE_1_END])
                    {
                        let p = Packet::from_bits(&bits[length..])?;
                        length += p.length;
                        sub_packets.push(p);
                    }
                } else {
                    // 15 bits, total length in bits of sub-packets
                    length += SUB_PACKET_TYPE_0_LEN;
                    while length
                        < SUB_PACKET_TYPE_0_END
                            + lib::bits_to_usize(&bits[HEADER_LEN + 1..SUB_PACKET_TYPE_0_END])
                    {
                        let p = Packet::from_bits(&bits[length..])?;
                        length += p.length;
                        sub_packets.push(p);
                    }
                };
            }
        }

        Ok(Packet {
            header,
            length,
            value,
            sub_packets,
        })
    }

    pub fn version_sum(&self) -> usize {
        self.header.version as usize
            + self
                .sub_packets
                .iter()
                .fold(0, |acc, p| acc + p.version_sum())
    }

    pub fn value(&self) -> usize {
        self.header.r#type.eval(self)
    }
}

pub fn hex_to_bits(s: &str) -> Vec<bool> {
    s.chars()
        .filter_map(|c| match c {
            '0' => Some("0000"),
            '1' => Some("0001"),
            '2' => Some("0010"),
            '3' => Some("0011"),
            '4' => Some("0100"),
            '5' => Some("0101"),
            '6' => Some("0110"),
            '7' => Some("0111"),
            '8' => Some("1000"),
            '9' => Some("1001"),
            'A' => Some("1010"),
            'B' => Some("1011"),
            'C' => Some("1100"),
            'D' => Some("1101"),
            'E' => Some("1110"),
            'F' => Some("1111"),
            _ => None,
        })
        .flat_map(|s| s.chars().map(|c| c == '1'))
        .collect()
}

pub fn puzzle1(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    let p = Packet::from_hex(&input[0]).unwrap();

    p.version_sum()
}

pub fn puzzle2(input_filename: &str) -> usize {
    let input = lib::read_lines(input_filename);

    let p = Packet::from_hex(&input[0]).unwrap();

    p.value()
}

#[cfg(test)]
mod tests {
    use crate::y2021::day16;

    #[test]
    fn test1() {
        assert_eq!(day16::puzzle1("src/y2021/day16/test.txt"), 16);
        assert_eq!(day16::puzzle1("src/y2021/day16/input.txt"), 860);
    }

    #[test]
    fn test2() {
        assert_eq!(day16::puzzle2("src/y2021/day16/test2.txt"), 3);
        assert_eq!(day16::puzzle2("src/y2021/day16/test3.txt"), 54);
        assert_eq!(day16::puzzle2("src/y2021/day16/test4.txt"), 7);
        assert_eq!(day16::puzzle2("src/y2021/day16/test5.txt"), 9);
        assert_eq!(day16::puzzle2("src/y2021/day16/test6.txt"), 1);
        assert_eq!(day16::puzzle2("src/y2021/day16/test7.txt"), 0);
        assert_eq!(day16::puzzle2("src/y2021/day16/test8.txt"), 0);
        assert_eq!(day16::puzzle2("src/y2021/day16/test9.txt"), 1);
        assert_eq!(day16::puzzle2("src/y2021/day16/input.txt"), 470949537659);
    }
}
