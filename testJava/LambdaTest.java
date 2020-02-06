package testJava;

public class LambdaTest {
    public static void main(String[] args) {
        int x = LambdaTest.run(new Action() {

            @java.lang.Override
            public int run() {
                return 1994;
            }
        });
        System.out.println(x);
    }

    public static native int run(Action action);
}

interface Action {
    int run();
}