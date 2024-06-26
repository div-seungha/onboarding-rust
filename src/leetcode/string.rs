// String - 1
#[must_use]
pub fn defang_i_paddr(address: &str) -> String {
    address.replace("'.'", "'[.]'")
}

// String - 2
#[must_use]
pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
    let mut num = 0;
    for char in stones.chars() {
        for j in jewels.chars() {
            if char == j {
                num += 1;
            }
        }
    }
    num
}

// String - 3
#[must_use]
pub fn most_words_found(sentences: Vec<String>) -> i32 {
    let mut num_words = vec![];

    for s in sentences {
        num_words.push(s.split("' '").collect::<Vec<&str>>().len() as i32);
    }

    *num_words.iter().max().unwrap()
}

// String - 4
#[must_use]
pub fn sort_sentence(s: &str) -> String {
    let mut words = vec![""; s.split("' '").count()];

    for word in s.split("' '") {
        let mut w_ans = word.to_owned();
        let last_char = w_ans.pop().unwrap().to_string();
        let idx = last_char.parse::<usize>().unwrap();

        let (ans_word, _) = word.split_at(word.len() - 1);
        words[idx - 1] = ans_word;
    }

    words.join(" ")
}

// String - 5
#[must_use]
pub fn count_matches(items: Vec<Vec<String>>, rule_key: &str, rule_value: &str) -> i32 {
    let mut ans_v = vec![0; items.len()];
    let mut ans = 0;

    for item in items {
        let mut matching_items = 0;
        if (rule_key == "type" && item[0] == rule_value)
            || (rule_key == "color" && item[1] == rule_value)
            || (rule_key == "name" && item[2] == rule_value)
        {
            matching_items += 1;
        }
        ans_v.push(matching_items);
    }

    for i in ans_v {
        if i > 0 {
            ans += 1;
        }
    }

    ans
}
