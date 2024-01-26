fn main() {
    let sample_text =
        "Rust is an amazing programming language with a focus on safety and performance io";

    let word_count = count_words(sample_text);
    let char_count = count_characters(sample_text);

    println!("Original Text: {}", sample_text);
    println!("Word Count: {}", word_count);
    println!("Character Count: {}", char_count);

    let contains_rust = contains_keyword(sample_text, "Rust");
    println!("Contains 'Rust': {}", contains_rust);

    let char_array: Vec<char> = sample_text.chars().collect();
    let first_few_chars = &char_array[0..5];
    println!("First Few Characters: {:?}", first_few_chars);

    let has_long_word = has_long_words(sample_text);
    println!("Contains Long Word : {}", has_long_word);

    let average_word_length = char_count as f64 / word_count as f64;
    println!("Average Word Length : {:.4}", average_word_length)
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_characters(text: &str) -> usize {
    text.chars().count()
}

fn contains_keyword(text: &str, keyword: &str) -> bool {
    text.contains(keyword)
}

fn has_long_words(text: &str) -> bool {
    text.split_whitespace().any(|word| word.len() > 20)
}
