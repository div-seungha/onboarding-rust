use std::collections::HashMap;

// encode-decode - 1
#[must_use]
pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
    encoded.iter().fold(vec![first], |mut acc, &x| {
        let last = acc.last().cloned();
        let v = match last {
            Some(v) => v,
            None => 0,
        };
        acc.push(v ^ x);
        acc
    })
}

// encode-decode - 2
#[must_use]
pub fn decompress_rl_elist(nums: &[i32]) -> Vec<i32> {
    nums.chunks(2)
        .flat_map(|chunk| vec![chunk[1]; chunk[0] as usize])
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
        .map(|c| match d.get(c) {
            Some(v) => v,
            None => &' ',
        })
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
pub fn get_decimal_value(head: &Option<Box<ListNode>>) -> i32 {
    let mut vec_n = vec![];
    let _v = head.iter().map(|x| {
        let x = if x.val == 1 { "1" } else { "0" };
        vec_n.push(x);
        x
    });

    let decimal_result = i32::from_str_radix(&vec_n.join(""), 2);

    match decimal_result {
        Ok(num) => num,
        Err(err) => {
            println!("Something is wrong_{:?}", err);
            0
        }
    }
}
