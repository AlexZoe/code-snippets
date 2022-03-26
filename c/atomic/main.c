/*
 * Modified example taken from https://en.cppreference.com/w/c/language/atomic
 */
#include <stdio.h>
#include <threads.h>
#include <stdatomic.h>
#include <signal.h>

atomic_int acnt;

// The following is used for signal handlers and cannot be used for inter-thread
// communication
// volatile sig_atomic_t acnt;

int f(void* thr_data)
{
    for(int n = 0; n < 1000; ++n) {
        // Uses memory_order_seq_cst for built-in increment/decrement and
        // compound assignment
        ++acnt;
        // for this example, relaxed memory order is sufficient, e.g.
        // atomic_fetch_add_explicit(&acnt, 1, memory_order_relaxed);
    }
    return 0;
}

int main(void)
{
    acnt = 0;
    printf("is lock free: %s\n",
            atomic_is_lock_free(&acnt) ? "true" : "false");
    thrd_t thr[10];
    for(int n = 0; n < 10; ++n)
        thrd_create(&thr[n], f, NULL);
    for(int n = 0; n < 10; ++n)
        thrd_join(thr[n], NULL);

    printf("The atomic counter is %u\n", acnt);
}
