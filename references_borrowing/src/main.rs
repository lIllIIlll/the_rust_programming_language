fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    change(r3);
    println!("{}", r3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
