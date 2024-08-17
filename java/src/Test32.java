/**
 * @author dawn
 * @date 2020/07/10
 */
public class Test32 {

    public static void main(String[] args) {
        System.out.println(new Test32().longestValidParentheses(")()())()()("));
    }

    public int longestValidParentheses(String s) {
        if (null == s) return -1;
        if (s.isEmpty()) return 0;
        return countLongestValidParentheses(s, 0, s.length() - 1);
    }

    private int countLongestValidParentheses(String s, int start, int end) {
        if (end < start) return 0;
        if (start == end) return 0;

        if (s.charAt(start) == ')') return countLongestValidParentheses(s, start + 1, end);
        if (s.charAt(end) == '(') return countLongestValidParentheses(s, start, end - 1);

        int max = 0;
        int current = 0;

        char[] chars = new char[end - start + 1];
        int j = -1;
        for (int i = 0; i < chars.length; i++) {
            if (s.charAt(i + start) == '(') {
                j = i;
                chars[i] = '(';
                if (max < current) max = current;
                current = 0;
            } else {
                if (0 <= j) {
                    chars[j] = '1';
                    chars[i] = '1';
                    current += 2;
                    int k = j - 1;
                    j = -1;
                    for ( ; 0 <= k; k--) {
                        if (chars[k] == '(') {
                            j = k;
                            break;
                        } else if (chars[k] == ')') {
                            break;
                        } else {
                            current++;
                        }
                    }
                } else {
                    chars[i] = ')';
                }
            }
        }
        if (max < current) max = current;
        return max;
    }


}
