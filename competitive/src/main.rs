use std::cmp::Ordering;

fn main() {}

// Binary Search https://leetcode.com/problems/binary-search/
pub fn binary_search_1(nums: Vec<i32>, target: i32) -> i32 {
    let (mut low, mut high) = (0, nums.len());

    while low < high {
        let mid = low + (high - low) / 2;
        match target.cmp(&nums[mid]) {
            Ordering::Equal => return mid as i32,
            Ordering::Less => high = mid,
            Ordering::Greater => low = mid + 1,
        }
    }
    -1
}

pub fn binary_search_2(k: i32, items: &[i32]) -> Option<usize> {
    if items.is_empty() {
        return None;
    }

    let mut low: usize = 0;
    let mut high: usize = items.len() - 1;

    while low <= high {
        let middle = (high + low) / 2;
        if let Some(current) = items.get(middle) {
            if *current == k {
                return Some(middle);
            }
            if *current > k {
                if middle == 0 {
                    return None;
                }
                high = middle - 1
            }
            if *current < k {
                low = middle + 1
            }
        }
    }
    None
}

// https://leetcode.com/problems/first-bad-version/
fn is_bad_version(version: i32) -> bool {
    // bad version 5-8
    match version {
        5..=8 => true,
        _ => false,
    }
}
pub fn first_bad_version(n: i32) -> i32 {
    let mut left = 1;
    let mut right = n;

    while left != right {
        let middle = left + (right - left) / 2;
        if is_bad_version(middle) {
            right = middle;
        } else {
            left = middle + 1;
        }
    }
    left
}

// https://leetcode.com/problems/search-insert-position/
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);
    if target < nums[0] {
        return 0;
    }
    while left <= right {
        let middle = left + (right - left) / 2;
        if nums[middle] == target {
            return middle as i32;
        }
        if nums[middle] < target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }
    left as i32
}

pub fn search_insert_one_line(nums: Vec<i32>, target: i32) -> i32 {
    nums.binary_search(&target).unwrap_or_else(|x| x) as i32
}

// https://leetcode.com/problems/squares-of-a-sorted-array/
pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable_by_key(|x| x.abs());
    nums.iter_mut().for_each(|x| *x = x.pow(2));
    nums
}

// https://leetcode.com/problems/rotate-array
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = (k as usize) % nums.len();
    nums.rotate_right(k);
}

pub fn rotate_reverse(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let k = k as usize % len;
    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse();
}

// https://leetcode.com/problems/move-zeroes/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut z = Vec::new();
    nums.retain(|x| {
        if *x != 0 {
            true
        } else {
            z.push(0);
            false
        }
    });
    &nums.append(&mut z);
}

// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len() - 1;
    loop {
        match (numbers[i] + numbers[j]).cmp(&target) {
            Ordering::Equal => return vec![i as i32 + 1, j as i32 + 1],
            Ordering::Greater => j -= 1,
            Ordering::Less => i += 1,
        }
    }
}

// https://leetcode.com/problems/reverse-string
pub fn reverse_string(s: &mut Vec<char>) {
    if s.len() > 0 {
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

pub fn reverse_string_one_line(s: &mut Vec<char>) {
    s.reverse();
}

// https://leetcode.com/problems/reverse-words-in-a-string-iii/
pub fn reverse_words(s: String) -> String {
    s.split(" ")
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn test_binary_search_1() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    assert_eq!(4, binary_search_1(nums.clone(), 9));
    assert_eq!(-1, binary_search_1(nums.clone(), 2));
}

#[test]
fn test_binary_search_2() {
    let items = vec![1, 2, 3, 4, 5];
    assert_eq!(Some(0), binary_search_2(1, &items));
    assert_eq!(Some(1), binary_search_2(2, &items));
    assert_eq!(Some(2), binary_search_2(3, &items));
    assert_eq!(Some(3), binary_search_2(4, &items));
    assert_eq!(Some(4), binary_search_2(5, &items));
    assert_eq!(None, binary_search_2(0, &items));
    assert_eq!(None, binary_search_2(90, &items));
    assert_eq!(None, binary_search_2(9000000, &items));

    let items = vec![2, 4, 6, 80, 90, 120, 180, 900, 2000, 4000, 5000, 60000];
    assert_eq!(None, binary_search_2(1, &items));
    assert_eq!(None, binary_search_2(1, &[]));
}

#[test]
fn test_first_bad_version() {
    assert_eq!(5, first_bad_version(6));
    assert_eq!(5, first_bad_version(8));
    assert_eq!(2, first_bad_version(2));
    assert_eq!(4, first_bad_version(4));
}

#[test]
fn test_search_insert() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    assert_eq!(4, search_insert(nums.clone(), 9));
    assert_eq!(2, search_insert_one_line(nums.clone(), 2));
}

#[test]
fn test_sorted_squares() {
    let nums = vec![-4, -1, 0, 3, 10];
    assert_eq!(vec![0, 1, 9, 16, 100], sorted_squares(nums));
}

#[test]
fn test_rotate() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    rotate(&mut nums, k);
    assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], nums);

    let mut nums = vec![-1, -100, 3, 99];
    let k = 2;
    rotate_reverse(&mut nums, k);
    assert_eq!(vec![3, 99, -1, -100], nums);
}

#[test]
fn test_move_zeroes() {
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut nums);
    assert_eq!(vec![1, 3, 12, 0, 0], nums);
}

#[test]
fn test_two_sum() {
    let nums = vec![2, 7, 11, 15];
    assert_eq!(vec![1, 2], two_sum(nums, 9));
}

#[test]
fn test_reverse_string() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
}

#[test]
fn test_reverse_words() {
    let s = "Let's take LeetCode contest".to_string();
    assert_eq!("s'teL ekat edoCteeL tsetnoc".to_string(), reverse_words(s));
}
