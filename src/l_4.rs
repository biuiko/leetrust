use crate::Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        if len % 2 == 1 {
            Self::get_kth_num(nums1, nums2, (len + 1) / 2) as f64
        } else {
            let a = Self::get_kth_num(nums1.clone(), nums2.clone(), len / 2) as f64;
            let b = Self::get_kth_num(nums1, nums2, len / 2 + 1) as f64;
            (a + b) / 2.0
        }
    }

    fn get_kth_num(nums1: Vec<i32>, nums2: Vec<i32>, mut k: usize) -> i32 {
        let nums1_len = nums1.len();
        let nums2_len = nums2.len();
        let mut index1 = 0;
        let mut index2 = 0;
        loop {
            if index1 == nums1_len {
                return nums2[index2 + k - 1];
            }
            if index2 == nums2_len {
                return nums1[index1 + k - 1];
            }
            if k == 1 {
                return nums1[index1].min(nums2[index2]);
            }
            let new_index1 = (index1 + k / 2 - 1).min(nums1_len - 1);
            let new_index2 = (index2 + k / 2 - 1).min(nums2_len - 1);
            if nums1[new_index1] <= nums2[new_index2] {
                k -= new_index1 - index1 + 1;
                index1 = new_index1 + 1;
            } else {
                k -= new_index2 - index2 + 1;
                index2 = new_index2 + 1;
            }
        }
    }
}
