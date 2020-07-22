package testJava;

import java.util.*;

public class BoxTest {
    public static void main(String[] args) throws InterruptedException{
        List<Integer> list = new ArrayList<>();
        list.add(1);
        list.add(2);
        list.add(3);
        for (int x : list) {
            Thread.sleep(3000);
            System.out.println(x);
        }
        System.out.println(System.getProperty("java.library.path"));
//        for (;;) {
//            School school = new School();
//            User user = new User("xiaohong",school);
//            school.setUser(user);
//        }
    }

    public void test(boolean bool){
        assert bool;
    }
}

//class User {
//    private String name;
//    private School school;
//    public User(String name, School school) {
//        this.name = name;
//        this.school = school;
//    }
//}
//
//class School {
//    private User user;
//
//    public void setUser(testJava.User user) {
//        this.user = user;
//    }
//}