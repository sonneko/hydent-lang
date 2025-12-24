//! # Arena Allocator
//!
//! This module provides an arena allocator for the Hydent compiler. Arena allocation
//! is a memory management technique where memory for a group of objects is
//! allocated from a single large block of memory. This can be more efficient
//! than allocating each object individually, especially for a large number of
//! small objects.
//!
//! The `Arena` struct is the main entry point to the arena allocator. It
//! provides methods for allocating objects and vectors within the arena.

use std::{
    alloc::{Layout, alloc}, cell::{Cell, RefCell}, marker::PhantomData, ops::Deref, ptr::NonNull
};

static ALIGNMENT: usize = 64;

/// A memory arena for allocating objects.
pub struct Arena {
    index: Cell<usize>,
    page_index: Cell<usize>,
    ptrs: RefCell<Vec<*mut u8>>,
}

/// A smart pointer for an object allocated in an `Arena`.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ArenaBox<T: Copy> {
    ptr: *mut T,
    _marker: PhantomData<T>,
}

impl<T: Copy> Deref for ArenaBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T: Copy> Deref for ArenaVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

/// A list of objects allocated in an `Arena`.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ArenaVec<T: Copy> {
    ptr: *mut T,
    len: usize,
    _marker: PhantomData<T>,
}

impl Arena {
    pub const BLOCK_SIZE: usize = 32 * 1024; // 32KB

    pub fn new() -> Self {
        let layout = unsafe { Layout::from_size_align_unchecked(Self::BLOCK_SIZE, ALIGNMENT) };
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            panic!("Out of memory");
        }
        Self {
            index: Cell::new(0),
            page_index: Cell::new(0),
            ptrs: RefCell::new(vec![ptr]),
        }
    }

    pub fn alloc<T>(&self, value: T) -> ArenaBox<T>
    where
        T: Copy,
    {
        let layout = Layout::new::<T>();

        assert!(
            std::mem::size_of::<T>() <= Self::BLOCK_SIZE,
            "Type T is too large for this arena"
        );
        if std::mem::size_of::<T>() == 0 {
            return ArenaBox {
                ptr: NonNull::dangling().as_ptr(),
                _marker: PhantomData,
            };
        }
        assert!(std::mem::align_of::<T>() <= 64, "Type T is not aligned");
        let size = layout.size();
        let align = layout.align();
        // indexをalignの倍数まで引き上げ
        let mut start = (self.index.get() + align - 1) & !(align - 1);
        if size + start > Self::BLOCK_SIZE {
            self.grow();
            start = 0;
        }
        let ptr = unsafe { self.ptrs.borrow()[self.page_index.get()].add(start) } as *mut T;
        self.index.set(start + size);
        unsafe {
            ptr.write(value);
        }
        ArenaBox {
            ptr: ptr as *mut T,
            _marker: PhantomData,
        }
    }

    pub fn alloc_slice<T>(&self, value: &[T]) -> ArenaVec<T>
    where
        T: Copy,
    {
        let size = std::mem::size_of::<T>() * value.len();
        assert!(
            size <= Self::BLOCK_SIZE,
            "Type T is too large for this arena"
        );
        assert!(std::mem::align_of::<T>() <= 64, "Type T is not aligned");

        if std::mem::size_of::<T>() == 0 {
            return ArenaVec {
                ptr: NonNull::dangling().as_ptr(),
                len: value.len(),
                _marker: PhantomData,
            };
        }
        let layout = Layout::from_size_align(size, std::mem::align_of::<T>()).unwrap();
        let align = layout.align();
        // indexをalignの倍数まで引き上げ
        let mut start = (self.index.get() + align - 1) & !(align - 1);
        if size + start > Self::BLOCK_SIZE {
            self.grow();
            start = 0;
        }
        let ptr = unsafe { self.ptrs.borrow()[self.page_index.get()].add(start) } as *mut T;
        self.index.set(start + size);
        unsafe {
            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
        }
        ArenaVec {
            ptr: ptr as *mut T,
            len: value.len(),
            _marker: PhantomData,
        }
    }

    fn grow(&self) {
        // lack of memory
        let new_block_ptr = unsafe {
            alloc(Layout::from_size_align_unchecked(
                Self::BLOCK_SIZE,
                ALIGNMENT,
            ))
        };
        self.ptrs.borrow_mut().push(new_block_ptr);
        self.page_index.set(self.page_index.get() + 1);
        self.index.set(0);
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        for ptr in self.ptrs.borrow().iter() {
            unsafe {
                let layout = Layout::from_size_align_unchecked(Self::BLOCK_SIZE, ALIGNMENT);
                std::alloc::dealloc(*ptr, layout);
            }
        }
    }
}
