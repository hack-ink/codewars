fn divisors(integer: u32) -> Result<Vec<u32>, String> {
  let divs = (2..integer)
      .filter(|k| integer % k == 0)
      .collect::<Vec<u32>>();
      
  if divs.len() > 0 { Ok(divs) } else { Err(format!("{} is prime", integer)) }
}
