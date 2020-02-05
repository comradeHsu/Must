package testJava;

public enum Color {
    BLUE,
    RED,
    BLACK,
    YELLOW
}

enum RGB {
    BLUE(255),
    RED(126),
    WHITE(1);

    private int param;

    RGB(int param) {
        this.param = param;
    }
}