// Number - 1
#[must_use]
pub fn minimum_sum(num: i32) -> i32 {
    fn is_four_digit(num: i32) -> Result<i32, String> {
        if num >= 1000 && num <= 9999 {
            Ok(num)
        } else {
            Err(format!(
                "{} This function cannot control this parameter:",
                num
            ))
        }
    }

    let mut digit = match is_four_digit(num) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            0
        }
    };

    let unit = vec![1000, 100, 10, 1];
    let mut digits = vec![];

    for item in unit {
        digits.push(digit / item);
        digit = digit % item;
    }

    (digits[0] * 10 + digits[3]) + (digits[1] * 10 + digits[2])
}

// Number - 2
pub fn num_identical_pairs(nums: Vec<u32>) -> u32 {
    use std::collections::HashMap;

    nums.iter()
        .fold((HashMap::new(), 0), |(mut a, mut b), &num| {
            if let Some(&c) = a.get(&num) {
                b += c;
            }

            a.entry(num).and_modify(|v: &mut u32| *v += 1);

            (a, b)
        })
        .1
}

// Number - 2 - for
pub fn num_identical_pairs_with_for(nums: Vec<u32>) -> u32 {
    let mut count = 0;

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                count += 1;
            }
        }
    }
    count
}

// Number - 3
#[must_use]
pub fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Vec<bool> {
    let max_candies = candies.iter().max();

    let c = match max_candies {
        Some(v) => v,
        None => {
            format!("Something is wrong.");
            &0
        }
    };

    candies
        .iter()
        .map(|&candy| candy + extra_candies >= *c)
        .collect()
}

// Number - 4
#[must_use]
pub fn subtract_product_and_sum(n: i32) -> u32 {
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .filter_map(|c| Some(c.to_digit(10).expect("Returns None, Something is wrong.")))
        .collect();

    let product: u32 = digits.iter().product();
    let sum: u32 = digits.iter().sum();

    product - sum
}

// Number - 5
#[must_use]
pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<usize> {
    nums.iter()
        .map(|i| nums.iter().filter(|j| *j < i).count())
        .collect()
}
