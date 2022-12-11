struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let length_of_x = x.to_string().len();
        let stringed_x = x.to_string();
        for (index, c) in x.to_string().chars().enumerate() {
            if c != stringed_x.chars().nth(length_of_x - index - 1).unwrap() {
                return false;
            }
        }
        true
    }
}
fn main() {
    let x = 121;
    let result = Solution::is_palindrome(x);
    println!("{}", result);
}

#[test]
fn test_is_palindrome() {
    let mut x = 121;
    let result = Solution::is_palindrome(x);
    assert_eq!(result, true);
}
#[test]
fn test_is_palindrome2() {
    let x = -121;
    let result = Solution::is_palindrome(x);
    assert_eq!(result, false);
}
