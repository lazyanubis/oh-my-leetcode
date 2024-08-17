/**
 * @author dawn
 * @date 2020/06/05
 */
public class Test1 {

    private volatile int v = 0;
    private Object lock = new Object();

    public static void main(String[] args) {
        Test1 t = new Test1();
        new Thread(() -> {
            synchronized (t.lock) {
                for (int i = 0; i < 5; i++) {
                    try {
                        t.lock.notifyAll();
                        t.lock.wait();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                    System.out.println(Thread.currentThread().getName() + "\t" + t.v);
                    t.v++;
                }
                t.lock.notifyAll();
            }
        }).start();
        new Thread(() -> {
            synchronized (t.lock) {
                for (int i = 0; i < 5; i++) {
                    try {
                        t.lock.notifyAll();
                        t.lock.wait();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                    System.out.println(Thread.currentThread().getName() + "\t" + t.v);
                    t.v--;
                }
                t.lock.notifyAll();
            }
        }).start();
        new Thread(() -> {
            synchronized (t.lock) {
                for (int i = 0; i < 5; i++) {
                    try {
                        t.lock.notifyAll();
                        t.lock.wait();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                    System.out.println(Thread.currentThread().getName() + "\t" + t.v);
                    t.v++;
                }
                t.lock.notifyAll();
            }
        }).start();
        new Thread(() -> {
            synchronized (t.lock) {
                for (int i = 0; i < 5; i++) {
                    try {
                        t.lock.notifyAll();
                        t.lock.wait();
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                    System.out.println(Thread.currentThread().getName() + "\t" + t.v);
                    t.v--;
                }
                t.lock.notifyAll();
            }
        }).start();
    }

}
