package testing;

public class ThreadTest {

    public static void main(String[] args) throws InterruptedException {
        Thread thread = Thread.currentThread();
        System.out.println(thread.getName());
        System.out.println(thread.getId());
        Thread child = new Thread(new Task(), "child");
        child.start();
        child.join();
//        Thread.sleep(5000);
    }

    private static class Task implements Runnable {

        @Override
        public void run() {
            System.out.println("new thread");
        }
    }
}