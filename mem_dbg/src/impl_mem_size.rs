/*
 * SPDX-FileCopyrightText: 2023 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::MemSize;

macro_rules! impl_memory_size {
    ($($ty:ty),*) => {$(
impl MemSize for $ty {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}
    )*};
}

impl_memory_size! {
   (), bool, char, f32, f64,
   u8, u16, u32, u64, u128, usize,
   i8, i16, i32, i64, i128, isize
}

impl MemSize for &'_ str {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

impl<T: MemSize> MemSize for &'_ [T] {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

impl<T: MemSize> MemSize for Option<T> {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>() - core::mem::size_of::<T>() // this is so we consider the 
            + self.as_ref().map_or(0, |x| x.mem_size())
    }
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
impl<T: MemSize> MemSize for Vec<T> {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>() + self.iter().map(|x| x.mem_size()).sum::<usize>()
    }
    fn mem_capacity(&self) -> usize {
        core::mem::size_of::<Self>()
            + self.iter().map(|x| x.mem_size()).sum::<usize>()
            + (self.capacity() - self.len()) * core::mem::size_of::<T>()
    }
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
impl<T: MemSize> MemSize for Box<[T]> {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>() + self.iter().map(|x| x.mem_size()).sum::<usize>()
    }
}

#[cfg(feature = "mmap_rs")]
impl MemSize for mmap_rs::Mmap {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

#[cfg(feature = "mmap_rs")]
impl MemSize for mmap_rs::MmapMut {
    #[inline(always)]
    fn mem_size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}
