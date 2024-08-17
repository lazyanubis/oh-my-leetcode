import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

/**
 * @author dawn
 * @date 2020/07/16
 */
public class Test39 {

    public static void main(String[] args) {
        System.out.println(new Test39().combinationSum(new int[] {7,3,9,6},
        6));
    }

    public List<List<Integer>> combinationSum(int[] candidates, int target) {
        if (null == candidates || 0 == candidates.length) return Collections.emptyList();
        if (target < candidates[0]) return Collections.emptyList();
        Arrays.sort(candidates);
        System.out.println(candidates);
        return combinationSum(candidates, candidates.length, target);
    }

    public List<List<Integer>> combinationSum(int[] candidates, int end, int target) {
        if (0 == end) return Collections.emptyList();
        if (target < candidates[0]) return Collections.emptyList();

        List<List<Integer>> list = new ArrayList<>();
        for (int i = end - 1; 0 <= i; i--) {
            if (candidates[i] < target) {
                List<List<Integer>> l = combinationSum(candidates, i+1, target - candidates[i]);
                for (List<Integer> ll : l) {
                    ll.add(candidates[i]);
                    list.add(ll);
                }
            } else if (candidates[i] == target) {
                List<Integer> ll = new ArrayList<>();
                ll.add(candidates[i]);
                list.add(ll);
            }
        }

        return list;
    }
}
