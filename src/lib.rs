pub fn gcd(p: i32, q: i32) -> i32 {
  if q == 0 { p } else {
    gcd(q, p % q)
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_test() {
      assert_eq!(3, gcd(6, 9));
    }
}
