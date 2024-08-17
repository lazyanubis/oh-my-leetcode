import java.util.ArrayList;
import java.util.Collections;
import java.util.LinkedList;
import java.util.List;
import java.util.concurrent.CountDownLatch;
import java.util.stream.Collectors;

/**
 * @author dawn
 * @date 2020/05/12
 */
public class Test {

    static int i1 = 0;
    static int i2 = 0;

    public static void main(String[] args) {
        String num = "12345678";
        String cha = "ABCDEFGH";
        Object o = new Object();

        new Thread(() -> {
            while (i1 < 8) {
                synchronized (o) {
                    try {
                        o.wait();
                        o.notify();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                    System.out.print(num.charAt(i1++));
                }
            }
        }).start();
        new Thread(() -> {
            while (i2 < 8) {
                synchronized (o) {
                    try {
                        o.notify();
                        o.wait();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                    System.out.print(cha.charAt(i2++));
                }
            }
        }).start();


    }


}

