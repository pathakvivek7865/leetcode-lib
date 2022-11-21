use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

        let nums = nums1.iter().zip(nums2.iter());
        let mut merged_arrays = vec![];
        for (num1, num2) in nums {
            if num1 == num2 {
                merged_arrays.push(num1);
            } else if num1 < num2 {
                merged_arrays.push(num1);
                merged_arrays.push(num2);
            } else {
                merged_arrays.push(num2);
                merged_arrays.push(num1);
            }
        }

        println!("{:?}", merged_arrays);

        let n = merged_arrays.len();

        if n % 2 == 0 {
            println!("even length");
            ((merged_arrays[(n / 2) - 1] + merged_arrays[((n / 2) + 1) - 1]) / 2) as f64
        } else {
            println!("odd length");

            (*merged_arrays[((n + 1) / 2) - 1]) as f64
        }
    }
}

fn main() {
    let md = Solution::find_median_sorted_arrays(vec![1, 3], vec![1, 2, 3]);

    println!("{}", md);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![1, 2, 3]),
            2.0
        );
    }

    #[test]
    fn ex2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.0
        );
    }
}
