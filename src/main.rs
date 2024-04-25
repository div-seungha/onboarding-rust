use rust_test::number::kids_with_candies;
use rust_test::number::minimum_sum;
use rust_test::string::count_matches;
use rust_test::string::defang_i_paddr;
use rust_test::string::most_words_found;
use rust_test::string::num_jewels_in_stones;
use rust_test::string::sort_sentence;
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

    fn print_string_answer() {
        println!("*****String****************************");
        println!(
            "Answer 1: {:?}",
            defang_i_paddr("101.204.305.506".to_string())
        );
        println!(
            "Answer 2: {:?}",
            num_jewels_in_stones("aBcEX".to_string(), "aaaCCBBBX".to_string())
        );
        println!(
            "Answer 3: {:?}",
            most_words_found(vec![
                "alice and bob love leetcode".to_string(),
                "i think so too".to_string(),
                "this is great thanks very much".to_string()
            ])
        );
        println!(
            "Answer 4: {:?}",
            sort_sentence("sentence4 a3 is2 This1".to_string())
        );
        println!(
            "Answer 5: {:?}",
            count_matches(
                vec![
                    vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                    vec![
                        "computer".to_string(),
                        "silver".to_string(),
                        "lenovo".to_string()
                    ],
                    vec![
                        "phone".to_string(),
                        "gold".to_string(),
                        "iphone".to_string()
                    ]
                ],
                "color".to_string(),
                "silver".to_string()
            )
        );
    }

    print_vector_answer();
    print_number_answer();
    print_string_answer();
}
