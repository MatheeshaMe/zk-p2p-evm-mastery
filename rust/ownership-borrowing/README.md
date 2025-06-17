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


  ## Borrowing

  Borrowing lets us use a value without taking the ownership instead of moving data, create a reference to it

  ## Rust Borrowing Rules

  - You can have any number of immutable references (&T) to a value.
  - You can have only one mutable reference (&mut T) at a time.
  - You cannot have mutable and immutable references to the same value simultaneously.
  - References must always be valid (no dangling pointers).


## Immutable Borrowing
  
  Allow read-only access

  ```rust 
  fn main() {
      let s = String::from("hello");
      let r1 = &s;
      let r2 = &s; // this is allowed
      println!("r1: {}, r2: {}", r1, r2); // Works
  }
  ```
## Mutable Borrowing

Mutable references (&mut T) allow modifying the data, but only one mutable reference can exist at a time.

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(", world!");
    println!("{}",r1);
    // let r2 = &mut s; // Error: cannot have another mutable borrow
}
```

## Mixing Immutable and Mutable Borrows

Can't Have an immutable borrow and a mutable borrow active at the same time

```rust
fn i_and_m_borrow(){
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s; //ERROR :: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}",r1);
}
```

## Scope of Borrows

Borrows are tied to their scope. Once a borrow’s scope ends, you can create new borrows.

```rust
fn scopes_of_borrows(){
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(", world!");
    }
    let r2 = &mut s;
    r2.push_str("!!");
    println!("{}",r2);
}
```