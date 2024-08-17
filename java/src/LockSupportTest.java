import java.util.concurrent.locks.LockSupport;

public class LockSupportTest {

    public static void main(String[] args) {
        Thread a = new Thread(() -> {
            System.out.println("a come in ");
            sleep(2);
            LockSupport.park();
            System.out.println("a 1");
            sleep(2);
            LockSupport.park();
            System.out.println("a 2");
            sleep(2);
            LockSupport.park();
            System.out.println("a come out");
        }, "a");
        a.start();

        new Thread(() -> {
            System.out.println("b come in");
            sleep(5);
            LockSupport.unpark(a);
            sleep(5);
            LockSupport.unpark(a);
            System.out.println("b come out");
        }, "b").start();

    }

    private static void sleep(long ms) {
        try {
            Thread.sleep(ms * 1000);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
    }

}
