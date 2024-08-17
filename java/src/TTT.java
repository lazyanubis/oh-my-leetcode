import java.math.BigDecimal;
import java.math.RoundingMode;

/**
 * @author dawn
 * @date 2020/10/25
 */
public class TTT {
    public static void main(String[] args) {
        test(1);
        test(-1);
        test(0.1);
        test(-0.1);
        test(0.01);
        test(-0.01);
        test(0.001);
        test(-0.001);
    }

    private static void test(double value) {
        System.out.println("==================");
        System.out.println(value + " -> " + annual(BigDecimal.valueOf(value), 365));
        System.out.println(value + " -> " + annual(BigDecimal.valueOf(value), 365 / 2));
        System.out.println(value + " -> " + annual(BigDecimal.valueOf(value), 365 / 3));
    }

    private static BigDecimal annual(BigDecimal rate, int days) {
        if (days <= 1) { return rate; }
        double r = rate.doubleValue();

        double a = 0;

        if (r >= 0) {
            a = Math.pow( 1 + r, 1.0 / days);
        } else {
            a = pow(1 + r, days);
        }

        double b = Math.pow(a, 365) - 1;

        return BigDecimal.valueOf(b * 100).setScale(2, RoundingMode.DOWN);
    }

    private static double pow(double base, long times) {
        if (1 == times) {
            return base;
        }
        double p1 = -0.1;
        double p2 = 0;
        double abs;
        while (true) {
            double r1 = Math.pow(1 + p1, times);
            abs = Math.abs(r1 - base);
            if (abs < 1E-14 || Math.abs(p1 - p2) < 1E-15) {
                break;
            }
            double p3 = (p1 + p2) / 2;
            double r3 = Math.pow(1 + p3, times);
            if (r3 < base) {
                p1 = p3;
            } else {
                p2 = p3;
            }
        }
        return p1 + 1;
    }
}
