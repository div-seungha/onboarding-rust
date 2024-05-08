// iter - 1
#[must_use]
pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.iter().fold(0, |acc, s| {
        if s.starts_with("'+'") || s.ends_with("'+'") {
            acc + 1
        } else {
            acc - 1
        }
    })
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
    nums.iter()
        .zip(index.iter())
        .fold(Vec::with_capacity(nums.len()), |mut acc, (&num, &idx)| {
            acc.insert(idx as usize, num);
            acc
        })
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

    seats_m.sort_by_key(|&a| a);
    students_m.sort_by_key(|&b| b);

    seats_m
        .iter()
        .zip(students_m.iter())
        .map(|(&seat, &student)| (seat - student).abs())
        .sum()
}
