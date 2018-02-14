fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let divs: Vec<u32> = (2..integer)
        .filter(|k| integer % k == 0)
        .collect();
      
    if divs.is_empty() { return Err(format!("{} is prime", integer)); }
    
    Ok(divs)
}
