import java.util.LinkedList;
import java.util.List;

/**
 * @author dawn
 * @date 2020/07/22
 */
public class Test44 {

    public static void main(String[] args) {
        System.out.println(new Test44().isMatch("c",
                "*?*"));
    }

    public boolean isMatch(String s, String p) {
        if (null == s) { return false; }
        if (null == p) { return false; }
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < p.length() - 1; i++) {
            if (p.charAt(i) == '*' && p.charAt(i + 1) == '*') {
                continue;
            }
            sb.append(p.charAt(i));
        }
        if (!p.isEmpty()) { sb.append(p.charAt(p.length() - 1)); }
        return isMatch(s, 0, s.length(), sb.toString(), 0, sb.length());
    }

    private boolean isMatch(String s, int s1, int e1, String p, int s2, int e2) {
        Boolean checked;
        checked = check(s1, e1, p, s2, e2); if (null != checked) { return checked; }

        while (s2 < e2 && p.charAt(s2) != '*') {
            checked = check(s1, e1, p, s2, e2); if (null != checked) { return checked; }
            if (p.charAt(s2) == '?') {
                s2++;
                s1++;
            } else {
                if (p.charAt(s2) == s.charAt(s1)) {
                    s2++;
                    s1++;
                } else {
                    return false;
                }
            }
        }

        while (s2 < e2 && p.charAt(e2 - 1) != '*') {
            checked = check(s1, e1, p, s2, e2); if (null != checked) { return checked; }
            if (p.charAt(e2 - 1) == '?') {
                e2--;
                e1--;
            } else {
                if (p.charAt(e2 - 1) == s.charAt(e1 - 1)) {
                    e2--;
                    e1--;
                } else {
                    return false;
                }
            }
        }

        checked = check(s1, e1, p, s2, e2); if (null != checked) { return checked; }

        int start = find(p, s2, e2);
        if (start < 0) { return false; }
        int end = start;
        for (int i = start; i < e2; i++) { if (p.charAt(i) == '*') { break; } else { end = i + 1; } }

        for (Integer i : find(s, s1, e1, p, start, end)) {
            int j = i + end - start;
            if (isMatch(s, s1, i, p, s2, start) && isMatch(s, j, e1, p, end, e2)) { return true; }
        }
        return false;
    }

    private int find(String p, int s2, int e2) {
        int max = 0;
        int i = -1;
        int count = 0;
        for (int j = s2; j < e2; j++) {
            if (p.charAt(j) == '*') {
                if (max < count) {
                    i = j - count;
                    max = count;
                }
                count = 0;
            } else {
                count++;
            }
        }
        return i;
    }

    private List<Integer> find(String s, int s1, int e1, String p, int s2, int e2) {
        List<Integer> list = new LinkedList<>();
        if (e1 - s1 < e2 - s2) { return list; }
        mark:
        for (int i = 0; i < e1 - s1 - (e2 - s2) + 1; i++) {
            if (s.charAt(s1 + i) == p.charAt(s2) || '?' == p.charAt(s2)) {
                if (s2 == e2 - 1) {
                    list.add(i + s1);
                } else {
                    for (int j = 1; j < e2 - s2; j++) {
                        if (p.charAt(s2 + j) != '?' && p.charAt(s2 + j) != s.charAt(s1 + i + j)) {
                            continue mark;
                        }
                    }
                    list.add(i + s1);
                }
            }
        }
        return list;
    }

    private Boolean check(int s1, int e1, String p, int s2, int e2) {
        if (s2 == e2 - 1 && p.charAt(s2) == '*') { return true; }
        if (e1 <= s1) {
            if (e2 <= s2) { return true; }
            for (int i = s2; i < e2; i++) {
                if (p.charAt(i) != '*') { return false; }
            }
            return true;
        }

        if (e2 <= s2) {
            return false;
        }
        return null;
    }
}
