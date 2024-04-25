use rust_test::iter::*;
use rust_test::number::*;
use rust_test::vector::*;

fn main() {
    fn print_vector_answer() {
        println!("*****Vector****************************");
        println!("Answer 1: {:?}", get_concatenation(vec![1, 2, 3, 4, 5]));
        println!("Answer 2: {:?}", build_array(vec![1, 2, 3, 4, 5]));
        println!("Answer 3: {:?}", running_sum(vec![1, 2, 3, 4, 5]));
        println!(
            "Answer 4: {:?}",
            maximum_wealth(vec![vec![1, 2], vec![3, 4,], vec![5, 6]])
        );
        println!("Answer 5: {:?}", shuffle(vec![1, 2, 3, 4, 5], 3));
    }

    fn print_number_answer() {
        println!("*****Number****************************");
        println!("Answer 1: {:?}", minimum_sum(3469));
        println!("Answer 2: 아직 정답을 찾지 못함");
        println!("Answer 3: {:?}", kids_with_candies(vec![2, 4, 3, 6, 8], 5));
        println!("Answer 4: 아직 정답을 찾지 못함");
        println!("Answer 5: 아직 정답을 찾지 못함");
    }

    fn print_iterations_answer() {
        println!("*****Iterations****************************");
        println!(
            "Answer 1: {:?}",
            final_value_after_operations(vec![
                "--X".to_string(),
                "X++".to_string(),
                "X++".to_string()
            ])
        );
        println!("Answer 2: {:?}", number_of_steps(14));
        println!(
            "Answer 3: {:?}",
            create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1])
        );
        println!("Answer 4: {:?}", number_of_matches(7));
        println!(
            "Answer 5: {:?}",
            min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4])
        );
    }

    print_vector_answer();
    print_number_answer();

    print_iterations_answer();
}
