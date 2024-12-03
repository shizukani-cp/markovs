use markov::Chain;
use std::io;

fn main() {
    let mut chain = Chain::new(3);

    let mut input = String::new();
    while let Ok(n) = io::stdin().read_line(&mut input) {
        if n == 0 {
            break;
        }
        let words: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        for window in words.windows(4) {
            if window.len() == 4 {
                let key = format!("{} {} {}", window[0], window[1], window[2]);
                chain.feed(key, window[3].clone());
            }
        }
        input.clear();
    }

    loop {
        let mut current_state = chain.random_state();
        let mut generated_text = current_state.clone();
        for _ in 0..100 {
            if let Some(next_word) = chain.next(&current_state) {
                generated_text.push_str(next_word);
                let parts: Vec<&str> = current_state.split_whitespace().collect();
                current_state = format!("{} {} {}", parts[1], parts[2], next_word);
            } else {
                break;
            }
        }
        println!("{}", generated_text);
    }
}
