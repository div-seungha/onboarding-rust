// String - 1
#[must_use]
pub fn defang_i_paddr(address: &str) -> String {
    address.replace("'.'", "'[.]'")
}

// String - 2
#[must_use]
pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> usize {
    let mut ans = 0;

    for jewel in jewels.chars() {
        ans += stones.chars().filter(|&stone| stone == jewel).count();
    }

    ans
}

// String - 3
#[must_use]
pub fn most_words_found(sentences: Vec<String>) -> Result<usize, String> {
    if let Some(v) = sentences.iter().map(|x| x.split_whitespace().count()).max() {
        Ok(v)
    } else {
        Err(format!("The maximum number of words that appear in a single sentence is not exist. Check the input."))
    }
}

// String - 4
#[must_use]
pub fn sort_sentence(s: &str) -> String {
    let mut words: Vec<&str> = vec![""; s.split_whitespace().count()];

    s.split_whitespace()
        .map(|word| {
            let a = word.chars().last();
            let b = (match a {
                Some(v) => v,
                None => ' ',
            })
            .to_digit(10);

            let c = match b {
                Some(v) => v as usize,
                None => 0,
            };

            (&word[..word.len() - 1], c - 1)
        })
        .for_each(|(word, i)| words[i] = word);

    words.join(" ")
}

// String - 5
#[must_use]
pub fn count_matches(items: Vec<Vec<String>>, rule_key: &str, rule_value: &str) -> usize {
    let idx = match rule_key {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => {
            format!("The rule_key of this input is wrong. The rule_key must be one of these values: 'type', 'color', or 'name'.");
            0
        }
    };

    items.iter().filter(|item| item[idx] == rule_value).count()
}
