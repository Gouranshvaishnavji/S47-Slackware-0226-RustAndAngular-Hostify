# Ownership, Borrowing & Lifetimes — Practical Guide

This document explains Rust's ownership model in simple terms and provides short backend-focused examples you can include in a PR.

## 1. Ownership (in plain words)

- Every value in Rust has a single owner (a variable). When the owner goes out of scope, the value is dropped and resources are freed.
- Moving a value transfers ownership. After a move, the previous owner can no longer use the value.
- This model avoids double-free and many classes of memory errors found in manual memory management.

### Ownership example

```rust
fn ownership_example() {
    let s1 = String::from("hello");
    let s2 = s1; // ownership moves from s1 to s2
    // println!("{}", s1); // ERROR: s1 is no longer valid after the move
    println!("s2 = {}", s2);
}
```

Why: `String` owns heap data. Assigning `s1` to `s2` moves the `String` (not a shallow copy). Rust prevents use of `s1` afterwards to avoid double-free.

## 2. Borrowing

- Instead of moving ownership, you can *borrow* a value via references.
- Immutable borrow `&T` allows any number of readers. Mutable borrow `&mut T` allows exactly one writer and no other borrows.
- Borrowing rules are enforced at compile time to prevent data races and invalid memory access.

### Borrowing example (correct)

```rust
fn print_len(s: &String) {
    println!("len = {}", s.len());
}

fn borrowing_ok() {
    let s = String::from("hello");
    print_len(&s); // immutable borrow
    print_len(&s); // many immutable borrows allowed
}
```

### Borrowing error example (incorrect)

```rust
fn mutate(s: &mut String) { s.push_str("!"); }

fn borrowing_error() {
    let mut s = String::from("hi");
    let r1 = &mut s;
    let r2 = &mut s; // ERROR: cannot borrow `s` as mutable more than once
    // use r1 and r2...
}
```

This rule prevents concurrent mutation bugs at compile time.

## 3. Lifetimes (brief)

- Lifetimes describe how long references are valid. Rust's borrow checker uses lifetimes to ensure no reference outlives the value it points to.
- Lifetime annotations (`'a`) express that multiple references are tied to the same scope so the compiler can check validity.

### Lifetime example

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn use_longest() {
    let a = String::from("abcd");
    let b = "xyz";
    let r = longest(a.as_str(), b);
    println!("longest = {}", r);
}
```

What `'a` guarantees: the returned reference lives at least as long as both input references, so it cannot outlive the data it points to.

## 4. Why this matters for backend development (Actix/Axum)

- Preventing dangling references: handlers return owned data (or references guaranteed by lifetimes), avoiding invalid responses.
- Avoiding invalid data returned in handlers: borrow rules force you to explicitly manage ownership when moving data into async handlers or across threads.
- Preventing race conditions in async tasks: Rust's ownership and borrowing model plus Send/Sync traits make it explicit what can be shared across threads.
- Ensuring DB connections are used safely: typed DB pools and ownership patterns ensure connections are checked out, used, and returned without leaks.

Example backend scenarios:
- Return a JSON value from a handler by moving ownership into the response (no dangling refs).
- Use `Arc<Mutex<T>>` or connection pools with clear ownership when shared across async tasks — the compiler enforces correct usage patterns.

## 5. PR steps & AI review

1. Add this file to your PR.  
2. Request a CodiumAI review by adding a PR comment or mentioning:

```
@CodiumAI-Agent /review
```

3. Apply the suggestions from the AI and commit updates so the PR shows visible improvement.

---
File: [conceptKlarity/docs/RUST_OWNERSHIP.md](conceptKlarity/docs/RUST_OWNERSHIP.md)
