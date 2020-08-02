package testJava;

import java.util.*;

public class ExceptionTest {
    public static void main(String[] args) {
        ExceptionTest test = new ExceptionTest();
        try {
            test.first();
        }catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void first(){
        System.out.println("method in : first");
        second();
    }

    public void second(){
        System.out.println("method in : second");
        third();
    }

    public void third(){
        System.out.println("method in : third");
        throwException();
    }

    public void throwException(){
        throw new NullPointerException("null");
    }
}