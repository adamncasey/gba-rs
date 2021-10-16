int multiply(int a) {
    volatile int b = 10;

    return a * b;
}

int test_multiply()
{
    volatile int a = 5;
    volatile int b = 2;
    volatile int c = a + b;

    return multiply(c) == 100;
}