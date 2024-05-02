// iter - 1
#[must_use]
pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut ans = 0;
    for s in operations.iter() {
        if s.starts_with("'+'") || s.ends_with("'+'") {
            ans += 1;
        } else {
            ans -= 1;
        };
    }
    ans
}

// iter - 2
#[must_use]
pub fn number_of_steps(num: i32) -> i32 {
    let mut ans = 0;
    let mut num_m = num;
    while num_m != 0 {
        if num_m % 2 == 0 {
            num_m /= 2;
        } else {
            num_m -= 1;
        }
        ans += 1;
    }
    ans
}

// iter - 3
#[must_use]
pub fn create_target_array(nums: &[i32], index: &[i32]) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::with_capacity(nums.len());
    for i in 0..nums.len() {
        ans.insert(index[i].try_into().unwrap(), nums[i]);
    }

    ans
}

// iter - 4
#[must_use]
pub fn number_of_matches(n: i32) -> i32 {
    n - 1
}

// iter - 5
#[must_use]
pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut seats_m = seats;
    let mut students_m = students;

    seats_m.sort_unstable();
    students_m.sort_unstable();

    let n = seats_m.len();

    let mut ans = 0;

    for i in 0..n {
        ans += (seats_m[i] - students_m[i]).abs();
    }
    ans
}
