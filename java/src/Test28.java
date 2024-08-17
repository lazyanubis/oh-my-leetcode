import java.util.ArrayList;
import java.util.List;

/**
 * @author dawn
 * @date 2020/07/07
 */
public class Test28 {

    public static void main(String[] args) {
        System.out.println(new Test28().divide(-2147483648, 2));
    }

    public int divide(int dividend, int divisor) {
        if (dividend == Integer.MIN_VALUE && -1 == divisor) return Integer.MAX_VALUE;

        if (0 == dividend) return 0;
        if (1 == divisor) return dividend;
        if (-1 == divisor) return -dividend;

        boolean sign = 0 != ((0 < dividend ? 1 : -1) + (0 < divisor ? 1 : -1));

        dividend = dividend < 0 ? dividend : -dividend;
        divisor = divisor < 0 ? divisor : -divisor;

        if (dividend > divisor) return 0;
        if (dividend == divisor) return sign ? 1 : -1;

        List<Integer> times = new ArrayList<>();
        List<Integer> values = new ArrayList<>();

        long temp = divisor;
        int i = -1;
        while (temp > dividend) {
            times.add(i);
            values.add((int)temp);
            i = i + i;
            temp = temp + temp;
        }

        int value = 0;
        for (i = times.size() - 1; 0 <= i; i--) {
            if (values.get(i) >= dividend) {
                dividend -= values.get(i);
                value -= times.get(i);
                i++;
            }
        }

        return sign ? value : -value;
    }

}
