import java.util.*;

/**
 * @author dawn
 * @date 2020/07/31
 */
public class Test131 {

    public static void main(String[] args) {
        List<List<String>> r = new Test131().partition("aab");
//        List<List<String>> r = new Test131().partition("bb");
//        List<List<String>> r = new Test131().partition("cdd");
        System.out.println(r);
    }

    private int length;

    private Map<Integer, List<List<String>>> map = new HashMap<>();

    public List<List<String>> partition(String s) {
        if (null == s || s.isEmpty()) return Collections.emptyList();

        length = s.length();

        if (1 == length) return Collections.singletonList(Collections.singletonList(s));

        return partition(s, 0);
    }

    private List<List<String>> partition(String s, int start) {
        if (start >= length) return Collections.emptyList();

        if (map.containsKey(start)) return map.get(start);
        List<List<String>> result = null;
        if (start == length - 1) result = Collections.singletonList(Collections.singletonList(s.substring(start, length)));
        else {
            result = new ArrayList<>();
            for (int i = start + 1; i < length; i++) {
                if (check(s, start, i - 1)) {
                    List<List<String>> right = partition(s, i);
                    if (!right.isEmpty()) {
                        for (List<String> r : right) {
                            List<String> rr = new LinkedList<>();
                            rr.add(s.substring(start, i));
                            rr.addAll(r);
                            add(result, rr);
                        }
                    }
                }
            }
            if (check(s, start, length - 1)) {
                add(result, Collections.singletonList(s.substring(start, length)));
            }
        }


        map.put(start, result);

        return result;
    }

    private boolean check(String s, int start, int end) {
        while (start < end) {
            if (s.charAt(start) != s.charAt(end)) {
                return false;
            }
            start++;
            end--;
        }
        return true;
    }

    private void add(List<List<String>> result, List<String> list) {
        for (List<String> l : result) {
            if (l.size() == list.size()) {
                boolean flag = true;
                for (int i = 0; i < l.size(); i++) {
                    if (!l.get(i).equals(list.get(i))) {
                        flag = false;
                        break;
                    }
                }
                if (flag) {
                    return;
                }
            }
        }
        result.add(list);
    }

}
