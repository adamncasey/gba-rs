
int test_multiply();
int test_add();
int test_alignment();

typedef int (*test_func)();

int _start()
{
    const test_func TESTS[] = {
        test_multiply,
        test_add,
        test_alignment,
    };

    for(int i=0; i<sizeof(TESTS); i++)
    {
        const int result = TESTS[i]();

        if(result != 0)
        {
            return i;
        }
    }

    return 0;
}
