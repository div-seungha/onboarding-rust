use std::collections::HashMap;

// Number - 1
#[must_use]
pub fn minimum_sum(num: i32) -> Result<i32, String> {
    if num >= 1000 && num <= 9999 {
        let mut digit = num;
        let unit = vec![1000, 100, 10, 1];
        let mut digits = vec![];
        for item in unit {
            digits.push(digit / item);
            digit = digit % item;
        }
        Ok((digits[0] * 10 + digits[3]) + (digits[1] * 10 + digits[2]))
    } else {
        Err(format!(
            "{} The input of this function must be 4 decimal digits. So this function cannot control this input:",
            num
        ))
    }
}

// Number - 2 - for
pub fn num_identical_pairs_with_for(nums: Vec<u32>) -> u32 {
    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut count = 0;

    for &num in &nums {
        count += *map.entry(num).and_modify(|e| *e += 1).or_insert(0)
    }

    count
}

// Number - 3
#[must_use]
pub fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Result<Vec<bool>, String> {
    if let Some(v) = candies.iter().max() {
        Ok(candies
            .iter()
            .map(|&candy| candy + extra_candies >= *v)
            .collect())
    } else {
        Err(format!(
            "{:?} To solve this problem, the first input must not be empty and at least have one maximum value. There is none of one maximum value in candies.",
            candies
        ))
    }
}

// Number - 4
#[must_use]
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let digits: Vec<i32> = n
        .to_string()
        .chars()
        .filter_map(|c|
            Some(c.to_digit(10).and_then(|v| i32::try_from(v).ok()).expect("Failed to convert character to digit. You may have to check the input of this function.")))
        .collect::<Vec<i32>>();

    let product: i32 = digits.iter().product();
    let sum: i32 = digits.iter().sum();

    product - sum
}

// Number - 5
#[must_use]
pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<usize> {
    nums.iter()
        .map(|i| nums.iter().filter(|j| *j < i).count())
        .collect()
}
