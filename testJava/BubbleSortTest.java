package testJava;

import java.lang.annotation.ElementType;
import java.lang.annotation.Inherited;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;


@Cell
public class BubbleSortTest {

    @Lua(bit = 100,name = {})
    private int init;

    private Color color = Color.BLUE;
    public static void main(String[] args) {
        for (String arg: args) {
            System.out.println(arg);
        }
        System.out.println("start sort");
        int[] arr = {22, 84, 77, 11, 95,  9, 78, 56, 36, 97, 65, 36, 10, 24 ,92};
        bubbleSort(arr);
        printArray(arr);
        System.out.println(void.class.getName()); // void
        System.out.println(boolean.class.getName()); // boolean
        System.out.println(byte.class.getName()); // byte
        System.out.println(char.class.getName()); // char
        System.out.println(short.class.getName()); // short
        System.out.println(int.class.getName()); // int
        System.out.println(long.class.getName()); // long
        System.out.println(float.class.getName()); // float
        System.out.println(double.class.getName()); // double
        System.out.println(Object.class.getName()); // java.lang.Object
        System.out.println(int[].class.getName()); // [I
        System.out.println(int[][].class.getName()); // [[I
        System.out.println(Object[].class.getName()); // [Ljava.lang.Object;
        System.out.println(Object[][].class.getName()); // [[Ljava.lang.Object;
        System.out.println(Runnable.class.getName()); // java.lang.Runnable
        System.out.println("abc".getClass().getName()); // java.lang.String
        System.out.println(new double[0].getClass().getName()); // [D
        System.out.println(new String[0].getClass().getName()); //[Lja
    }
    private static void bubbleSort(int[] arr) {
        boolean swapped = true;
        int j = 0;
        int tmp;
        while (swapped) {
            swapped = false;
            j++;
            for (int i = 0; i < arr.length - j; i++) {
                if (arr[i] > arr[i + 1]) {
                    tmp = arr[i];
                    arr[i] = arr[i + 1];
                    arr[i + 1] = tmp;
                    swapped = true;
                }
            }
        }
    }
    private static void printArray(int[] arr) {
        for (int i : arr) {
            System.out.println(i);
        }
    }

    @Cell
    @Lua(name={"test","lua"},value = 99,code = "hahaha",type = ElementType.METHOD,ints = {0,1,2})
    public void test() {
        System.out.println(99);
    }
}

@Target({ElementType.METHOD, ElementType.TYPE})
@Retention(RetentionPolicy.RUNTIME)
@interface Cell {

}

@Target({ElementType.METHOD, ElementType.TYPE, ElementType.FIELD})
@Retention(RetentionPolicy.RUNTIME)
@interface Lua {

    String[] name() default {};

    int value() default -1;

    boolean bool() default false;

    byte bit() default 1;

    char charCode() default 'L';

    short shot() default 9;

    long longSet() default 100;

    float floaty() default 0.0f;

    double doubley() default 1.0;

    String code() default "code";

    ElementType type() default ElementType.FIELD;

    int[] ints() default {};

    Class cla() default Void.class;
}

enum Color {
    BLUE,
    RED,
    BLACK,
    YELLOW
}
