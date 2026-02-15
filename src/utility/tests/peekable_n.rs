use crate::utility::peekable_n::PeekableN;

#[test]
fn test_peekable_n_basic() {
    let data = [1, 2, 3, 4, 5];
    let mut iter = PeekableN::<_, _, 2>::new(data.into_iter());

    assert_eq!(iter.peek_n::<0>(), Some(&1));
    assert_eq!(iter.peek_n::<1>(), Some(&2));

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.peek_n::<0>(), Some(&2));
    assert_eq!(iter.peek_n::<1>(), Some(&3));

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.peek_n::<0>(), Some(&3));
    assert_eq!(iter.peek_n::<1>(), Some(&4));

    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.peek_n::<0>(), Some(&4));
    assert_eq!(iter.peek_n::<1>(), Some(&5));

    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.peek_n::<0>(), Some(&5));
    assert_eq!(iter.peek_n::<1>(), None);

    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.peek_n::<0>(), None);
    assert_eq!(iter.peek_n::<1>(), None);

    assert_eq!(iter.next(), None);
}

#[test]
fn test_peek_dyn() {
    let data = [10, 20, 30];
    let iter = PeekableN::<_, _, 3>::new(data.into_iter());

    assert_eq!(iter.peek_dyn(0), Some(&10));
    assert_eq!(iter.peek_dyn(1), Some(&20));
    assert_eq!(iter.peek_dyn(2), Some(&30));
}

#[test]
fn test_circular_wrap() {
    let data = [1, 2, 3, 4, 5, 6];
    let mut iter = PeekableN::<_, _, 4>::new(data.into_iter());

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    
    // buffer content shifted.
    assert_eq!(iter.peek_n::<0>(), Some(&4));
    assert_eq!(iter.peek_n::<3>(), None);

    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.peek_n::<0>(), Some(&5));
}
