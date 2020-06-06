use std::fs;
use regex::Regex;

struct IhexLine {
    byte_count:     u8,
    address:        u16,
    record_type:    u8,
    data:           Vec<u8>,
    checksum:       u8,
}

fn split_ihex_line(line: &str) {
    if line.starts_with(":") {
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