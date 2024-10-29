#[allow(dead_code)]
pub struct StringsAndVec;

impl StringsAndVec {
    // link to exercise: https://leetcode.com/problems/merge-sorted-array/?envType=study-plan-v2&envId=top-interview-150
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.truncate(m as usize);
        nums2.truncate(n as usize);
        nums1.append(nums2);
        nums1.sort();
    }

    // link to exercise: https://leetcode.com/problems/remove-element/?envType=study-plan-v2&envId=top-interview-150
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k = k + 1;
            }
        }

        return k as i32;
    }

    // link to exercise: https://leetcode.com/problems/remove-duplicates-from-sorted-array/?envType=study-plan-v2&envId=top-interview-150
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut k = 0;

        for i in 1..nums.len() {
            if nums[k] != nums[i] {
                k += 1;
                nums[k] = nums[i];
            }
        }

        return (k+1) as i32;
    }
}

pub fn run_string_and_vec() {
    // -------------- Ex 1 ---------------
    let mut nums1 = [1, 2, 3, 0, 0, 0].to_vec();
    let mut nums2 = [2, 5, 6].to_vec();
    let m = 3;
    let n = 3;
    StringsAndVec::merge(&mut nums1, m, &mut nums2, n);

    // -------------- Ex 2 ---------------
    let mut nums = [3, 2, 2, 3].to_vec();
    let val = 3;
    StringsAndVec::remove_element(&mut nums, val);

    // -------------- Ex 3 ---------------
    let mut muns = [0,0,1,1,1,2,2,3,3,4].to_vec();
    StringsAndVec::remove_duplicates(&mut muns);
}
