use getch::Getch;

const LENGTH: usize = 6;

#[allow(dead_code)]
enum Endian {
    Big,
    Little
}

fn main() {
    let g = Getch::new();
    let mut bytes = vec![];

    for i in 0..LENGTH {
        bytes.push(0_u8);    

        display_byte(bytes[i]);

        for _ in 0..8 {
            let mut input = g.getch().unwrap();
            while !(input == 49 || input == 48) {
                input = g.getch().unwrap();
            }

            bytes[i] = (bytes[i] << 1) | if input == 49 { 1 } else { 0 };
            display_byte(bytes[i]);
        }

        println!("{:02X?}: {}", bytes, decode_utf8(&bytes, false));

        let u16bytes = u8_to_u16(&bytes, Endian::Big);
        println!("{:04X?}: {}\n", u16bytes, String::from_utf16_lossy(&u16bytes));
    }
}


fn decode_utf8(vec: &Vec<u8>, by_char: bool) -> String {
    if by_char {
        vec.iter().map(|x| 
            match *x {
                0..=0x20 => String::from("□"),
                0x21..=0x7E => String::from_utf8(vec![*x]).unwrap(),
                0x7F..=0xFF => String::from("�"),
            }
        ).collect()
    } else {
        String::from_utf8_lossy(vec).to_string()
    }
}

fn u8_to_u16(vec: &Vec<u8>, e: Endian) -> Vec<u16> {
    match e {
        Endian::Little => (0..vec.len()-1).step_by(2).map(|i| vec[i] as u16 + ((vec[i+1] as u16) << 8)).collect(),
        Endian::Big => (0..vec.len()-1).step_by(2).map(|i| vec[i+1] as u16 + ((vec[i] as u16) << 8)).collect()
    }
}

fn display_byte(byte: u8) {
    println!("{0:<3} ({0:#010b}) ({0:#04X}) {1}", byte, decode_utf8(&vec![byte], true));
} 
