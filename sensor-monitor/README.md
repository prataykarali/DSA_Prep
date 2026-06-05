<img src="https://cdn.prod.website-files.com/677c400686e724409a5a7409/6790ad949cf622dc8dcd9fe4_nextwork-logo-leather.svg" alt="NextWork" width="300" />

# Priority Queues and Ring Buffers in Rust

**Project Link:** [View Project](https://learn.nextwork.org/projects/3ee98dd4-6222-44de-8a58-ff3e0c7c07e2)

**Author:** Dark prince  
**Email:** darkpeince309@gmail.com

---

![Image](https://learn.nextwork.org/proud_white_zealous_hyena/uploads/3ee98dd4-6222-44de-8a58-ff3e0c7c07e2_0bq682cm)

## Building a Sensor Alert Monitoring System

### Project goals and approach

In this step, I'm setting up rust and python dependencies so that I can build the temperature sensor alert system.

![Image](https://learn.nextwork.org/proud_white_zealous_hyena/uploads/3ee98dd4-6222-44de-8a58-ff3e0c7c07e2_trz8ny7s)

## Implementing the Ring Buffer in Python

### What this step builds

In this step, I'm building RingBuffer class so that the sensor monitor can push, get and iterate support, overwrite oldest data with new ones!

### Handling overflow with circular overwrite

When the buffer is full Overwrites the oldest data: The incoming value is written directly to the physical index tracked by the tail pointer, which overwrites the oldest element stored in that slot.



Advances the head pointer: Because that oldest element was just overwritten, we must advance the head pointer forward by 1 (using modulo arithmetic so it wraps around if it reaches the end). This ensures that our logical index 0 continues to point to the next oldest, surviving item.



Maintains the same size: The overall size does not increase. Because we swapped out one old item for one new item, _size stays capped exactly at capacity.

## Porting the Ring Buffer to Rust

### What this step builds

In this step, I'm implementing a generic function type RingBuffer  so that I can apply the get, push and iter methods pass as a generic parameter type T, to work out the Ring buffer

![Image](https://learn.nextwork.org/proud_white_zealous_hyena/uploads/3ee98dd4-6222-44de-8a58-ff3e0c7c07e2_3c1fmenx)

### Rust-specific features: generics and ownership

My Rust RingBuffer uses Generics with Trait Bounds (T: Clone) to ensure compile-time type safety while allowing the buffer to hold any clonable type. It also uses Lifetimes ('a) to guarantee that the iterator cannot outlive the buffer, and the Option Type (Option<T>) to represent empty/filled buffer slots safely without null pointer crashes.

## Computing Rolling Statistics with a Sliding Window

### What this step builds

In this step, I'm setting up python sliding window with statistics methods so that I can nuild slidingwindow that wraps RingBuffer and computes rolling statistics the readings of buffer

![Image](https://learn.nextwork.org/proud_white_zealous_hyena/uploads/3ee98dd4-6222-44de-8a58-ff3e0c7c07e2_rwhbdxqa)

### How the window average updates on overwrite

The sliding window's average will increase if the new value is larger than the oldest overwritten reading, decrease if it is smaller, or remain the same if they are equal. Since the window size (the divisor) stays constant at capacity, any change in the average is driven entirely by the difference between the new incoming reading and the evicted oldest reading

## Building a Priority Queue for Alert Triage

### What this step builds

In this step, I'm implementing a priority queue so that I can rank alert by severity , when multiple sensors trigger at once , a technician needs to handle the most critical issue first ! 

### How sift-up maintains the min-heap property

After inserting a new element at the end of the heap, _sift_up compares the new element with its parent node. If the new element's priority is smaller than its parent's, they swap positions. This process repeats, bubbling the element up the tree, until it either reaches the root (index 0) or finds a parent with a smaller or equal priority value, thereby restoring the min-heap property.

## Wiring the Full Sensor Alert System

### Connecting all three data structures

In this step, I'm wiring together all the 3 data structures so that I can make the program read through sliding window check for threshold violations and push alerts into priority queue

### How priority ordering surfaces the most critical alerts

The priority queue is implemented as a min-heap, which always pops the lowest numerical priority value first. To make the highest temperature print first, we negate the temperature values when pushing them (e.g., 36.8C becomes -36.8). Since -36.8 is numerically smaller than -30.5, the min-heap sifts the highest temperatures to the top of the queue. We then negate the popped values back to positive floats when displaying them.

## Bonus: Monotonic Deque for O(n) Sliding Window Maximum

![Image](https://learn.nextwork.org/proud_white_zealous_hyena/uploads/3ee98dd4-6222-44de-8a58-ff3e0c7c07e2_xptk0p3c)

### Why the deque approach beats the naive O(n*k) solution

In this project extension, the deque achieves O(n) because Every single element in your array is pushed onto the back of the deque exactly once.



An element can only be popped off the deque (either from the back because a larger item came, or from the front because it grew too old) at most once.



## Reflections and Takeaways

### Key tools and concepts learned

The key tools I used include... Key concepts I learnt include Cursor IDE as the primary code editor for navigating, editing, and comparing the Python and Rust projects side by side.



Python 3 along with its standard library debugging tools, the native assert statement, and packages like collections.deque and heapq to prototype and compare custom implementations.



The Rust Compiler and Cargo toolchain (cargo build, cargo test, and cargo run) to handle module registration, compile static types, execute built-in unit tests, and measure execution benchmarks.

Key concepts I learnt include:





Using modulo arithmetic to wrap pointers in a circular buffer, allowing infinite data streams to be mapped onto finite, pre-allocated memory.



Managing head and tail pointers along with a dedicated size counter to resolve the circular "empty-vs-full" structural ambiguity.

### Time and challenges

This project took me approximately. 2hrs. The most challenging part was last one priority queue setup.

### Personal learning goals

I did this project today to learn how to implement 3 datastructures together in a system like temperature sensor. Another skill I want to learn is RAG

---

*Built with [NextWork](https://learn.nextwork.org) - [View this project](https://learn.nextwork.org/projects/3ee98dd4-6222-44de-8a58-ff3e0c7c07e2)*
