// import trait_impl.rs
mod trait_impl;
// import trait
use crate::trait_impl::Summary;

// only allow types that implement PartialOrd
fn largest<T>(list: &[T]) -> &T
    where T: std::cmp::PartialOrd
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// take types which implement PartialOrd and Copy trait
fn smallest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut smallest = list[0];

    // item requires copy trait
    for &item in list {
        if item < smallest {
            smallest = item;
        }
    }

   smallest
}

fn call_templated_func() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn main() {
    call_templated_func();
    let tweet = trait_impl::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

