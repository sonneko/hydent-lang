//! ArenaAlocator implement with Nested Iterator Allocation support

use core::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    ptr::NonNull,
};
use std::alloc::{alloc, Layout};

static ALIGNMENT: usize = 64;

#[derive(Debug, Clone, Copy)]
struct AllocState {
    item_size: usize,
    align: usize,
    zst_count: u32,
    buffer_start_offset: usize, // iter_buffer内でのこの階層の開始位置
}

pub struct Arena {
    index: Cell<usize>,
    page_index: Cell<usize>,
    ptrs: Box<UnsafeCell<Vec<*mut u8>>>,
    // ネストを管理するスタック
    states: UnsafeCell<Vec<AllocState>>,
    // すべてのネストで共有する作業用バッファ
    iter_buffer: UnsafeCell<Vec<u8>>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ArenaBox<T: Copy> {
    id: u32,
    _marker: PhantomData<T>,
}

impl<T: Copy> ArenaBox<T> {
    #[inline]
    pub fn get<'a>(&self, arena: &'a Arena) -> &'a T {
        if std::mem::size_of::<T>() == 0 {
            return unsafe { &*NonNull::dangling().as_ptr() };
        }
        let page = (self.id as usize) / Arena::BLOCK_SIZE;
        let offset = (self.id as usize) % Arena::BLOCK_SIZE;
        unsafe {
            let pages = &*arena.ptrs.get();
            let page_ptr = *(*pages).as_ptr().add(page);
            &*(page_ptr.add(offset) as *const T)
        }
    }

    #[inline]
    pub fn get_mut<'a>(&mut self, arena: &'a mut Arena) -> &'a mut T {
        if std::mem::size_of::<T>() == 0 {
            return unsafe { &mut *NonNull::dangling().as_ptr() };
        }
        let page = (self.id as usize) / Arena::BLOCK_SIZE;
        let offset = (self.id as usize) % Arena::BLOCK_SIZE;
        unsafe {
            let pages = &*arena.ptrs.get();
            let page_ptr = *(*pages).as_ptr().add(page);
            &mut *(page_ptr.add(offset) as *mut T)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ArenaIter<T: Copy> {
    start_id: u32,
    len: u32,
    _marker: PhantomData<T>,
}

impl<T: Copy> ArenaIter<T> {
    pub fn into_ref<'a>(&self, arena: &'a Arena) -> ArenaIterReader<'a, T> {
        ArenaIterReader {
            arena,
            current_id: self.start_id,
            len: self.len,
            _marker: PhantomData,
        }
    }
}

pub struct ArenaIterReader<'a, T: Copy> {
    arena: &'a Arena,
    current_id: u32,
    len: u32,
    _marker: PhantomData<T>,
}

impl<'a, T: Copy + 'static> std::iter::Iterator for ArenaIterReader<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let size = std::mem::size_of::<T>();
        if size == 0 {
            return Some(unsafe { &mut *NonNull::<T>::dangling().as_ptr() });
        }

        let mut page = (self.current_id as usize) / Arena::BLOCK_SIZE;
        let mut offset = (self.current_id as usize) % Arena::BLOCK_SIZE;

        if offset + size > Arena::BLOCK_SIZE {
            page += 1;
            offset = 0;
            self.current_id = (page * Arena::BLOCK_SIZE) as u32;
        }

        let ptr = unsafe {
            let pages = &*self.arena.ptrs.get();
            let page_base = *pages.as_ptr().add(page);
            page_base.add(offset) as *mut T
        };

        self.current_id += size as u32;
        Some(unsafe { &mut *ptr })
    }
}

impl Arena {
    pub const BLOCK_SIZE: usize = 1024 * 1024 * 8;

    pub fn new() -> Self {
        let layout = unsafe { Layout::from_size_align_unchecked(Self::BLOCK_SIZE, ALIGNMENT) };
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            panic!("OOM");
        }
        Self {
            index: Cell::new(0),
            page_index: Cell::new(0),
            ptrs: Box::new(UnsafeCell::new(vec![ptr])),
            states: UnsafeCell::new(Vec::with_capacity(4)),
            iter_buffer: UnsafeCell::new(Vec::with_capacity(1024)),
        }
    }

    pub fn alloc<T: Copy>(&self, value: T) -> ArenaBox<T> {
        let size = std::mem::size_of::<T>();
        if size == 0 {
            return ArenaBox {
                id: u32::MAX,
                _marker: PhantomData,
            };
        }

        let align = std::mem::align_of::<T>();
        let mut start = (self.index.get() + align - 1) & !(align - 1);

        if size + start > Self::BLOCK_SIZE {
            self.grow();
            start = 0;
        }

        let id = (self.page_index.get() * Self::BLOCK_SIZE + start) as u32;
        unsafe {
            let pages = &*self.ptrs.get();
            let page_ptr = *pages.as_ptr().add(self.page_index.get());
            let ptr = page_ptr.add(start) as *mut T;
            ptr.write(value);
        }
        self.index.set(start + size);
        ArenaBox {
            id,
            _marker: PhantomData,
        }
    }

    // --- Nested Iterator Allocation API ---

    pub fn start_iter_allocation<T: Copy>(&self) {
        let states = unsafe { &mut *self.states.get() };
        let buffer = unsafe { &*self.iter_buffer.get() };

        states.push(AllocState {
            item_size: std::mem::size_of::<T>(),
            align: std::mem::align_of::<T>(),
            zst_count: 0,
            buffer_start_offset: buffer.len(),
        });
    }

    pub fn alloc_iter_item<T: Copy>(&self, value: &T) {
        let states = unsafe { &mut *self.states.get() };
        let current = states.last_mut().expect("No active iteration");

        if current.item_size == 0 {
            current.zst_count += 1;
        } else {
            let buffer = unsafe { &mut *self.iter_buffer.get() };
            let ptr = value as *const T as *const u8;
            let slice = unsafe { std::slice::from_raw_parts(ptr, current.item_size) };
            buffer.extend_from_slice(slice);
        }
    }

    pub fn finish_iter_allocation<T: Copy>(&self) -> ArenaIter<T> {
        let states = unsafe { &mut *self.states.get() };
        let state = states.pop().expect("No active iteration");
        let buffer = unsafe { &mut *self.iter_buffer.get() };

        if state.item_size == 0 {
            return ArenaIter {
                start_id: u32::MAX,
                len: state.zst_count,
                _marker: PhantomData,
            };
        }

        let data_len = buffer.len() - state.buffer_start_offset;
        let count = (data_len / state.item_size) as u32;

        if count == 0 {
            return ArenaIter {
                start_id: u32::MAX,
                len: 0,
                _marker: PhantomData,
            };
        }

        // Arenaへの書き込み
        let mut start_pos = (self.index.get() + state.align - 1) & !(state.align - 1);
        if start_pos + state.item_size > Self::BLOCK_SIZE {
            self.grow();
            start_pos = 0;
        }
        let start_id = (self.page_index.get() * Self::BLOCK_SIZE + start_pos) as u32;

        for i in 0..count {
            if self.index.get() + state.item_size > Self::BLOCK_SIZE {
                self.grow();
            }
            unsafe {
                let pages = &*self.ptrs.get();
                let dest = (*pages.as_ptr().add(self.page_index.get())).add(self.index.get());
                let src = buffer
                    .as_ptr()
                    .add(state.buffer_start_offset + (i as usize * state.item_size));
                dest.copy_from_nonoverlapping(src, state.item_size);
            }
            self.index.set(self.index.get() + state.item_size);
        }

        // バッファをこの階層の開始位置まで巻き戻して再利用可能にする
        buffer.truncate(state.buffer_start_offset);

        ArenaIter {
            start_id,
            len: count,
            _marker: PhantomData,
        }
    }

    pub fn alloc_iter<T, I>(&self, value: I) -> ArenaIter<T>
    where
        T: Copy,
        I: Iterator<Item = T>,
    {
        self.start_iter_allocation::<T>();
        for item in value {
            self.alloc_iter_item(&item);
        }
        self.finish_iter_allocation::<T>()
    }

    pub fn alloc_with<T: Copy>(&self, mut f: impl FnMut() -> Option<T>) -> ArenaIter<T> {
        self.start_iter_allocation::<T>();
        while let Some(item) = f() {
            self.alloc_iter_item(&item);
        }
        self.finish_iter_allocation::<T>()
    }

    fn grow(&self) {
        let layout = unsafe { Layout::from_size_align_unchecked(Self::BLOCK_SIZE, ALIGNMENT) };
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            panic!("OOM");
        }
        unsafe { (*self.ptrs.get()).push(ptr) };
        self.page_index.set(self.page_index.get() + 1);
        self.index.set(0);
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        for ptr in unsafe { (*self.ptrs.get()).iter() } {
            unsafe {
                let layout = Layout::from_size_align_unchecked(Arena::BLOCK_SIZE, ALIGNMENT);
                std::alloc::dealloc(*ptr, layout);
            }
        }
    }
}

impl<T: Copy + std::hash::Hash> std::hash::Hash for ArenaBox<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T: Copy + std::hash::Hash> std::hash::Hash for ArenaIter<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.start_id.hash(state);
        self.len.hash(state);
    }
}
