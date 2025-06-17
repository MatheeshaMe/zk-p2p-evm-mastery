fn main() {
    // let variable_1 = String::from("Hello");
    // let variable_2 = variable_1; // Ownership of variable_1 moved to variable_2
    // println!("{}",variable_2); // this works fine
    // println!("{}",variable_1);
    // i_and_m_borrow();
    scopes_of_borrows();
}

fn i_and_m_borrow(){
    let mut s = String::from("hello");
    let r1 = &s;
    // let r2 = &mut s;
    println!("{}",r1);
}

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