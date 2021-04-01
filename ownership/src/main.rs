fn main() {
    let s = String::from("hello");

    takes_ownership(s.clone());

    println!("{}", s);

    let x = 5;

    make_copy(x);
    
    println!("{}", x);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s:String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}
