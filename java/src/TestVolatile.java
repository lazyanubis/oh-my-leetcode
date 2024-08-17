import java.util.concurrent.CountDownLatch;

public class TestVolatile {
    public static void main(String[] args) {
//        new TestVolatile().test0();
//        new TestVolatile().test1();
        for (int i = 0; i < 20; i++) {
            new TestVolatile().test3();
        }
        new TestVolatile().test3();
    }
    private void test0() {
        Data d = new Data();
        new Thread(() -> {
            System.out.println("come in");
            try {
                Thread.sleep(3000);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
            d.number0 = 50;
            System.out.println("come out");
        }).start();

        while (d.number0 == 0){}

        System.out.println("mission over");
    }
    private void test1() {
        Data d = new Data();
        new Thread(() -> {
            System.out.println("come in");
            try {
                Thread.sleep(3000);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
            d.number1 = 50;
            System.out.println("come out");
        }).start();

        while (d.number1 == 0){}

        System.out.println("mission over");
    }

    private void test2() {
        Data d = new Data();
        for (int i = 0; i < 20; i++) {
            new Thread(() -> {
                for (int j = 0; j < 1000; j++) {
                    d.addOne();
                }
            }, String.valueOf(i)).start();
        }
        while (Thread.activeCount() > 2){
            Thread.yield();
        }
        System.out.println("number1: " + d.number1);
    }

    private void test3() {
        Data d = new Data();
        CountDownLatch latch = new CountDownLatch(20);
        for (int i = 0; i < 20; i++) {
            new Thread(() -> {
                for (int j = 0; j < 1000; j++) {
                    d.addOne();
                }
                latch.countDown();
            }, String.valueOf(i)).start();
        }
        try {
            latch.await();
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        System.out.println("number1: " + d.number1);
    }

}

class Data {
    int number0 = 0;
    volatile int number1 = 0;
    synchronized void addOne() {
        number1++;
    }
}
