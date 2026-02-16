use crate::compiler::arena::{Arena, ArenaBox};

#[test]
fn test_arena_basic_alloc() {
    let arena = Arena::new();
    let val = arena.alloc(42);
    assert_eq!(*val, 42);
}

#[test]
fn test_arena_alloc_slice() {
    let arena = Arena::new();
    let data = [1, 2, 3, 4, 5];
    let mut iter = arena.alloc_slice(data);

    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_arena_alloc_iter() {
    let arena = Arena::new();
    let data = vec![10, 20, 30];
    let mut iter = arena.alloc_iter(data.into_iter());

    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(20));
    assert_eq!(iter.next(), Some(30));
    assert_eq!(iter.next(), None);
}

#[test]
#[cfg(not(miri))]
fn test_arena_large_allocation() {
    // Test allocation that exceeds a single block size
    let arena = Arena::new();
    let count = 1000_000; // Should be enough to trigger multiple blocks
    let mut iter = arena.alloc_iter(0..count);
    panic!("{}", Arena::BLOCK_SIZE);
    for i in 0..count {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
#[cfg(miri)]
fn test_arena_large_allocation() {
    // Test allocation that exceeds a single block size
    let arena = Arena::new();
    let count = 1000; // Should be enough to trigger multiple blocks
    let mut iter = arena.alloc_iter(0..count);

    for i in 0..count {
        assert_eq!(iter.next(), Some(i));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_mixed_allocations() {
    let arena = Arena::new();
    let a = arena.alloc(1u8);
    let b = arena.alloc(100u64);
    let c = arena.alloc(200u32);
    let d = arena.alloc(true);
    let e: ArenaBox<[u128; _]> = arena.alloc([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);

    assert_eq!(*a, 1);
    assert_eq!(*b, 100);
    assert_eq!(*c, 200);
    assert_eq!(*d, true);
    assert_eq!(*e, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
}
