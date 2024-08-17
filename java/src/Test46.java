import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

/**
 * @author dawn
 * @date 2020/07/23
 */
public class Test46 {

    public static void main(String[] args) {
        System.out.println(new Test46().permute(new int[] { 1, 2, 3}));
    }


    public List<List<Integer>> permute(int[] nums) {
        if (null == nums) return null;
        if (0 == nums.length) return Collections.emptyList();
        if (1 == nums.length) return Collections.singletonList(Collections.singletonList(nums[0]));

        int length = nums.length;
        List<List<Integer>> list = new ArrayList<>();
        for (int i = 0; i < length; i++) {
            List<Integer> t = new ArrayList<>(length);
            t.add(nums[i]);
            list.add(t);
        }
        int length2;
        while (list.get(0).size() < length) {
            length2 = list.get(0).size();
            List<List<Integer>> list2 = new ArrayList<>();
            for (List<Integer> l : list) {
                boolean first = true;
                for (Integer i : nums) {
                    if (!l.contains(i)) {
                        if (first) {
                            l.add(i);
                            list2.add(l);
                            first = false;
                        } else {
                            List<Integer> temp = new ArrayList<>(length);
                            temp.addAll(l.subList(0,length2));
                            temp.add(i);
                            list2.add(temp);
                        }
                    }
                }
            }
            list = list2;
        }

        return list;
    }

}
