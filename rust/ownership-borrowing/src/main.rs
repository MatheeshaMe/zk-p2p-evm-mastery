fn main() {
    let variable_1 = String::from("Hello");
    let variable_2 = variable_1; // Ownership of variable_1 moved to variable_2
    println!("{}",variable_2); // this works fine
    println!("{}",variable_1);
}