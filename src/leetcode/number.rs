// Number - 1
#[must_use]
pub fn minimum_sum(num: i32) -> i32 {
    let mut digits = [num % 10, num / 10 % 10, num / 100 % 10, num / 1000 % 10];
    digits.sort_by_key(|&x| x);

    (digits[0] * 10 + digits[3]) + (digits[1] * 10 + digits[2])
}

// Number - 2
pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    nums.iter()
        .fold((HashMap::new(), 0), |(mut a, mut b), &num| {
            if let Some(&c) = a.get(&num) {
                b += c;
            }

            *a.entry(num).or_insert(0) += 1;

            (a, b)
        })
        .1
}

// Number - 3
#[must_use]
pub fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Vec<bool> {
    let max_candies = candies.iter().max();

    let c = match max_candies {
        Some(v) => v,
        None => &0,
    };

    candies
        .iter()
        .map(|&candy| candy + extra_candies >= *c) // Check if each child can have the most candies
        .collect()
}

// Number - 4
#[must_use]
pub fn subtract_product_and_sum(n: i32) -> u32 {
    let v = n.to_string();
    let digits = v.chars().filter_map(|c| c.to_digit(10)).map(|d| d);

    let product: &u32 = &digits.clone().product();
    let sum: &u32 = &digits.sum();

    product - sum
}

// Number - 5
#[must_use]
pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<usize> {
    nums.iter()
        .map(|i| nums.iter().filter(|j| j < &i).count())
        .collect()
}
