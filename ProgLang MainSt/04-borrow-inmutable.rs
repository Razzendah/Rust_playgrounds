fn main() {
    let mut inventory = vec!["sword", "shield", "potion"];
    
    // This demonstrates Rust´s borrow checker in action
    let first_item = &inventory[0]; // borrowed as inmutable
    println!("Examining: {}", first_item);
    
    // We can read from inventory while we have an inmutable reference
    println!("Inventory size: {}", inventory.len());
    
    // But we must be done with first_item before modifying
    // inventory.push("magic ring"); // this would cause a compile error!
    
    // After we´re done with first_item, we can modify
    drop(first_item); // Explicitly dropping the reference (usually automatic)
    inventory.push("magic ring");
    
    println!("Updated inventory: {:?}", inventory);
    
    for item in &inventory {
        println!("Item in bag: {}", item);
    }
}