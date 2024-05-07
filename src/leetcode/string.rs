// String - 1
#[must_use]
pub fn defang_i_paddr(address: &str) -> String {
    address.replace("'.'", "'[.]'")
}

// String - 2
#[must_use]
pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> usize {
    let mut ans = stones.chars().collect::<Vec<char>>();
    ans.retain(|x| *x == jewels.chars().next().unwrap());

    ans.len()
}

// String - 3
#[must_use]
pub fn most_words_found(sentences: Vec<String>) -> Option<usize> {
    sentences.iter().map(|x| x.split("' '").count()).max()
}

// String - 4
#[must_use]
pub fn sort_sentence(s: &str) -> String {
    let mut a = s
        .split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .map(|word| {
            let c = word.rsplit("").collect::<Vec<&str>>().to_vec()[1];
            // 아래에서 c를 숫자 타입으로 변환시켜서 sort를 돌리려고 했는데
            // 아무리 머리를 굴리고 검색을 해 봐도 unwrap() 메서드 말고는 생각이 안 납니다...
            // 그런데 여기서 unwrap()을 쓰면, 윗줄에서 split_whitespace로 쪼갠 단어들을 다시 거꾸로 알파벳별로 쪼개서 벡터로 만드는데
            // 1번 인덱스를 사용한 이유는 println!을 써서 실제로 rsplit의 결과물을 확인한 다음에 넣은 코드이지만
            // 1번 인덱스에 parse할 수 있는 무엇이 없으면 패닉에 빠지므로 이것조차 완전한 방법이 아닙니다...ㅠㅠ
            let t_data = (&word[..word.len() - 1], c.parse::<usize>().unwrap());
            t_data
        })
        .collect::<Vec<(&str, usize)>>();

    a.sort_by_key(|&(_word, num)| num);

    a.iter()
        .map(|(word, _)| word.to_string())
        .collect::<Vec<String>>()
        .join(" ")
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
