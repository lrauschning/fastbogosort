# Fastbogo

Fastbogo is an efficient, multithreaded implementation of the bogosort algorithm.
For any given number of cores, the runtime follows a Poisson distribution, with both the expected runtime and the standard deviation in O(n!), with n as the length of the input array.
In the limit of an infinite number of available cores, the runtime will approach O(n) even for non-presorted input arrays, outperforming other sorting algorithms.

Quantum bogosort to be added at a later date.

Uses features only available on unix systems for cleaning up the threads.
If the relevant code is commented out, fastbogo should also work on other operating systems (untested).
