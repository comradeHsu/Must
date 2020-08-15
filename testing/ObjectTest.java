package testing;

public class ObjectTest {

    public static int staticVar;

    public int instanceVar;

    public static void main(String[] args) {
        int x = 32768;             // ldc
        ObjectTest myObj = new ObjectTest();     // new
        ObjectTest.staticVar = x;         // putstatic
        x = ObjectTest.staticVar;         // getstatic
        myObj.instanceVar = x;         // putfield
        x = myObj.instanceVar;         // getfield
        Object obj = myObj;
        if (obj instanceof ObjectTest) {     // instanceof
            myObj = (ObjectTest) obj;     // checkcast
            System.out.println(myObj.instanceVar);
        }
    }

}