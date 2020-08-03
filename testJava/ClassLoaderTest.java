package testJava;

import java.io.ByteArrayOutputStream;
import java.io.FileInputStream;
import java.lang.reflect.Constructor;

public class ClassLoaderTest {

    public static void main(String[] args) {
        ClassLoader cl = new SelfClassLoader() ;
        cl.getParent();
        try {
            Class clz = cl.loadClass("com.example.demo.controller.User") ;
            System.out.println("load done");
            Constructor constructor = clz.getConstructor(String.class, int.class);
            Object obj = constructor.newInstance("John", 18);
            System.out.println(obj);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}

class SelfClassLoader extends ClassLoader {
    protected Class<?> findClass(String name) {
        String rename = name.replace('.','/');
        try {
            String path = "D:\\workspace\\demo\\" + rename + ".class1" ;
            FileInputStream in = new FileInputStream(path) ;
            ByteArrayOutputStream baos = new ByteArrayOutputStream();
            byte[] buf = new byte[1024] ;
            int len = -1 ;
            while((len = in.read(buf)) != -1){
                baos.write(buf , 0 , len);
            }
            in.close();
            byte[] classBytes = baos.toByteArray();
            return defineClass(name,classBytes , 0 , classBytes.length);
        } catch (Exception e) {
            e.printStackTrace();
        }
        return null ;
    }
}

class User {

    private String name;

    private int age;

    public User() {

    }

    public User(String name,int age) {
        this.name = name;
        this.age = age;
    }

    @Override
    public String toString() {
        return "User {" + "Name = " + name + ", Age = " + age + "}";
    }
}