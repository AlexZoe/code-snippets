use std::env;

fn get_max_depth(input: &String) -> Option<i32> {
    let mut depth = 0;
    let mut max_depth = 0;
    for i in input.chars() {
        match &i {
            '(' => depth+=1,
            ')' => {
                if depth == 0 {
                    println!("Error: not balanced");
                    return None;
                } else {
                    depth-=1;
                }
            },
            _ => continue,
        }

        if depth > max_depth {
            max_depth = depth;
        }
    }

    if depth != 0 {
        println!("Error: not balanced");
        return None;
    }

    Some(max_depth)
}


fn get_max_depth_compact(input: &String) -> Option<i32> {
    input.chars()
        .filter(|&x| "()".contains(x))
        .map(|x| if let '(' = x { 1 } else {-1})
        .scan(0, |acc, x| { *acc += x; Some(*acc) })
        .max()
}

fn main() {
    let input: Vec<String> = env::args().collect();

    if input.len() < 2 {
        return;
    }

    println!("max depth is: {}", get_max_depth(&input[1]).unwrap());
    println!("max depth is: {}", get_max_depth_compact(&input[1]).unwrap());
}
