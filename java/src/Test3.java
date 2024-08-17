import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.ReentrantLock;

/**
 * @author dawn
 * @date 2020/06/05
 */
public class Test3 {

    public static void main(String[] args) {
        ReentrantLock lock = new ReentrantLock();
        Condition ac = lock.newCondition();
        Condition bc = lock.newCondition();
        Condition cc = lock.newCondition();

        new Thread(() -> {
            lock.lock();
            try {
                for (int i = 0; i < 15; i++) {
                    System.out.println("AA");
                    bc.signal();
                    try {
                        ac.await();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                }
            } finally {
                bc.signal();
                lock.unlock();
            }
        }, "AA").start();
        new Thread(() -> {
            lock.lock();
            try {
                for (int i = 0; i < 15; i++) {
                    System.out.println("BB");
                    cc.signal();
                    try {
                        bc.await();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                }
            } finally {
                cc.signal();
                lock.unlock();
            }
        }, "BB").start();
        new Thread(() -> {
            lock.lock();
            try {
                for (int i = 0; i < 15; i++) {
                    System.out.println("CC");
                    ac.signal();
                    try {
                        cc.await();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                }
            } finally {
                lock.unlock();
            }
        }, "CC").start();

    }

}
