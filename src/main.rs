mod add_two_nums;
mod di_string_match;
mod house_robber;
mod is_alien_sorted;
mod jump_game_ii;
mod l_11;
mod l_128;
mod l_15;
mod l_4;
mod l_436;
mod l_462;
mod l_5;
mod l_668;
mod l_961;
mod length_of_longest_substring;
mod number_of_recent_calls;
mod one_away_lcci;
mod record_data_in_log_files;
mod two_sum;
pub struct Solution;
fn main() {
    // println!("{:?}", two_sum::two_sum(vec![3,3], 6));
    // let logs = vec!["dig1 8 1 5 1".to_string(),"let1 art can".to_string(),"dig2 3 6".to_string(),"let2 own kit dig".to_string(),"let3 art zero".to_string()];
    // println!("{:?}", record_data_in_log_files::reorder_log_files(logs));
    // println!("{:?}",length_of_longest_substring::length_of_longest_substring("".to_string()));
    // let mut counter = number_of_recent_calls::RecentCounter::new();
    // println!("{:?}",counter.ping(1));
    // println!("{:?}",counter.ping(100));
    // println!("{:?}",counter.ping(3001));
    // println!("{:?}",counter.ping(3002));
    // println!("{:?}", Solution::di_string_match("IIIIIIIIIIIDDDIIIIIIIIIIII".to_string()));
    // print!("{:?}", Solution::jump(vec![2, 3, 0, 1, 4]));
    // let first = String::from("abc");
    // let second = String::from("abdd");
    // println!("{:?}", Solution::one_edit_away(first,second));

    // println!(
    //     "{:?}",
    //     Solution::is_alien_sorted(vec![s("apple"), s("app")], s("abcdefghijklmnopqrstuvwxyz"))
    // )

    // println!("{:?}", Solution::find_kth_number(3, 3, 5))
    // println!(
    //     "{:?}",
    //     Solution::three_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4])
    // );
    // println!("{:?}", Solution::min_moves2(vec![1, 10, 2, 9]))
    // println!(
    //     "{:?}",
    //     Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]])
    // );
    // println!(
    //     "{:?}",
    //     Solution::longest_consecutive(vec![100, 55, 200, 11, 31, 2])
    // )

    // println!("{:?}", Solution::max_area(vec![1, 1]));
    // println!(
    //     "{:?}",
    //     Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 2, 4])
    // );
    println!("{:?}", Solution::longest_palindrome(s("bb")));
}

fn s(s: &str) -> String {
    s.to_string()
}
