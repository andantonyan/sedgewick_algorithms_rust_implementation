pub fn gcd(p: i32, q: i32) -> i32 {
  if q == 0 { p } else {
    gcd(q, p % q)
  }
}
