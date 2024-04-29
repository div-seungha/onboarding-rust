use std::collections::HashMap;

// Number - 1
pub fn minimum_sum(num: i32) -> i32 {
    let unit = vec![1000, 100, 10, 1];
    let mut digit_num_arr = vec![];
    let mut digit = num;

    for item in unit {
        digit_num_arr.push(digit / item);
        digit = digit % item;
    }

    digit_num_arr.sort();

    let min = digit_num_arr[0] * 10 + digit_num_arr[3];
    let min_2 = digit_num_arr[1] * 10 + digit_num_arr[2];

    min + min_2
}

// Number - 2

// Number - 3
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut ans = vec![];

    for item in &candies {
        let max_candies: i32 = item + extra_candies;
        if max_candies >= candies.clone().into_iter().max().unwrap() {
            ans.push(true)
        } else {
            ans.push(false)
        }
    }

    ans
}

// Number - 4
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut num = n;
    let mut a = 1;
    let mut b = 0;

    while num != 0 {
        let c = num % 10;
        num = num / 10;
        a = a * c;
        b = b + c;
    }
    a - b
}

// Number - 5
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];

    for a in 0..nums.len() {
        let mut b = 0;

        for c in 0..nums.len() {
            if a != c && nums[a] > nums[c] {
                b += 1
            }
        }

        ans.push(b)
    }

    ans
}
