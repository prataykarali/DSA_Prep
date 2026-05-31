use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut word_counts = HashMap::new();
    for word in text.split_whitespace() {
        let normalized = word.to_lowercase();
        let count = word_counts.entry(normalized).or_insert(0);
        *count += 1;
    }
    word_counts
}

fn sort_by_frequency(word_counts: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut sorted_words: Vec<(String, usize)> = word_counts.into_iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    sorted_words
}

fn print_bar_chart(sorted_words: &[(String, usize)], top_n: usize, max_bar_width: usize) {
    let top_words: Vec<&(String, usize)> = sorted_words.iter().take(top_n).collect();
    if top_words.is_empty() {
        println!("No words to display.");
        return;
    }

    let max_count = top_words[0].1;

    println!("\n{:<15} {:<width$}  {:>5}", "Word", "Bar", "Count", width = max_bar_width);
    println!("{}", "-".repeat(15 + max_bar_width + 9));
    for (word, count) in &top_words {
        let bar_length = (*count as f64 / max_count as f64 * max_bar_width as f64) as usize;
        let bar: String = "#".repeat(bar_length);
        println!("{:<15} {:<width$}  ({})", word, bar, count, width = max_bar_width);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    let text = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Error: File '{}' not found.", file_path);
            process::exit(1);
        }
    };

    println!("=== Word Frequency Analyzer (Rust) ===\n");

    let word_counts = count_words(&text);
    let total_unique = word_counts.len();
    let sorted_words = sort_by_frequency(word_counts);

    println!("Total unique words: {}", total_unique);
    println!("Showing top 10 most frequent words:\n");

    print_bar_chart(&sorted_words, 10, 20);
}