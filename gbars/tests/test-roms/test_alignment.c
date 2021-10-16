#include <stdint.h>

int test_alignment()
{
    volatile uint32_t *iwram_start = (uint32_t*)0x3000000;

    // Write some value where every byte is unique
    *iwram_start = 0xdeadbeef;

    volatile uint8_t* iwram_start_u8 = (uint8_t*)iwram_start;

    uint32_t EXPECTED[] = {0xdeadbeef, 0xefdeadbe, 0xbeefdead, 0xadbeefde};

    for (int i=0; i < 4; i++)
    {
        volatile uint32_t *val = (uint32_t*)(iwram_start_u8 + i);
        uint32_t actual = *val;

        if (actual != EXPECTED[i]) {
            return 1;
        }
    }

    return 0;
}