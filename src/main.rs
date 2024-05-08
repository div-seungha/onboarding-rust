use rust_test::encode_decode::*;
use rust_test::iter::*;
use rust_test::number::*;
use rust_test::string::*;
use rust_test::vector::*;

fn main() {
    fn print_vector_answer() {
        println!("*****Vector****************************");
        println!(
            "Answer 1: {:?}",
            get_concatenation([1, 2, 3, 4, 5].as_slice())
        );
        println!("Answer 2: {:?}", build_array([1, 2, 3, 4, 5].as_slice()));
        println!("Answer 3: {:?}", running_sum([1, 2, 3, 4, 5].as_slice()));
        println!(
            "Answer 4: {:?}",
            maximum_wealth([vec![1, 2], vec![3, 4,], vec![5, 6]].as_slice())
        );
        println!("Answer 5: {:?}", shuffle([1, 2, 3, 4, 5].as_slice()));
    }

    fn print_number_answer() {
        println!("*****Number****************************");
        println!("Answer 1: {:?}", minimum_sum(3469));
        println!("Answer 2: {:?}", "");
        println!(
            "Answer 3: {:?}",
            kids_with_candies([2, 4, 3, 6, 8].as_slice(), 5)
        );
        println!("Answer 4: {:?}", subtract_product_and_sum(123));
        println!(
            "Answer 5: {:?}",
            smaller_numbers_than_current([1, 2, 3, 4, 5].as_slice())
        );
    }

    fn print_string_answer() {
        println!("*****String****************************");
        println!("Answer 1: {:?}", defang_i_paddr("101.204.305.506"));
        println!("Answer 2: {:?}", num_jewels_in_stones("aBcEX", "aaaCCBBBX"));
        println!(
            "Answer 3: {:?}",
            most_words_found(vec![
                "alice and bob love leetcode".to_string(),
                "i think so too".to_string(),
                "this is great thanks very much".to_string()
            ])
        );
        println!("Answer 4: {:?}", sort_sentence("sentence4 a3 is2 This1"));
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
                "color",
                "silver"
            )
        )
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
            create_target_array([0, 1, 2, 3, 4].as_slice(), [0, 1, 2, 2, 1].as_slice())
        );
        println!("Answer 4: {:?}", number_of_matches(7));
        println!(
            "Answer 5: {:?}",
            min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4])
        );
    }

    fn print_encode_decode_answer() {
        println!("*****Encode-Decode****************************");
        println!("Answer 1: {:?}", decode([1, 2, 3].as_slice(), 1));
        println!(
            "Answer 2: {:?}",
            decompress_rl_elist([1, 2, 3, 4].as_slice())
        );
        println!(
            "Answer 3: {:?}",
            restore_string("codeleet", [4, 5, 6, 7, 0, 2, 1, 3].as_slice())
        );
        println!(
            "Answer 4: {:?}",
            decode_message(
                "the quick brown fox jumps over the lazy dog",
                "vkbs bs t suepuv"
            )
        );
        println!(
            "Answer 5: {:?}",
            get_decimal_value(&Option::<Box<ListNode>>::None)
        );
    }

    print_vector_answer();
    print_number_answer();
    print_string_answer();
    print_iterations_answer();
    print_encode_decode_answer();
}
