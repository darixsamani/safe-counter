### Safe counter in Rust

Development of a thread-safe counter with parallel increment of 10 threads.

### Workshop: Development of Thread-Safe Counter

In this tutorial we will implement parallel increment of a common counter.

We will combine Arc and Mutex, create 10 OS system threads, each of which will increase our counter by exactly 1000 units.

We will wait for all threads to complete (join) and check that the final counter value is exactly 10000 without any data loss (Race Conditions).