//! ArenaAlocator implement
//!
//! # Unsafe(**Important**)
//! The contents of `ArenaBox` or `ArenaIter` will live as long as `Arena`.
//! Be sure that `ArenaBox` and `ArenaIter` drop before `Arena` drop.

use core::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    ops::Deref,
    ptr::NonNull,
};
use std::alloc::{alloc, Layout};

static ALIGNMENT: usize = 64;

pub struct Arena {
    index: Cell<usize>,
    page_index: Cell<usize>,
    ptrs: Box<UnsafeCell<Vec<*mut u8>>>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ArenaBox<T: Copy> {
    ptr: *mut T,
}

impl<T: Copy> Deref for ArenaBox<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ArenaIter<T: Copy> {
    page: usize,
    index: usize,
    len: usize,
    size: usize,
    pages_list_ptr: *const UnsafeCell<Vec<*mut u8>>,
    _marker: PhantomData<T>,
}

impl<T> Iterator for ArenaIter<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            if self.size == 0 {
                return Some(unsafe { *NonNull::<T>::dangling().as_ptr() });
            }
            if self.index + self.size > Arena::BLOCK_SIZE {
                self.page += 1;
                self.index = 0;
            }
            let ptr = unsafe {
                let vec_ptr = (*self.pages_list_ptr).get();
                let page_ptr = *((*vec_ptr).as_ptr().add(self.page));
                page_ptr.add(self.index) as *mut T
            };
            self.index += self.size;
            Some(unsafe { *ptr })
        }
    }
}

impl Arena {
    #[cfg(not(miri))]
    pub const BLOCK_SIZE: usize = 128 * 1024 * 8; // 128KB

    #[cfg(miri)]
    pub const BLOCK_SIZE: usize = 128; // 128 byte in test

    pub fn new() -> Self {
        let layout = unsafe { Layout::from_size_align_unchecked(Self::BLOCK_SIZE, ALIGNMENT) };
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            panic!("Out of memory in arena alocator");
        }
        Self {
            index: Cell::new(0),
            page_index: Cell::new(0),
            ptrs: Box::new(UnsafeCell::new(vec![ptr])),
        }
    }

    pub fn alloc<T>(&self, value: T) -> ArenaBox<T>
    where
        T: Copy,
    {
        let layout = Layout::new::<T>();

        const {
            assert!(
                std::mem::size_of::<T>() <= Self::BLOCK_SIZE,
                "Type T is too large for this arena"
            );
            assert!(std::mem::align_of::<T>() <= 64, "Type T is not aligned");
        }
        if std::mem::size_of::<T>() == 0 {
            return ArenaBox {
                ptr: NonNull::dangling().as_ptr(),
            };
        }
        let size = layout.size();
        let align = layout.align();
        let mut start = (self.index.get() + align - 1) & !(align - 1); // alignment
        if size + start > Self::BLOCK_SIZE {
            self.grow();
            start = 0;
        }
        let ptr = unsafe { *((*(*self.ptrs).get()).as_ptr().add(self.page_index.get())).add(start) as *mut T };
        self.index.set(start + size);
        unsafe {
            ptr.write(value);
        }
        ArenaBox { ptr }
    }

    pub fn alloc_iter<T, I>(&self, value: I) -> ArenaIter<T>
    where
        T: Copy,
        I: Iterator<Item = T>,
    {
        const {
            assert!(
                std::mem::size_of::<T>() <= Self::BLOCK_SIZE,
                "Type T is too large for this arena"
            );
            assert!(std::mem::align_of::<T>() <= 64, "Type T is not aligned");
        };
        let each_size = std::mem::size_of::<T>();
        if each_size == 0 {
            let counter = value.count();
            return ArenaIter {
                index: 0,
                page: 0,
                len: counter,
                pages_list_ptr: &*self.ptrs,
                size: 0,
                _marker: PhantomData,
            };
        }
        let each_layout = Layout::new::<T>();
        let align = each_layout.align();
        let size = each_layout.size();
        let start = (self.index.get() + align - 1) & !(align - 1);
        self.index.set(start);
        let start_page = self.page_index.get();

        let mut counter = 0;
        let mut page_ptr = unsafe {*((*(*self.ptrs).get()).as_ptr().add(self.page_index.get()))};

        for value in value {
            if self.index.get() + size <= Self::BLOCK_SIZE {
                let ptr: *mut T = unsafe { page_ptr.add(self.index.get()) as *mut T };
                self.index.set(self.index.get() + size);
                unsafe {
                    ptr.write(value);
                }
            } else {
                self.grow();
                page_ptr = unsafe {*((*(*self.ptrs).get()).as_ptr().add(self.page_index.get()))};
                let ptr: *mut T = unsafe { page_ptr.add(self.index.get()) as *mut T };
                self.index.set(size);
                unsafe { ptr.write(value) }
            }
            counter += 1;
        }

        ArenaIter {
            index: start,
            page: start_page,
            len: counter,
            pages_list_ptr: &*self.ptrs,
            size,
            _marker: PhantomData,
        }
    }

    pub fn alloc_with<T: Copy>(&self, f: impl FnMut() -> Option<T>) -> ArenaIter<T> {
        let iter = std::iter::from_fn(f);
        self.alloc_iter(iter)
    }

    pub fn alloc_slice<T: Copy, const N: usize>(&self, slice: [T; N]) -> ArenaIter<T> {
        self.alloc_iter(slice.into_iter())
    }

    fn grow(&self) {
        // lack of memory
        let new_block_ptr = unsafe {
            alloc(Layout::from_size_align_unchecked(
                Self::BLOCK_SIZE,
                ALIGNMENT,
            ))
        };
        unsafe { (*self.ptrs.get()).push(new_block_ptr) };
        self.page_index.set(self.page_index.get() + 1);
        self.index.set(0);
    }

}

impl Drop for Arena {
    fn drop(&mut self) {
        for ptr in unsafe { (*self.ptrs.get()).iter() } {
            unsafe {
                let layout = Layout::from_size_align_unchecked(Self::BLOCK_SIZE, ALIGNMENT);
                std::alloc::dealloc(*ptr, layout);
            }
        }
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + std::hash::Hash> std::hash::Hash for ArenaIter<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for value in *self {
            value.hash(state);
        }
    }
}

impl<T: Copy + std::hash::Hash> std::hash::Hash for ArenaBox<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}
