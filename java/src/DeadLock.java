public class DeadLock {

    static class Test implements Runnable {
        private String a;
        private String b;

        public Test(String a, String b) {
            this.a = a;
            this.b = b;
        }

        @Override
        public void run() {
            synchronized (a) {
                System.out.println(Thread.currentThread().getName() + " " + a + " -> " + b);
                synchronized (b) {
                    System.out.println(Thread.currentThread().getName() + " " + a + " got " + b);
                }
            }
        }
    }

    public static void main(String[] args) {
        String a = "A";
        String b = "B";
        new Thread(new Test(a, b)).start();
        new Thread(new Test(b, a)).start();
    }

}
