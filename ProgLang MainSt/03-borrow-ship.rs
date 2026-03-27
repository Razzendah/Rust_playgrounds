fn main() {
    // Creating and transferring ownership
    let spaceship = String::from("Millenium Falcon");
    
    // Ownership moves to new_owner
    let new_owner = take_ownership(spaceship);
    
    // This woul cause a compile error:
    // println!("Original: {}", spaceship); // spaceship is no longer valid
    println!("New owner has: {}", new_owner);
    
    // Borrowing instead of moving
    let enterprise = String::from("USS Enterprise");
    let length = get_length(&enterprise);
    
    println!("Ship: {} has {} characters", enterprise, length);
}

fn take_ownership(ship: String) -> String {
    println!("Taking control of: {}", ship);
    ship // returning ownership back
}

fn get_length(ship: &String) -> usize {
    println!("Scanning ship: {}", ship);
    ship.len() // borrowing, not taking ownership
}