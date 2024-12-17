fn main() {
    println!("{}", pig("apple"));
    println!("{}", pig("first"));
}

fn pig(n: &str) -> String {
    let vowels: Vec<char> = vec!['a', 'i', 'u', 'e', 'o'];
    let mut chars = n.chars();

    match chars.next() {
        Some(first_char) if vowels.contains(&first_char) => {
            format!("{n}-ay")
        }

        Some(first_char) => {
            let rest: String = chars.collect();
            format!("{rest}-{first_char}ay")
        }
        None => String::new(),
    }
}
