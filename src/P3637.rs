pub fn is_trionic(nums: &[i32]) -> bool {
    let sublen = nums.len() - 1;
    let mut i = 0;

    if sublen <= i || nums[i + 1] <= nums[i] { return false }; i += 1;
    while i < sublen && nums[i + 1] > nums[i] { i += 1; }

    if sublen <= i || nums[i + 1] >= nums[i] { return false }; i += 1;
    while i < sublen && nums[i + 1] < nums[i] { i += 1; }

    if sublen <= i || nums[i + 1] <= nums[i] { return false }; i += 1;
    while i < sublen && nums[i + 1] > nums[i] { i += 1; }

    sublen <= i
}

#[cfg(test)]
mod tests {
    use super::is_trionic;

    #[test]
    fn test1() {
        assert_eq!(is_trionic(&[1, 3, 5, 4, 2, 6]), true)
    }

    #[test]
    fn test2() {
        assert_eq!(is_trionic(&[2, 1, 3]), false)
    }
}