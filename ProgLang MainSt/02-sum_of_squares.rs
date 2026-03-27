fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // High-level functional Style
    let sum_of_squares: i32 = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
        
    println!("Sum of squares of even numbers: {}", sum_of_squares);
    
    // This compiles to the same optimized code as a manual loop!
    let mut manual_sum = 0;
    for &num in &numbers {
        if num % 2 == 0 {
            manual_sum += num * num;
        }
    }
    println!("Manual calculation result: {}", manual_sum);
    println!("Both methods are equally fast!");
}