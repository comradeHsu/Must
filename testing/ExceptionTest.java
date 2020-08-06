package testing;

public class ExceptionTest {

    public static void main(String[] var0) {
        ExceptionTest var1 = new ExceptionTest();

        try {
            var1.first();
        } catch (Exception var3) {
            var3.printStackTrace();
        }

    }

    public void first() {
        System.out.println("method in : first");
        this.second();
    }

    public void second() {
        System.out.println("method in : second");
        this.third();
    }

    public void third() {
        System.out.println("method in : third");
        this.throwException();
    }

    public void throwException() {
        throw new NullPointerException("null");
    }
}