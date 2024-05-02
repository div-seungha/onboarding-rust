use std::collections::HashMap;

// encode-decode - 1
#[must_use]
pub fn decode(encoded: &[i32], first: i32) -> Vec<i32> {
    let mut first_m = first;
    let mut arr = vec![first_m];
    for i in 0..encoded.len() {
        arr.push(first_m ^ encoded[i]);
        first_m = arr[i + 1];
    }
    arr
}

// encode-decode - 2
#[must_use]
pub fn decompress_rl_elist(nums: &[i32]) -> Vec<i32> {
    let n = nums.len() / 2;
    let mut ans = vec![];
    for i in 0..n {
        for _ in 0..nums[2 * i] {
            ans.push(nums[2 * i + 1]);
        }
    }
    ans
}

// encode-decode - 3 --> 정답 아님
#[must_use]
pub fn restore_string(s: &str, indices: &[i32]) -> String {
    let mut ans = vec![""; indices.len()];
    let mut s_vec: Vec<&str> = s.split("").collect();

    s_vec.pop();
    s_vec.drain(0..1);

    for i in 0..indices.len() - 1 {
        let idx: usize = indices[i].try_into().unwrap();
        ans[idx] = s_vec[i];
    }

    ans.join("")
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
pub fn get_decimal_value(head: &Option<Box<ListNode>>) -> i32 {
    let mut ans = 0;
    let mut cur = head;
    while let Some(node) = cur {
        ans = (ans << 1) | node.val;
        cur = &node.next;
    }
    ans
}
