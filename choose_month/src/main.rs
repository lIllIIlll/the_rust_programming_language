use std::io;

fn main() {
    let months: Vec<String> = vec![
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december"]
        .into_iter()
        .map(String::from)
        .map(|word| word.to_uppercase())
        .collect();

    let mut your_month = String::new();

    io::stdin()
        .read_line(&mut your_month)
        .expect("Failed to read your month");

    let your_month: usize = your_month
        .trim()
        .parse()
        .expect("Your month entered was not a number");

    if your_month > 12 || your_month <= 0 {
        return println!("Wrong month!");
    }

    let element = &months[your_month - 1];

    println!(
        "Month choosed is {}",
                    element
    );
}
