mod solution;

fn main() {
    let s = "barfoothefoobarman".to_string();
    let words = vec!["foo".to_string(), "bar".to_string()];
    let result = solution::Solution::find_substring(s, words);
    println!("result = {:?}", result);
}
