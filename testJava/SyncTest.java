package testJava;
public class SyncTest {

    private int count = 0;

    public static void main(String[] args) {

    }

    public synchronized int random() {
        return 99;
    }

    public void test() {
        synchronized (this) {
            count++;
        }
    }

    public void println() {
        synchronized (SyncTest.class) {
            System.out.println(666);
        }
    }
}