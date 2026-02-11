pub struct PeekableN<I, T, const N: usize>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    iter: I,
    head: usize,
    buf: [Option<T>; N],
}

impl<I, T, const N: usize> PeekableN<I, T, N>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    #[inline(always)]
    pub fn new(mut iter: I) -> Self {
        const { assert!(N > 0, "N must be greater than 0") };
        let buf = std::array::from_fn(|_| iter.next());
        Self { iter, head: 0, buf }
    }

    #[inline(always)]
    pub fn peek_n<const M: usize>(&self) -> Option<&T> {
        const { assert!(M < N, "M must be less than N") };
        let index = self.wrap_index(M);
        unsafe { self.buf.get_unchecked(index) }.as_ref()
    }

    #[inline(always)]
    pub fn peek_dyn(&self, m: usize) -> Option<&T> {
        let index = self.wrap_index(m);
        self.buf[index].as_ref()
    }

    #[inline(always)]
    pub unsafe fn peek_dyn_unchecked(&self, m: usize) -> Option<&T> {
        let index = self.wrap_index(m);
        self.buf.get_unchecked(index).as_ref()
    }

    #[inline(always)]
    pub fn peek_safe(&self, m: usize) -> Option<Option<&T>> {
        let index = self.wrap_index(m);
        self.buf.get(index).map(Option::as_ref)
    }

    #[inline(always)]
    fn wrap_index(&self, m: usize) -> usize {
        if N.is_power_of_two() {
            (self.head + m) & (N - 1)
        } else {
            (self.head + m) % N
        }
    }
}

impl<I, T, const N: usize> Iterator for PeekableN<I, T, N>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.buf[self.head];
        self.buf[self.head] = self.iter.next();
        self.head = self.wrap_index(1);
        item
    }
}
