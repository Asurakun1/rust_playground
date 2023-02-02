fn main() {
    let s = String::from("Hello world!");

    let first_word = first_word(&s);
    println!("{} {}", first_word, "world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}