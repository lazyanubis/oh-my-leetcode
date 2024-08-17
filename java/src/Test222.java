/**
 * @author dawn
 * @date 2020/06/17
 */
public class Test222 {

    public static void main(String[] args) {
        System.out.println(new Test222().myAtoi("-91283472332"));
    }

    public int myAtoi(String str) {
        if (null == str) return 0;
        str = str.trim();
        if ("".equals(str)) return 0;
        char first = str.charAt(0);
        boolean sign = true;
        int index = 0;
        if ('-' == first || '+' == first || ('0' <= first && first <= '9')) {
            if ('-' == first) { sign = false; index = 1; }
            if ('+' == first) { index = 1; }
            char next;
            long value = 0;
            while(true) {
                next = str.charAt(index);
                if ('0' <= next && next <= '9') {
                    value = value * 10 + (next - '0');
                    if (sign && Integer.MAX_VALUE < value) return Integer.MAX_VALUE;
                    if (!sign && value < Integer.MIN_VALUE) return Integer.MIN_VALUE;
                    index++;
                    if (str.length() <= index) return sign ? (int) value : (int) -value;
                } else {
                    return sign ? (int) value : (int) -value;
                }
            }
        }
        return 0;
    }

}
