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

// https://leetcode.com/problems/middle-of-the-linked-list/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;
    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    slow.clone()
}

// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: -1,
        next: head,
    });

    let mut right = dummy.clone();
    let mut left = dummy.as_mut();

    for _ in 0..n {
        right = right.next.unwrap();
    }

    while let Some(node) = right.next {
        right = node;
        left = left.next.as_mut().unwrap();
    }

    left.next = left.next.as_mut().unwrap().next.clone();

    dummy.next
}

// https://leetcode.com/problems/sort-array-by-parity-ii
pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(a.len());
    let mut odds = a.clone();
    odds.retain(|&x| x % 2 != 0);
    let mut evens = a.clone();
    evens.retain(|&x| x % 2 == 0);
    for i in 0..a.len() {
        if i % 2 != 0 {
            res.push(odds.pop().unwrap());
        } else {
            res.push(evens.pop().unwrap());
        }
    }
    res
}

pub fn sort_array_by_parity_ii_functional(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .filter(|&num| num % 2 == 0)
        .zip(nums.iter().filter(|&num| num % 2 != 0))
        .flat_map(|(&even, &odd)| vec![even, odd].into_iter())
        .collect()
}

// https://leetcode.com/problems/contains-duplicate/
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut v = nums.clone();
    v.sort_unstable();
    v.dedup();
    v.len() != nums.len()
}

// https://leetcode.com/problems/single-number/
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |acc, &x| acc ^ x)
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

#[test]
fn test_middle_node() {
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(4)));
    head.as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next
        .as_mut()
        .unwrap()
        .next = Some(Box::new(ListNode::new(5)));
    assert_eq!(3, middle_node(head).unwrap().val);
}

#[test]
fn test_sort_array_by_parity_ii() {
    let nums = vec![4, 2, 5, 7];
    assert_eq!(vec![2, 7, 4, 5], sort_array_by_parity_ii(nums.clone()));
    assert_eq!(vec![4, 5, 2, 7], sort_array_by_parity_ii_functional(nums));
}

#[test]
fn test_contains_duplicate() {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(true, contains_duplicate(nums.clone()));
    assert_eq!(false, contains_duplicate(vec![1, 2, 3, 4]));
}

#[test]
fn test_single_number() {
    let nums = vec![4, 1, 2, 1, 2];
    assert_eq!(4, single_number(nums));
}
