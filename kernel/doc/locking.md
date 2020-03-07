Consider:
- CPU-ID holding the lock
- Pushing/Popping Interrupt Disable

No SMP
- Spin locks aren't required, we can just disable pre-emption while the lock is held
- Preempt/context switch while spin lock is held should panic

SMP
- If we try to take a spin lock that is held by the current thread, panic

Notes:
- soft IRQs - should be able to disable soft interrupts as well
- harware IRQs - most restrictive should disable hw interrupts

Links: 
- https://www.kernel.org/doc/Documentation/locking/spinlocks.txt
- https://preshing.com/20120305/implementing-a-recursive-mutex/
- https://www.kernel.org/doc/htmldocs/kernel-locking/index.html