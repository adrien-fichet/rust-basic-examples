// https://doc.rust-lang.org/std/iter/trait.Iterator.html
// todo: itertools usage

fn next_count_last() {
    let mut iter = 1..=3;

    assert_eq!(iter.size_hint(), (3, Some(3))); // lower/upper bounds on the remaining length of the iterator
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.count(), 0);

    assert_eq!((1..=3).count(), 3); // consumes the iterator
    assert_eq!((1..=3).last(), Some(3));
}

fn nth() {
    let mut iter = 1..=3;
    assert_eq!(iter.nth(1), Some(2)); // consumes the preceding elements, so multiple nth() calls will return different elements
    assert_eq!(iter.nth(1), None);
}

fn infinite_iterator() {
    let infinite_iter = 0..;
    assert_eq!(infinite_iter.size_hint(), (usize::MAX, None)); // maximum possible lower bound, no upper bound

    // infinite_iter.count(); ---> will panic because the iterator has more than usize::MAX elements
}

fn method_chaining() {
    let iter = (1..=10)
        .skip(1)
        .step_by(2)
        .skip(2)
        .chain(12..=14)
        .map(|e| e + 1)
        .filter(|e| e % 2 == 1)
        .skip_while(|e| *e < 10)
        .take_while(|e| *e < 15)
        .take(2);
    assert_eq!(iter.collect::<Vec<i32>>(), [11, 13]);
}

fn zipping() {
    let numbers = [1, 2, 3];
    let food = ["Apple", "Banana", "Carrot"];
    numbers
        .iter()
        .zip(food.iter())
        .enumerate()
        .for_each(|(i, e)| assert_eq!(e, (&numbers[i], &food[i])));
}

fn peeking() {
    let mut peekable_iter = [1, 2, 3].iter().peekable();
    assert_eq!(peekable_iter.peek(), Some(&&1));
    assert_eq!(peekable_iter.next(), Some(&1));
    let two_to_four = peekable_iter.peek_mut();
    *two_to_four.unwrap() = &4;
    assert_eq!(peekable_iter.next(), Some(&4));
}

fn main() {
    next_count_last();
    nth();
    infinite_iterator();
    method_chaining();
    zipping();
    peeking();
}
