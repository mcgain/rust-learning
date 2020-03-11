use std::io;

fn main() {
    println!("Pig Latin!\n\nGimmie some words: ");
    let input = read_stdin().trim_end().to_string();

    let words = split_words(&input);

    let mutated_words: Vec<String> = words.iter().map(|word| piggify(&word)).collect();

    print!("{}", mutated_words.join(" "));
}

fn piggify(word: &str) -> String {
    let mut chars = word.chars();
    let first_char = chars.nth(0);
    match first_char {
        Some(c) => {
            if is_vowell(c) {
                return word.to_owned() + "tay";
            } else {
                let d = c.to_string().to_owned();
                let mut m = word[1..].to_owned();
                m.push_str(&d);
                m.push_str("ay");
                return m.to_owned();
            }
        }
        None => return word.to_owned(),
    }
}

fn is_vowell(c: char) -> bool {
    ['a', 'e', 'i', 'o', 'u'].to_vec().contains(&c)
}

fn split_words(input: &String) -> Vec<&str> {
    input.split(' ').collect()
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("you probably should type something");
    buffer
}
