mod add_two_nums;
mod di_string_match;
mod house_robber;
mod is_alien_sorted;
mod jump_game_ii;
mod l_668;
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

    println!("{:?}", Solution::find_kth_number(3, 3, 5))
}

fn s(s: &str) -> String {
    s.to_string()
}