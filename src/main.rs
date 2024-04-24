use rust_test::number::kids_with_candies;
use rust_test::number::minimum_sum;
use rust_test::vector::build_array;
use rust_test::vector::get_concatenation;
use rust_test::vector::maximum_wealth;
use rust_test::vector::running_sum;
use rust_test::vector::shuffle;

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

    print_vector_answer();
    print_number_answer();
}
