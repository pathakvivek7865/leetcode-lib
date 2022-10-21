use std::ops::Index;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        println!("{}", s.len());
        let mut len: i32 = 0;
        let mut substring = String::new();

        for val in s.chars() {
            if substring.contains(val) {
                let mut new_string = String::new();
                let mut found_repeated_char = false;
                for sub_char in substring.chars() {
                    if found_repeated_char {
                        new_string.push(sub_char);
                    }
                    if sub_char == val {
                        found_repeated_char = true;
                    }
                }
                substring = new_string;
            }

            substring.push(val);
            if substring.len() as i32 > len {
                len = substring.len() as i32;
            }
        }

        len
    }
}

fn main() {
    let count = Solution::length_of_longest_substring(String::from("dvdf"));
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_owned()),
            3
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::length_of_longest_substring(" ".to_owned()), 1);
    }
}
