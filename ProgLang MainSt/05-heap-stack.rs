fn main() {
    // Rust gives us low-level control with high-level safety
    let data = vec![1, 4, 2, 8, 5, 7 ,3, 6];
    
    // Zero-copy operations where possible
    let slice = &data[2..6];
    println!("Working with slice: {:?}", slice);
    
    // Stack-allocated array for maximum speed
    let fast_array: [i32; 4] = [10, 20, 30, 40];
    
    // Direct memory access without bounds checking (unsafe but fast)
    let sum = calculate_sum(&fast_array);
    println!("Lighting-fast sum: {}", sum);
    
    // Heap vs Stack demonstration
    let stack_string = "Speed of light"; // Stack-allocated string literal
    let heap_string = String::from("Flexibility of heap"); // Heap-allocated
    
    println!("Stack string: {}", stack_string);
    println!("Heap string: {}", heap_string);
    
    // Smart pointers give us control over memory layout
    let boxed_value = Box::new(42);
    println!("Boxed value: {}", boxed_value);
}

fn calculate_sum(arr: &[i32; 4]) -> i32 {
    arr[0] + arr[1] + arr[2] + arr[3]
}