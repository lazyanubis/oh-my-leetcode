import java.util.*;

/**
 * @author dawn
 * @date 2020/07/23
 */
public class Test45 {

    public static void main(String[] args) {
        System.out.println(new Test45().jump(new int[] { 10,9,8,7,6,5,4,3,2,1,1,0}));
    }

    private int max;

    public int jump(int[] nums) {
        if (null == nums || 0 == nums.length) return -1;
        if (1 == nums.length) return 0;

        int[] index = new int[nums.length];

        for (int i = 0; i < nums.length - 1; i++) {
            for (int j = 1; j <= nums[i]; j++) {
                if (i + j < nums.length && ((index[i] + 1) < index[i + j] || 0 == index[i + j])) {
                    index[i + j] = index[i] + 1;
                }
            }
        }

        return index[nums.length - 1];

//        for (int i : nums) {
//            if (max < i) max = i;
//        }
//
//        Map<Integer, List<Integer>> map = new HashMap<>();
//        map.put(nums.length - 1, new ArrayList<>());
//        for (int i = nums.length - 1; 0 <= i; i--) {
//            if (map.containsKey(i)) {
//                map.get(i).addAll(find(nums, i));
//                for (Integer index : map.get(i)) {
//                    if (!map.containsKey(index)) {
//                        map.put(index, new ArrayList<>());
//                    }
//                }
//            }
//        }
//
//        int count = 0;
//        Set<Integer> records = new HashSet<>();
//        records.add(nums.length - 1);
//        Set<Integer> temp = new HashSet<>();
//        mark:
//        while(true) {
//            count++;
//            temp = new HashSet<>();
//            for (Integer i : records) {
//                temp.addAll(map.get(i));
//            }
//            for (Integer i : temp) {
//                if (i == 0) {
//                    break mark;
//                }
//            }
//            records = temp;
//        }
//
//        return count;
    }

//    private List<Integer> find(int[] nums, int index) {
//        List<Integer> list = new ArrayList<>();
//        for (int i = 1; i <= max; i++) {
//            if (index - i < 0) break;
//            if (nums[index - i] >= i) {
//                list.add(index - i);
//            }
//        }
//        return list;
//    }
}
