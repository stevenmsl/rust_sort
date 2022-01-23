use rust_sort::*;
fn main() {
    let mut intervals = TestFixtures::test_fixture_1();

    println!("{:?}", intervals[0].start.cmp(&intervals[1].start));

    intervals.sort_by(|int_1, int_2| int_1.start.cmp(&int_2.start));

    println!("{:?}", intervals);
}
