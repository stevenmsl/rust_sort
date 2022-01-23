/*
  - we need PartialEq so that assert_eq!
    can work
*/

#[derive(Debug, PartialEq)]
pub struct Interval {
  pub start: usize,
  pub end: usize,
}

pub struct TestFixtures {}
impl TestFixtures {
  pub fn test_fixture_1() -> Vec<Interval> {
    vec![
      Interval { start: 15, end: 20 },
      Interval { start: 0, end: 30 },
      Interval { start: 5, end: 10 },
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_stable_sort() {
    let mut intervals = TestFixtures::test_fixture_1();

    /*
       - by default it is ascending
       - the order is decided by the
         "start" alone
    */
    intervals.sort_by(|int_1, int_2| int_1.start.cmp(&int_2.start));

    let target = vec![
      Interval { start: 0, end: 30 },
      Interval { start: 5, end: 10 },
      Interval { start: 15, end: 20 },
    ];
    assert_eq!(intervals, target);
  }
}
