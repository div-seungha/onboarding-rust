// Vector - 1
#[must_use]
pub fn get_concatenation(nums: &[i32]) -> Vec<i32> {
    [nums, nums].concat()
}

// Vector - 2
#[must_use]
pub fn build_array(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .filter_map(|&x| nums.get(x as usize).cloned())
        .collect()
}

// Vector - 3
#[must_use]
// Vector - 3 - "scan"
pub fn running_sum(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

// Vector - 3 - "fold"
#[must_use]
pub fn running_sum_fold(nums: &[i32]) -> Vec<i32> {
    let mut sum = 0;

    nums.iter()
        .fold(Vec::with_capacity(nums.len()), |mut acc, &x| {
            sum += x;
            acc.push(sum);
            acc
        })
}

// Vector - 4
#[must_use]
pub fn maximum_wealth(accounts: &[Vec<i32>]) -> Option<i32> {
    accounts
        .iter()
        .map(|account| account.iter().sum::<i32>())
        .max()
}

// Vector - 5
pub fn shuffle(nums: &[i32], _n: i32) -> Vec<i32> {
    let len_half = nums.len() / 2;
    let first = &nums[..len_half];
    let second = &nums[len_half..nums.len()];

    (0..(nums.len() / 2))
        .flat_map(|x| vec![first[x], second[x]])
        .collect()
}
