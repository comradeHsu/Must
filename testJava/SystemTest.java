package testJava;
public class SystemTest {

    public static void main(String[] args) {
        byte[] s = new byte[]{1,2,3,4,5,6};
        byte[] d = new byte[]{7,8,9,10,11,12};
        System.arraycopy(d,0,s,0,6);
        System.out.println(s[0]);
        System.arraycopy(s,1,s,4,1);
        System.out.println(s[4]);
        System.out.println(s[5]);
        double[] f = new double[]{1.0,2.0,3.0,4.0,5.0,6.0};
        double[] g = new double[]{7.0,8.0,9.0,10.0,11.0,12.0};
        System.arraycopy(g,0,f,0,6);
        System.out.println(f[0]);
        System.arraycopy(f,1,f,4,1);
        System.out.println(f[4]);
        System.out.println(f[5]);
    }
}