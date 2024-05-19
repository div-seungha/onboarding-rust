// iter - 1
#[must_use]
pub fn final_value_after_operations(operations: Vec<&str>) -> i32 {
    let mut ans = 0;

    for operation in operations {
        match operation {
            "++X" | "X++" => ans += 1,
            "--X" | "X--" => ans -= 1,
            _ => (),
        }
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
pub fn create_target_array(nums: &[i32], index: &[usize]) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len());

    for (&num, &idx) in nums.iter().zip(index.iter()) {
        ans.insert(idx, num);
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
    let (mut seats_m, mut students_m) = (seats, students);

    seats_m.sort_unstable();
    students_m.sort_unstable();

    seats_m
        .iter()
        .zip(students_m.iter())
        .map(|(&seat, &student)| (seat - student).abs())
        .sum()
}
