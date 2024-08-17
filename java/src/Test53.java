import java.util.logging.Level;

/**
 * @author dawn
 * @date 2020/07/31
 */
public class Test53 {

    public static void main(String[] args) {
        System.out.println(new Test53().maxSubArray(new int[] { -2,1,-3,4,-1,2,1,-5,4 }));
    }
    public int maxSubArray(int[] nums) {
        if (null == nums) return -1;
        if (0 == nums.length) return 0;
        if (1 == nums.length) return nums[0];

        int start = 0;
        int max = nums[0];
        while (start < nums.length && nums[start] <= 0) {
            if (max < nums[start]) max = nums[start];
            start++;
        }
        if (start == nums.length) return max;

        return solve(nums, start + 1, nums[start], nums[start]);
    }

    private int solve(int[] nums, int start, int value, int max) {
        if (nums.length <= start) return max;

        if (0 <= nums[start]) return solve(nums, start + 1, value + nums[start], Math.max(Math.max(max, value + nums[start]), nums[start]));

        int temp = nums[start++];

        for (;start < nums.length; start++) {
            if (0 < nums[start]) break;
            temp += nums[start];
        }

        if (value < -temp) return solve(nums, start, 0, max < value ? value : max);
        return solve(nums, start, value + temp, max < value ? value : max);
    }

}
