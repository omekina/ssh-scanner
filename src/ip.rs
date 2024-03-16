pub fn parse(raw: &str) -> u32 {
    let raw_split: Vec<&str> = raw.split(".").collect();
    if raw_split.len() != 4 {
        panic!("Could not parse IP");
    }
    let mut factor: u32 = 0;
    let mut result: u32 = 0;
    for group in raw_split.iter().rev() {
        factor = match factor {
            0 => 1,
            _ => factor * 256,
        };
        let val: u32 = group.parse().unwrap();
        result += val * factor;
    }
    result
}


pub fn build(mut binary: u32) -> String {
    let mut groups: Vec<String> = Vec::new();
    let mut group_id: usize = 0;
    while group_id < 4 {
        groups.push((binary % 256).to_string());
        binary /= 256;
        group_id += 1;
    }
    groups.reverse();
    groups.join(".")
}


/**
* Both ends are inclusive
* Returns (subnet start, subnet end) 
*/
pub fn get_subnet_bounds(base_ip: u32, mask: u8) -> (u32, u32) {
    if mask > 32 {
        panic!("Invalid mask");
    }
    let mask: u32 = (1 << (32 - mask)) - 1;
    let start_ip = base_ip & (!mask);
    (start_ip, start_ip + mask)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ip_parse() {
        assert_eq!(parse(&"0.0.0.1".to_string()), 1);
        assert_eq!(parse(&"255.255.255.255".to_string()), 4294967295);
    }

    #[test]
    fn ip_build() {
        assert_eq!(build(1), "0.0.0.1".to_string());
        assert_eq!(build(4294967295), "255.255.255.255");
    }

    #[test]
    fn subnet_bounds() {
        assert_eq!(get_subnet_bounds(parse(&"192.168.1.42".to_string()), 24), (parse(&"192.168.1.0".to_string()), parse(&"192.168.1.255".to_string())));
        assert_eq!(get_subnet_bounds(69, 32), (69, 69));
    }
}
