use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::ptr;

use crate::core::mem;

struct RawVect<T>(Box<[MaybeUninit<T>]>);

unsafe impl<T: Send> Send for RawVect<T> {}
unsafe impl<T: Sync> Sync for RawVect<T> {}

impl<T> RawVect<T> {
    fn new() -> Self {
        // !0 is usize::MAX. This branch should be stripped at compile time.
        Self(unsafe {
            mem::alloc(
                if std::mem::size_of::<T>() == 0 { !0 } else { 0 },
                mem::MemoryTag::Vector,
            )
        })
    }

    fn grow(&mut self) {
        // since we set the capacity to usize::MAX when T has size 0,
        // getting to here necessarily means the Vec is overfull.
        crate::assert_msg!(std::mem::size_of::<T>() != 0, "capacity overflow");

        // This can't overflow because we ensure self.cap <= isize::MAX.
        let new_cap = if self.0.len() == 0 {
            1
        } else {
            2 * self.0.len()
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        crate::assert_msg!(new_cap <= isize::MAX as usize, "Allocation too large");

        unsafe {
            mem::realloc(&mut self.0, new_cap, mem::MemoryTag::Vector);
        }
    }
}

impl<T> Drop for RawVect<T> {
    fn drop(&mut self) {
        let elem_size = std::mem::size_of::<T>();

        if self.0.len() != 0 && elem_size != 0 {
            unsafe {
                mem::dealloc(std::mem::take(&mut self.0), mem::MemoryTag::Vector);
            }
        }
    }
}

pub struct Vect<T> {
    buf: RawVect<T>,
    len: usize,
}

impl<T> Vect<T> {
    fn ptr(&self) -> *mut T {
        unsafe { std::mem::transmute(self.buf.0.as_ptr()) }
    }

    fn cap(&self) -> usize {
        self.buf.0.len()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn new() -> Self {
        Self {
            buf: RawVect::new(),
            len: 0,
        }
    }
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow();
        }

        self.buf.0[self.len].write(elem);

        // Can't overflow, we'll OOM first.
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(unsafe { self.buf.0[self.len].assume_init_read() })
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        crate::assert_msg!(index <= self.len, "index out of bounds");
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            self.buf.0[index].write(elem);
        }

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        crate::assert_msg!(index < self.len, "index out of bounds");

        self.len -= 1;

        unsafe {
            let result = self.buf.0[index].assume_init_read();
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            result
        }
    }

    pub fn drain(&mut self) -> Drain<T> {
        let iter = unsafe { RawValIter::new(&self) };

        // this is a mem::forget safety thing. If Drain is forgotten, we just
        // leak the whole Vec's contents. Also we need to do this *eventually*
        // anyway, so why not do it now?
        self.len = 0;

        Drain {
            iter,
            vec: PhantomData,
        }
    }
}

impl<T> Drop for Vect<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
        // deallocation is handled by RawVect
    }
}

impl<T> Deref for Vect<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::mem::transmute(&self.buf.0[0..self.len]) }
    }
}

impl<T> DerefMut for Vect<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::mem::transmute(&mut self.buf.0[0..self.len]) }
    }
}

impl<T> IntoIterator for Vect<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let (iter, buf) = unsafe { (RawValIter::new(&self), ptr::read(&self.buf)) };

        std::mem::forget(self);

        IntoIter { iter, _buf: buf }
    }
}

struct RawValIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> RawValIter<T> {
    unsafe fn new(slice: &[T]) -> Self {
        RawValIter {
            start: slice.as_ptr(),
            end: if std::mem::size_of::<T>() == 0 {
                ((slice.as_ptr() as usize) + slice.len()) as *const _
            } else if slice.len() == 0 {
                slice.as_ptr()
            } else {
                slice.as_ptr().add(slice.len())
            },
        }
    }
}

impl<T> Iterator for RawValIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if std::mem::size_of::<T>() == 0 {
                    self.start = (self.start as usize + 1) as *const _;
                    Some(MaybeUninit::<T>::uninit().assume_init_read())
                } else {
                    let old_ptr = self.start;
                    self.start = self.start.offset(1);
                    Some(ptr::read(old_ptr))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = std::mem::size_of::<T>();
        let len =
            (self.end as usize - self.start as usize) / if elem_size == 0 { 1 } else { elem_size };
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RawValIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if std::mem::size_of::<T>() == 0 {
                    self.end = (self.end as usize - 1) as *const _;
                    Some(MaybeUninit::<T>::uninit().assume_init_read())
                } else {
                    self.end = self.end.offset(-1);
                    Some(ptr::read(self.end))
                }
            }
        }
    }
}

pub struct IntoIter<T> {
    _buf: RawVect<T>, // we don't actually care about this. Just need it to live.
    iter: RawValIter<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

pub struct Drain<'a, T: 'a> {
    vec: PhantomData<&'a mut Vec<T>>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        // pre-drain the iter
        for _ in &mut *self {}
    }
}
