use std::collections::HashMap;

// encode-decode - 1
#[must_use]
pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
    let mut ans = vec![first];
    ans.extend(encoded.iter().scan(first, |state, &x| {
        *state ^= x;
        Some(*state)
    }));
    ans
}

// (나를 위한 메모)
// `extend`는 원본 컬렉션을 직접 변경한다,`concat`은 여러 컬렉션의 요소를 새로운 컬렉션으로 결합하여 반환한다

// encode-decode - 2
#[must_use]
pub fn decompress_rl_elist(nums: &[i32]) -> Vec<i32> {
    nums.chunks(2)
        .flat_map(|chunk| {
            let length: Result<usize, _> = chunk[0].try_into();

            match length {
                Ok(v) => vec![chunk[1]; v],
                Err(err) => {
                    println!(
                        "{:?} The number of i32 type cannot convert to usize type. This function will return an empty vector.",
                        err
                    );
                    vec![]
                }
            }
        })
        .collect()
}

// encode-decode
#[must_use]
pub fn restore_string(s: &str, indices: &[usize]) -> String {
    let mut shuffled = vec![' '; s.len()];

    indices
        .iter()
        .zip(s.chars())
        .for_each(|(&index, ch)| shuffled[index] = ch);

    shuffled.into_iter().collect()
}

// encode-decode - 4
#[must_use]
pub fn decode_message(key: &str, message: &str) -> String {
    let mut d = HashMap::new();
    for c in key.as_bytes() {
        if *c == b' ' || d.contains_key(c) {
            continue;
        }
        d.insert(c, char::from((97 + d.len()) as u8));
    }

    message
        .as_bytes()
        .iter()
        .map(|c| d.get(c).unwrap_or(&' '))
        .collect()
}

// encode-decode - 5

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[must_use]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[must_use]
pub fn get_decimal_value(head: &[i32]) -> i32 {
    let mut vec_n = vec![];
    let _v = head.iter().for_each(|x| {
        if *x == 1 {
            vec_n.push("1");
        } else {
            vec_n.push("0");
        }
    });

    let decimal_result = i32::from_str_radix(&vec_n.join(""), 2);

    match decimal_result {
        Ok(num) => num,
        Err(err) => {
            println!("{:?} There is a problem to parse String type value to i32 number type. This function will return 0.", err);
            0
        }
    }
}
