# 🦀 Rust Memory Management – Learning Notes

## 📚 What I Learned Today

* Memory Management Approaches
* Garbage Collector vs Manual vs Rust Ownership
* Stack vs Heap
* How `String` works internally
* Why Rust is memory safe without GC

---

# 1️⃣ Memory Management Approaches

## 🟢 Garbage Collector (GC)

**Used in:** Java, JavaScript, Python

### How it Works:

* Memory allocation is automatic.
* Garbage collector frees unused memory automatically.

### Advantages:

* No manual memory freeing
* Rare dangling pointers
* Easier for beginners

### Disadvantages:

* Less control
* GC pause can affect performance

---

## 🟠 Manual Memory Management

**Used in:** C, C++

### How it Works:

* Programmer allocates memory manually
* Programmer must free memory manually

### Advantages:

* Full control
* High performance

### Disadvantages:

* Memory leaks
* Dangling pointers
* Double free errors
* Hard to debug

---

## 🔵 The Rust Way (Ownership Model)

Rust does NOT use:

* Garbage Collector ❌
* Manual memory freeing like C ❌

### Ownership Rules:

1. Each value has one owner
2. When owner goes out of scope → memory is freed
3. Only one mutable reference OR many immutable references

### Why Rust is Powerful:

* No GC pauses
* No memory leaks (safe Rust)
* No dangling pointers
* Compile-time memory safety

---

# 2️⃣ Stack vs Heap

## 🟦 Stack

### Characteristics:

* Very fast
* Fixed size
* Stores small/simple data
* Automatically cleaned when scope ends
* Follows LIFO (Last In First Out)

### Example:

```rust
let x: i32 = 1;     // 4 bytes
let y: bool = true;
```

Stored directly on the stack because:

* Size known at compile time
* Small data

---

## 🟥 Heap

### Characteristics:

* Slower than stack
* Used for dynamic data
* Size can grow/shrink
* Requires allocation

### Example:

```rust
let z = String::from("hello");
```

### What Happens Internally:

**Stack stores:**

* Pointer
* Length
* Capacity

**Heap stores:**

* Actual string data ("hello")

---

# 3️⃣ How `String` Works

When we do:

```rust
let z = String::from("asa");
```

Memory layout:

STACK:

* z → (pointer, length, capacity)

HEAP:

* 'a', 's', 'a'

If we use:

```rust
z.push_str("more");
```

The heap allocation can grow dynamically.

---

# 4️⃣ Why Stack is Faster Than Heap

* Stack allocation = moving a pointer
* Heap allocation = searching free memory + bookkeeping
* Stack has better CPU cache locality

---

# 5️⃣ Key Takeaways

* Stack → Fast, fixed size, simple data
* Heap → Flexible, dynamic size
* Rust uses Ownership instead of GC
* Memory is freed automatically when owner goes out of scope
* `String` uses both stack (metadata) and heap (actual data)

---

# 🚀 Next Topics to Learn

* Move vs Copy
* Borrowing (&)
* Mutable vs Immutable references
* Lifetimes
* How Rust prevents dangling pointers at compile time

---

## ✅ Summary

Today I built strong fundamentals in:

* Memory Management concepts
* Stack vs Heap understanding
* Internal working of `String`
* Rust Ownership basics

This forms the foundation for mastering Rust and systems programming.
