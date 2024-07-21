Chapter 16: Fearless Concurrency

Section 16.1: Using threads to run code simultaneously
* In most operating systems, an executed program's code is run
through a process.  The operating system will manage running
multiple processes at once.

Within a program, one can also have independent parts that run
simultaneously.  The features that run these independent parts are
called threads. 

Splitting the computation of one's prorgram into multiple threads
can improve performance, but it also adds complexity.

Because threads run simultaneously, there is not guarantee about the
order the different threads will run.  This can lead to problems
such as:

* Race condition, where threads access data in an inconsistent order
* Deadlocks, where two threads wait for each other, preventing both
threads from continuation
* Bugs that occur only in certain situations, and are difficult to
reproduce and fix.

Rust attempts to mitigate the negative effects of threads, but 
programs with multithreading still require a code structure that is
different from programs that run in a single thread.

Rust uses a 1:1 model, where a program uses one operating system
thread per one language thread.

Use thread::spawn to create a new thread.

Section 16.2: Using Message Passing to Transfer Data Between threads
From Go programming language, "Don't communicate by sharing memory; instead,
share memory by communicating."

Rust uses channels to send data from one thread to another.

A channel has two halves, a transmitter and a receiver.
A channel is said to be closed if either the transmitter or receiver is dropped.

A receiver has two methods, recv and try_recv.  The recv command blocks the
main current thread's execution and wait until a value is sent down the channel.
The try_recv command does not block, but instead, returns a Result<T, E>
immediately, with an Ok value holding the message if a message is available, and
with an Err value if there aren't any messages at this time.

Section 16.3: Shared-State Concurrency

Mutex is short for mutual exclusion: mutex allows only one thread to access some
data at a time.  To access data, a thread must signal to mutex that the thread
wants to acquire the mutex's lock.  The lock is a data structure that keeps track
of who currently has exclusive access to the data.  Thus, mutex is described
as guarding the data it holds via a locking system.

