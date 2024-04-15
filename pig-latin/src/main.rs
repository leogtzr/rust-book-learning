fn to_pig_latin(word: String) -> String {
    let mut piglatin = String::new();

    if let Some((_, _)) = word.char_indices().next() {
        let first_char = word.chars().nth(0).unwrap_or('\0');
        let remaining_word: String = word[0 + 1..].to_string();

        let is_vowel = match first_char {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false
        };

        if is_vowel {
            let formatted = format!("{}-hay", word);
            piglatin.push_str(&formatted);
        } else {
            let formatted = format!("{}-{}ay", remaining_word, first_char);
            piglatin.push_str(&formatted);
        }
    }

    piglatin
}

fn main() {
    let text = "mi nombre es Leonardo";

    for word in text.split_whitespace() {
        println!("{}", to_pig_latin(word.to_string()));
    }
}
