int _start() {
    volatile int a = 5;
    volatile int b = 2;
    volatile int c = a + b;

    return multiply(c);
}

int multiply(int a) {
    volatile int b = 10;

    return a * b;
}