fn main() {
    // Vector - 1
    fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let nums_len: usize = nums.len();
        let ans_len: usize = nums_len * 2;
        for i in 0..ans_len {
            if i < nums_len {
                ans.push(nums[i])
            } else if i >= nums_len && i < 2 * nums_len {
                ans.push(nums[i - nums_len])
            }
        };
        ans
    }

    // Vector - 2
    fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let nums_len = nums.len();

        for i in 0..nums_len {
            let val: usize;
            val = nums[i] as usize;
            ans.push(nums[val])
        }

        ans
    }

    // Vector - 3
    fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len();
        let mut ans: Vec<i32> = vec![];
        ans.push(nums[0]);
        for i in 1..size {
           ans.push(ans[i - 1] + nums[i])
        }

        ans
    }

    // Vector - 4
    fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut sum_arr: Vec<i32> = vec![];
         for item in accounts {
            sum_arr.push(item.iter().sum())
         }

         *sum_arr.iter().max().unwrap()
     }

     // Vector - 5
    fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {

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
                inner_arr.push(first[i-1]);
                inner_arr.push(second[i-1]);
                ans.push(inner_arr)
            }
        }

        ans.into_iter().flatten().collect()
    }
}
