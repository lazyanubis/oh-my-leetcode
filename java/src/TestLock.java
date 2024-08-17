import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

public class TestLock {

    static class Data {
        private int number = 0;
        private Lock lock = new ReentrantLock();
        private Condition a = lock.newCondition();
        private Condition b = lock.newCondition();
        private Condition c = lock.newCondition();

        void print5() {
            lock.lock();
            try {
                // 判断
                while (number != 0) {
                    a.await();
                }
                // 干活
                System.out.println("AAA");
                // 发通知
                number = 1;
                b.signal();
            } catch (InterruptedException e) {
                e.printStackTrace();
            } finally {
                lock.unlock();
            }
        }

        void print10() {
            lock.lock();
            try {
                // 判断
                while (number != 1) {
                    b.await();
                }
                // 干活
                System.out.println("BBB");
                // 发通知
                number = 2;
                c.signal();
            } catch (InterruptedException e) {
                e.printStackTrace();
            } finally {
                lock.unlock();
            }
        }

        void print15() {
            lock.lock();
            try {
                // 判断
                while (number != 2) {
                    c.await();
                }
                // 干活
                System.out.println("CCC");
                // 发通知
                number = 0;
                a.signal();
            } catch (InterruptedException e) {
                e.printStackTrace();
            } finally {
                lock.unlock();
            }
        }

    }

    public static void main(String[] args) {
        Data d = new Data();
        for (int i = 0; i < 5; i++) { new Thread(d::print5).start(); }
        for (int i = 0; i < 5; i++) { new Thread(d::print10).start(); }
        for (int i = 0; i < 5; i++) { new Thread(d::print15).start(); }
    }

}
