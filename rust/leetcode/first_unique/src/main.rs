use std::env;
use std::collections::HashMap;

fn parse_args() -> String {
    let mut args = env::args();
    // Program name
    args.next();

    let input_string = match args.next() {
        Some(arg)   => arg,
        None        => String::from("")
    };
    input_string
}


struct Info {
    pos: i32,
    count: i32,
}


fn find_unique_use_hash(input_string: String) -> i32 {
    let mut char_map: std::collections::HashMap<char, Info> = HashMap::new();

    for (pos, ch) in input_string.chars().enumerate() {
        if char_map.contains_key(&ch) {
            if let Some(entry) = char_map.get_mut(&ch) {
                (*entry).count += 1;
            }
        } else {
            char_map.insert(ch, Info {pos: pos as i32, count: 1});
        }
    }

    let mut pos = std::i32::MAX;
    for (ch, info) in char_map {
        println!("Processing {}", &ch);
        if info.count == 1 {
            pos = std::cmp::min(pos, info.pos);
        }
    }

    if pos != std::i32::MAX {
        println!("Found unique character at {}", pos);
        return pos;
    } else {
        println!("Found no unique character");
        return -1;
    }
}


fn find_unique_use_vec(input_string: String) -> i32 {
    let mut counts = vec![0; 26];
    for ch in input_string.chars() {
        counts[ch as usize - 'a' as usize] += 1;
    }

    for (idx, ch) in input_string.chars().enumerate() {
        if counts[ch as usize - 'a' as usize] == 1 {
            return idx as i32;
        }
    }
    -1
}


fn main() {
    let ret = find_unique_use_vec(parse_args());
}
