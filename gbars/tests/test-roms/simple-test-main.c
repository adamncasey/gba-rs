
int _start()
{
    volatile int a = 5;
    volatile int b = 2;
    int c = a + b;

    return c == 7;
}
