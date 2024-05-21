// Vector - 1
#[must_use]
pub fn get_concatenation(nums: &[i32]) -> Vec<i32> {
    [nums, nums].concat()
}

// Vector - 2
#[must_use]
pub fn build_array(nums: &[usize]) -> Vec<usize> {
    nums.iter()
        .filter_map(|&x| nums.get(x).cloned())
        .collect::<Vec<usize>>()
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
pub fn shuffle(nums: &[i32]) -> Vec<i32> {
    (0..nums.len() / 2)
        .flat_map(|x| vec![nums[x], nums[x + nums.len() / 2]])
        .collect()
}

#[test]
fn test_get_concatenation() {
    let param = [1, 2, 3, 4, 5];
    let result = get_concatenation(&param);

    assert_eq!(result, [1, 2, 3, 4, 5, 1, 2, 3, 4, 5])
}

#[test]
fn test_suffle() {
    let param = [1, 2, 3, 4, 5].as_slice();
    let result = shuffle(param);

    assert_eq!(result, [1, 3, 2, 4])
}
