int test_add()
{
    volatile int a = 5;
    volatile int b = 2;
    volatile int c = a + b;

    return c == 7;
}
