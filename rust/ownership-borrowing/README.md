# Ownership and borrowing

Ownership in rust is most unique to rust language, It helps rust to manage memeory without needing a garbage collector.

## What is Garbage Collection (GC)

garbale collection is the automated process of reclaiming memory periodically or on demand if the heap is close to full or above some threshold, it then looks for unused and frees their memory depending on the algorithm.


## How Rust handles this

In rust memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

## Rust Ownership rules

```This ensures that memory is deallocated safely when it’s no longer needed.```

  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.

### Ownership in Action

```rust
fn main() {
    let variable = String::from("hello"); // variable is the owner of the String
    println!("{}", s1); // s1 is used
} // variable goes out of scope, String is dropped
```

### Moving Ownership

```rust
fn main() {
    let variable_1 = String::from("Hello");
    let variable_2 = variable_1; // Ownership of variable_1 moved to variable_2
    println!("{}",variable_2) // this works fine
    /*after adding  println!("{}",variable_1) line will get an erroe (compile time error)
      ^^^^^^^^^^ value borrowed here after move
    */
    println!("{}",variable_1)
}
```

   - Types like String (heap-allocated) are moved.
   - Simple types like `i32` or `bool` (Stored on the stack)
 are **copied** because they implement the ```Copy``` trait.

 
 ## Final Notes

  - Ownership ensures memory safety by tracking who "owns" data.
  - Moves transfer ownership; copies duplicate stack data.
  - Rust’s compiler enforces these rules at compile time, preventing bugs like use-after-free.