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
        .map(|&x| *nums.get(x as usize).unwrap_or(&0))
        .collect()
}

// Vector - 3
#[must_use]
pub fn running_sum(nums: &[i32]) -> Vec<i32> {
    let size = nums.len();
    let mut ans: Vec<i32> = vec![];
    ans.push(nums[0]);
    for i in 1..size {
        ans.push(ans[i - 1] + nums[i]);
    }
    ans
}

// Vector - 4
#[must_use]
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut sum_arr: Vec<i32> = vec![];
    for item in accounts {
        sum_arr.push(item.iter().sum())
    }
    *sum_arr.iter().max().unwrap()
}

// Vector - 5
pub fn shuffle(nums: Vec<i32>, _n: i32) -> Vec<i32> {
    let len: usize = nums.len();
    let len_half: usize = nums.len() / 2;
    let mut ans = vec![];
    let first = &nums[..len_half];
    let second = &nums[len_half..len];
    for i in 0..len_half {
        let mut inner_arr = vec![];
        if i < len_half {
            inner_arr.push(first[i]);
            inner_arr.push(second[i]);
            ans.push(inner_arr);
        } else {
            inner_arr.push(first[i - 1]);
            inner_arr.push(second[i - 1]);
            ans.push(inner_arr);
        }
    }

    ans.into_iter().flatten().collect()
}
