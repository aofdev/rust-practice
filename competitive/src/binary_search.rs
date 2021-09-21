use std::cmp::Ordering;

fn main() {}

// Binary Search https://leetcode.com/problems/binary-search/
pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut low, mut high) = (0, nums.len());

    while low < high {
        dbg!(low, high);
        let mid = dbg!(low + (high - low) / 2);
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

#[test]
fn test_binary_search() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    assert_eq!(4, binary_search(nums.clone(), 9));
    assert_eq!(-1, binary_search(nums.clone(), 2));
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
