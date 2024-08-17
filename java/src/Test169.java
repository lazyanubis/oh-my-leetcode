import java.util.HashMap;

public class Test169 {

    public static void main(String[] args) {
        int r = new Test169().majorityElement(new int[]{1, 2, 3, 4, 4, 5, 4, 4, 5, 4, 4, 3, 4});
        System.out.println(r);
    }

    public int majorityElement(int[] nums) {
        HashMap<Integer, Integer> map = new HashMap<>();

        int num = nums[0];
        int count = 1;
        map.put(num, count);

        int i = 1;
        while (i < nums.length) {
            int n = nums[i];

            if (map.containsKey(n)) {
                int v = map.get(n) + 1;
                map.put(n, v);

                if (n == num) {
                    count = v;
                } else {
                    if (count < v) {
                        num = n;
                        count = v;
                    }
                }
            } else {
                map.put(n, 1);
            }

            i += 1;
        }

        return num;
    }

//    public int majorityElement(int[] nums) {
//        int left = 1;
//        int right = 0;
//
//        int num = nums[0];
//        int count = 1;
//
//        int i = left;
//        while (i < nums.length - right) {
//            int n = nums[i];
//
//            int p = -1;
//            int j = 0;
//            while (j < left) {
//                if (nums[j] == n) {
//                    p = j;
//                    break;
//                }
//                j += 1;
//            }
//
//            if (p == -1) {
//                nums[left] = n;
//                left += 1;
//            } else {
//                if (p < right) {
//                    nums[nums.length - p - 1] += 1;
//                    int v = nums[nums.length - p - 1];
//                    if (n == num) {
//                        count = v;
//                    } else {
//                        if (count < v) {
//                            num = n;
//                            count = v;
//                        }
//                    }
//                } else {
//                    nums[i] = nums[nums.length - right - 1];
//                    nums[nums.length - right - 1] = 2;
//                    nums[p] = nums[right];
//                    nums[right] = n;
//                    right += 1;
//                    i -= 1;
//
//                    if (n == num) {
//                        count = 2;
//                    } else {
//                        if (count < 2) {
//                            num = n;
//                            count = 2;
//                        }
//                    }
//                }
//            }
//
//            i += 1;
//        }
//
//        return num;
//    }
}