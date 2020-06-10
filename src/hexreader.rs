use std::fs;
use regex::Regex;

use std::str;

enum FieldNumber {
    BCField = 1,
    AddrField = 2,
    RTField = 3,
    DatField = 4,
    CSField = 5
}

impl FieldNumber {
    fn from_usize(value: usize) -> FieldNumber {
        match value {
            1 => FieldNumber::BCField,
            2 => FieldNumber::AddrField,
            3 => FieldNumber::RTField,
            4 => FieldNumber::DatField,
            5 => FieldNumber::CSField,
            _ => panic!("Unknown FieldNumber: {}", value)
        }
    }
}

#[derive(Debug)]
struct IhexLine {
    byte_count:     u8,
    address:        u16,
    record_type:    u8,
    data:           Vec<u8>,
    checksum:       u8,
}



fn split_ihex_line(line: &str) {
    if line.starts_with(":") {
        let mut ihexline = IhexLine{ byte_count: 0, 
                                     address: 0, 
                                     record_type: 0, 
                                     data: Vec::new(), 
                                     checksum: 0 };

        let re = Regex::new(r":(?P<byte_count>[[:xdigit:]]{2})(?P<address>[[:xdigit:]]{4})(?P<record_type>[[:xdigit:]]{2})(?P<data>[[:xdigit:]]+)?(?P<check_sum>[[:xdigit:]]{2})").unwrap();

        let caps = re.captures(line).unwrap();

        println!(
            "byte_count : {}\n\
             address    : {}\n\
             record_type: {}\n\
             data       : {}\n\
             check_sum  : {}\n\
            ",
            caps.get(1).map_or("None", |m| m.as_str()),
            caps.get(2).map_or("None", |m| m.as_str()),
            caps.get(3).map_or("None", |m| m.as_str()),
            caps.get(4).map_or("None", |m| m.as_str()),
            caps.get(5).map_or("None", |m| m.as_str())
        );

        for i in 1..6 {
            let field = caps.get(i).map_or("None", |m| m.as_str());
            
            match FieldNumber::from_usize(i) {
                FieldNumber::BCField => {
                    if field != "None" {
                        ihexline.byte_count = u8::from_str_radix(field, 16).unwrap()
                    }
                },

                FieldNumber::AddrField => {
                    if field != "None" {
                        let addr = u16::from_str_radix(field, 16).unwrap();
                        ihexline.address = addr
                    }
                },

                FieldNumber::RTField => {
                    if field != "None" {
                        ihexline.record_type = u8::from_str_radix(field, 16).unwrap()
                    }
                },

                FieldNumber::DatField => {
                    if field != "None" {
                        let subs = field.as_bytes()
                                        .chunks(2)
                                        .map(str::from_utf8)
                                        .collect::<Result<Vec<&str>, _>>()
                                        .unwrap();
                        
                        for s in subs {
                            ihexline.data.push(
                                u8::from_str_radix(s, 16).unwrap()
                            )
                        }
                    }
                },

                FieldNumber::CSField => {
                    if field != "None" {
                        ihexline.checksum = u8::from_str_radix(field, 16).unwrap()
                    }
                }
            }
        }

        println!("{:?}\n-----", ihexline)

    }
    else {
        panic!("Encountered {}, but line does not start with ':'", line);
    }

}

pub fn read_ihex(path: &str) {
    let data = fs::read_to_string(path).expect("Cannot read file");

    for line in data.lines() {
        split_ihex_line(line);
        //println!("{}", line);
    }

    //println!("{}", data);
}