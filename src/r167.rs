impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left: i32 = 0;
        let mut right: i32 = numbers.len().try_into().unwrap();

        while left < right {
            let reminder = target - numbers[left];
            while left < right && reminder < numbers[right] {
                right = right - 1;
            }
            if left >= right {
                break;
            }
            if (reminder == numbers[right]) {
                break;
            }
            left = left + 1;
        }

        return vec![left, right];
    }
}