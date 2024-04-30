// Vector - 1
#[must_use]
pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let ans_1 = nums.as_slice();
    let ans_2 = nums.as_slice();

    [ans_1, ans_2].concat()
}

// Vector - 2
#[must_use]
pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .filter_map(|&x| nums.get(x as usize).cloned())
        .collect()
}

// Vector - 3
#[must_use]
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len());
    let mut sum = 0;
    nums.iter()
        .map(|&x| {
            sum += x;
            ans.push(sum);
        })
        .last();

    ans
}

// Vector - 4
#[must_use]
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let sum_arr: Vec<i32> = accounts
        .iter()
        .map(|account| account.iter().sum())
        .collect();

    *sum_arr.iter().max().unwrap()
}

// Vector - 5
pub fn shuffle(nums: Vec<i32>, _n: i32) -> Vec<i32> {
    let len_half = nums.len() / 2;
    let first = &nums[..len_half];
    let second = &nums[len_half..nums.len()];

    first
        .iter()
        .zip(second.iter())
        .flat_map(|(&x, &y)| vec![x, y])
        .collect()
}
