package testJava;

import sun.misc.Resource;
import sun.misc.URLClassPath;
import sun.net.www.ParseUtil;

import java.io.*;
import java.lang.reflect.Field;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.net.*;
import java.security.AccessControlContext;
import java.security.ProtectionDomain;
import java.util.jar.JarFile;

public class ClassPathTest {

    private static int x = 0;

    static {
        x = 1001;
    }

    /* The search path for classes and resources */
    private final URLClassPath ucp;

    /* The context to be used when loading classes and resources */
    private final AccessControlContext acc;

    public ClassPathTest(URL[] urls) {
        this.acc = new AccessControlContext(new ProtectionDomain[0]);
        ucp = new URLClassPath(urls, acc);
    }

    public void findClass(String name) {
        String path = name.replace('.', '/').concat(".class");
        Resource res = ucp.getResource(path, false);
        if (res != null) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }
    }

    public static void main(String[] args) throws IOException, NoSuchFieldException, IllegalAccessException, NoSuchMethodException, InvocationTargetException {
        URL path = new URL("file", "", "D:/workspace/rust-jvm/");
        URL[] urls = new URL[1];
        urls[0] = path;
        ClassPathTest test = new ClassPathTest(urls);
        test.findClass("testJava/BubbleSortTest");

        File file = new File("D:\\workspace\\rust-jvm/testJava/BubbleSortTest.class");
        System.out.println("---------------start----------------");
        System.out.println("file exist:" + file.exists() + ",path:" + file.getCanonicalPath());
//        Class cls = file.getClass();
//        Field field = cls.getDeclaredField("fs");
//        field.setAccessible(true);
//        Object fs = field.get(file);
//        Method method = fs.getClass().getMethod("resolve",File.class);
//        method.setAccessible(true);
//        String res = (String) method.invoke(fs,file);
//        System.out.println("resolve path:"+ res);


        FileLoader loader = new FileLoader(path);
        Resource resource = loader.getResource("testJava/BubbleSortTest.class", false);
        if (resource != null) {
            System.out.println(true);
        } else {
            System.out.println(false);
        }

        String s = "D:\\workspace\\rust-jvm/testJava/BubbleSortTest.class";
        System.out.println(x);
        FileInputStream inputStream = new FileInputStream("D:\\workspace\\rust-jvm\\testJava\\ClassLoaderTest.class");
        byte[] data = new byte[64];
        long start = System.currentTimeMillis();
        while (inputStream.read(data) != -1) {
            //print(data);
        }
        long end = System.currentTimeMillis();
        System.out.print("spend"+(end-start)+"ms");
    }

    private static void print(byte[] data) {
        for (byte b : data) {
            System.out.print(b);
            System.out.print(" ");
        }
        System.out.print("\n");
    }

    private static class Loader implements Closeable {
        private final URL base;
        private JarFile jarfile;

        Loader(URL var1) {
            this.base = var1;
        }

        URL getBaseURL() {
            return this.base;
        }

        URL findResource(String var1, boolean var2) {
            URL var3;
            try {
                var3 = new URL(this.base, ParseUtil.encodePath(var1, false));
            } catch (MalformedURLException var7) {
                throw new IllegalArgumentException("name");
            }

            try {
                if (var2) {
                    //URLClassPath.check(var3);
                }

                URLConnection var4 = var3.openConnection();
                if (var4 instanceof HttpURLConnection) {
                    HttpURLConnection var5 = (HttpURLConnection) var4;
                    var5.setRequestMethod("HEAD");
                    if (var5.getResponseCode() >= 400) {
                        return null;
                    }
                } else {
                    var4.setUseCaches(false);
                    InputStream var8 = var4.getInputStream();
                    var8.close();
                }

                return var3;
            } catch (Exception var6) {
                return null;
            }
        }

        Resource getResource(final String var1, boolean var2) {
            return null;
        }

        Resource getResource(String var1) {
            return this.getResource(var1, true);
        }

        public void close() throws IOException {
            if (this.jarfile != null) {
                this.jarfile.close();
            }

        }

        URL[] getClassPath() throws IOException {
            return null;
        }
    }

    private static class FileLoader extends Loader {
        private File dir;

        FileLoader(URL var1) throws IOException {
            super(var1);
            if (!"file".equals(var1.getProtocol())) {
                throw new IllegalArgumentException("url");
            } else {
                String var2 = var1.getFile().replace('/', File.separatorChar);
                var2 = ParseUtil.decode(var2);
                System.out.println("vars is " + var2);
                this.dir = (new File(var2)).getCanonicalFile();
                System.out.println("Dir:" + dir.getPath());
            }
        }

        URL findResource(String var1, boolean var2) {
            Resource var3 = this.getResource(var1, var2);
            return var3 != null ? var3.getURL() : null;
        }

        Resource getResource(final String var1, boolean var2) {
            try {
                URL var4 = new URL(this.getBaseURL(), ".");
                final URL var3 = new URL(this.getBaseURL(), ParseUtil.encodePath(var1, false));
                if (!var3.getFile().startsWith(var4.getFile())) {
                    System.out.println("路径不对1");
                    return null;
                } else {
                    if (var2) {
                        //URLClassPath.check(var3);
                    }

                    final File var5;
                    if (var1.indexOf("..") != -1) {
                        var5 = (new File(this.dir, var1.replace('/', File.separatorChar))).getCanonicalFile();
                        if (!var5.getPath().startsWith(this.dir.getPath())) {
                            System.out.println("路径不对");
                            return null;
                        }
                    } else {
                        var5 = new File(this.dir, var1.replace('/', File.separatorChar));
                    }

                    if (!var5.exists()) {
                        System.out.println("dir:" + this.dir.getPath());
                        System.out.println("file not exist:" + var5.getPath());
                    }

                    return var5.exists() ? new Resource() {
                        public String getName() {
                            return var1;
                        }

                        public URL getURL() {
                            return var3;
                        }

                        public URL getCodeSourceURL() {
                            return null;
                        }

                        public InputStream getInputStream() throws IOException {
                            return new FileInputStream(var5);
                        }

                        public int getContentLength() throws IOException {
                            return (int) var5.length();
                        }
                    } : null;
                }
            } catch (Exception var6) {
                return null;
            }
        }
    }

}
