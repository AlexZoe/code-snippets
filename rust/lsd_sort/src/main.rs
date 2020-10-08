use std::env;

const ALPHABET: usize = 256;

fn sort(items: &mut Vec<String>, width: &usize) -> Vec<String> {
    let num_of_items = items.len();
    let mut tmp_items: Vec<String> = vec!(String::from(""); num_of_items);

    for d in width-1 .. 0 {
        let mut count: [u32; ALPHABET+1] = [0; ALPHABET+1];

        for i in 0 .. num_of_items {
            count[(items[i].as_bytes()[d as usize] + 1) as usize] += 1;
        }

        for r in 0 .. ALPHABET {
            count[(r+1) as usize] += count[r as usize];
        }

        for i in 0 .. *width as u32 {
            tmp_items[(count[items[i as usize].as_bytes()[d as usize] as usize] + 1) as usize] = items[i as usize].clone();
        }
    }

    tmp_items
}

fn main() {
    let mut items: Vec<String> = env::args().collect();
    println!("got {} items", items.len());

    let width = items[0].len();

    for i in &items {
        println!("{}", i);
    }

    let sorted_items = sort(&mut items, &width);

    for i in sorted_items {
        println!("{}", i);
    }
}
