// Number - 1
#[must_use]
pub fn minimum_sum(num: i32) -> i32 {
    let mut digits = [num % 10, num / 10 % 10, num / 100 % 10, num / 1000 % 10];
    digits.sort_by_key(|&x| x);

    (digits[0] * 10 + digits[3]) + (digits[1] * 10 + digits[2])
}

// Number - 2

// Number - 3
#[must_use]
pub fn kids_with_candies(candies: &[i32], extra_candies: i32) -> Vec<bool> {
    let mut ans = vec![];

    for item in candies {
        let max_candies: i32 = item + extra_candies;
        if max_candies >= candies.iter().copied().max().unwrap() {
            ans.push(true);
        } else {
            ans.push(false);
        }
    }

    ans
}

// Number - 4
#[must_use]
pub fn subtract_product_and_sum(n: i32) -> i32 {
    let mut num = n;
    let mut a = 1;
    let mut b = 0;

    while num != 0 {
        let c = num % 10;
        num /= 10;
        a *= c;
        b += c;
    }
    a - b
}

// Number - 5
#[must_use]
pub fn smaller_numbers_than_current(nums: &[i32]) -> Vec<i32> {
    let mut ans = vec![];

    for a in 0..nums.len() {
        let mut b = 0;

        for c in 0..nums.len() {
            if a != c && nums[a] > nums[c] {
                b += 1;
            }
        }

        ans.push(b);
    }

    ans
}
