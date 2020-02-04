package testJava;

import java.util.*;

public class BoxTest {
    public static void main(String[] args) {
        List<Integer> list = new ArrayList<>();
        list.add(1);
        list.add(2);
        list.add(3);
        list.getClass().getAnnotations();
        System.out.println(list.toString());
        for (int x : list) {
            System.out.println(x);
        }
    }
}