package java;
public class SystemTest {

    public static void main(String[] args) {
        byte[] s = new byte[]{1,2,3,4,5,6};
        byte[] d = new byte[]{7,8,9,10,11,12};
        System.arraycopy(d,0,s,0,6);
        System.out.println(s[0]);
        System.arraycopy(s,1,s,5,1);
        System.out.println(s[5]);
    }
}