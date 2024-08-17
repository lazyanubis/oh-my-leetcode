import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

/**
 * @author dawn
 * @date 2020/06/05
 */
public class Test2 {

    private int v = 0;
    private Lock lock2 = new ReentrantLock();
    private Condition condition = lock2.newCondition();

    public static void main(String[] args) {
        Test2 t = new Test2();
        new Thread(() -> {
            try {
                t.lock2.lock();
                for (int i = 0; i < 5; i++) {
                    try {
                        t.condition.signal();
                        t.condition.await();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                    System.out.println(t.v);
                    t.v++;
                }
            } finally {
                t.lock2.unlock();
            }
        }).start();
        new Thread(() -> {
            try {
                t.lock2.lock();
                for (int i = 0; i < 5; i++) {
                    try {
                        t.condition.signal();
                        t.condition.await();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                    System.out.println(t.v);
                    t.v--;
                }
            } finally {
                t.lock2.unlock();
            }
        }).start();
    }

}
