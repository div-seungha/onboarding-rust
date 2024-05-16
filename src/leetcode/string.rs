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
    let mut a = s.split_whitespace().collect::<Vec<&str>>();
    a.sort_by_key(|v| {
        v.chars()
            .last()
            .expect("Each word has a fixed length. It must be return the last character.")
            .to_digit(10)
            .expect("The last character of each word must be a decimal number.")
    });

    a.iter()
        .map(|word| &word[..word.len() - 1])
        .collect::<Vec<&str>>()
        .join(" ")
}

// String - 5
#[must_use]
pub fn count_matches(
    items: Vec<Vec<String>>,
    rule_key: &str,
    rule_value: &str,
) -> Result<usize, String> {
    let idx = match rule_key {
        "type" => 0,
        "color" => 1,
        "name" => 2,
        _ => return Err(format!("{}: The rule_key is invalid. The rule_key must be one of these values: 'type', 'color', or 'name'.", rule_key)),
    };

    Ok(items.iter().filter(|item| item[idx] == rule_value).count())
}
