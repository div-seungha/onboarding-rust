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
            "{} This function cannot control this parameter:",
            num
        ))
    }
}

// Number - 2 -- 현재 풀이 중 ---------------------------------------------
// pub fn num_identical_pairs(nums: Vec<u32>) -> u32 {
//     use std::collections::HashMap;

//     for n in nums {
//         let t = (HashMap::new(), 0);
//         let v = 0;
//     }

//     nums.iter()
//         .fold((HashMap::new(), 0), |(mut a, mut b), &num| {
//             if let Some(&c) = a.get(&num) {
//                 b += c;
//             }

//             a.entry(num).and_modify(|v| *v += 1);

//             (a, b)
//         })
//         .1
// }

// Number - 2 - for
// pub fn num_identical_pairs_with_for(nums: Vec<u32>) -> u32 {
//     let mut map = HashMap::new();
//     let mut t = (map, 0);

//     for i in 0..nums.len() - 1 {
//         if nums[i] == map.get(i) {
//             t.entry(nums[i]).and_modify(|counter| *counter += 1);
//         }
//     }

//     10
// }
// -------------------------------------------------------------------

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
            "{:?} There is none of max value in candies. Something is wrong.",
            candies
        ))
    }
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
