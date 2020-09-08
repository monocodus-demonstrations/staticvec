use core::cmp::{Ordering, PartialOrd};
use core::intrinsics;
use core::mem::MaybeUninit;

use crate::StaticVec;

/// An internal function for calculating pointer offsets as usizes, while accounting
/// directly for possible ZSTs. This is used specifically in the iterator implementations.
#[inline(always)]
pub(crate) const fn distance_between<T>(dest: *const T, origin: *const T) -> usize {
  match intrinsics::size_of::<T>() {
    0 => unsafe { (dest as usize).wrapping_sub(origin as usize) },
    // Safety: this function is used strictly with linear inputs
    // where dest is known to come after origin.
    _ => unsafe { intrinsics::ptr_offset_from(dest, origin) as usize },
  }
}

/// A simple reversal function that returns a new array, called in
/// [`StaticVec::reversed`](crate::StaticVec::reversed).
#[inline]
pub(crate) fn reverse_copy<T, const N: usize>(
  length: usize,
  this: &MaybeUninit<[T; N]>,
) -> MaybeUninit<[T; N]>
where
  T: Copy,
{
  let mut i = length;
  let src = StaticVec::first_ptr(this);
  let mut res = StaticVec::new_data_uninit();
  let mut dest = StaticVec::first_ptr_mut(&mut res);
  // We know these are valid pointers based on how and where this
  // function is called from, so these are safe hints to give to the
  // optimizer.
  unsafe {
    intrinsics::assume(!src.is_null());
    // Curiously, the explicit typecast to `*mut T` on the next line
    // is necessary to get it to compile. Without the typecast, `rustc` can't figure out
    // what the type is supposed to be for some reason.
    intrinsics::assume(!(dest as *mut T).is_null());
  }
  while i > 0 {
    unsafe {
      src.add(i - 1).copy_to_nonoverlapping(dest, 1);
      dest = dest.offset(1);
      i -= 1;
    }
  }
  res
}

/// Previously this was what one of the forms of the [`staticvec!`] macro used internally. Currently
/// it's not used at all, and may be removed if I don't think of another use for it in the next
/// little while.
#[inline(always)]
pub fn new_from_value<T, const COUNT: usize>(value: T) -> StaticVec<T, COUNT>
where T: Copy {
  StaticVec {
    data: {
      unsafe {
        let mut data = StaticVec::new_data_uninit();
        for i in 0..COUNT {
          // Can't use `first_ptr_mut` here as the type inference doesn't work
          // in this context for some reason.
          (data.as_mut_ptr() as *mut T).add(i).write(value);
        }
        data
      }
    },
    length: COUNT,
  }
}

/// A version of the default `partial_cmp` implementation with a more flexible function signature.
#[inline]
pub(crate) fn partial_compare<T1, T2: PartialOrd<T1>>(
  this: &[T2],
  other: &[T1],
) -> Option<Ordering>
{
  let min_length = this.len().min(other.len());
  unsafe {
    let left = this.get_unchecked(0..min_length);
    let right = other.get_unchecked(0..min_length);
    for i in 0..min_length {
      match left.get_unchecked(i).partial_cmp(right.get_unchecked(i)) {
        Some(Ordering::Equal) => (),
        non_eq => return non_eq,
      }
    }
  }
  this.len().partial_cmp(&other.len())
}

/// A simple quicksort function for internal use, called in
/// ['quicksorted_unstable`](crate::StaticVec::quicksorted_unstable).
#[inline]
pub(crate) fn quicksort_internal<T: Copy + PartialOrd>(
  values: *mut T,
  mut low: isize,
  mut high: isize,
)
{
  // We call this function from exactly one place where `low` and `high` are known to be within an
  // appropriate range before getting passed into it, so there's no need to check them again here.
  // We also know that `values` will never be null, so we can safely give an optimizer hint here.
  unsafe { intrinsics::assume(!values.is_null()) };
  loop {
    let mut i = low;
    let mut j = high;
    unsafe {
      let p = *values.offset(low + ((high - low) >> 1));
      loop {
        while *values.offset(i) < p {
          i += 1;
        }
        while *values.offset(j) > p {
          j -= 1;
        }
        if i <= j {
          if i != j {
            let q = *values.offset(i);
            *values.offset(i) = *values.offset(j);
            *values.offset(j) = q;
          }
          i += 1;
          j -= 1;
        }
        if i > j {
          break;
        }
      }
    }
    if j - low < high - i {
      if low < j {
        quicksort_internal(values, low, j);
      }
      low = i;
    } else {
      if i < high {
        quicksort_internal(values, i, high)
      }
      high = j;
    }
    if low >= high {
      break;
    }
  }
}

/// Copied locally from `core/ptr/mod.rs` so we can use it in `const fn` versions of the slice
/// creation methods.
#[repr(C)]
pub(crate) struct FatPtr<T> {
  data: *const T,
  pub(crate) len: usize,
}

/// Copied locally from `core/ptr/mod.rs` so we can use it in `const fn` versions of the slice
/// creation methods.
#[repr(C)]
pub(crate) union Repr<T> {
  pub(crate) rust: *const [T],
  rust_mut: *mut [T],
  pub(crate) raw: FatPtr<T>,
}

/// A local `const fn` version of `ptr::slice_from_raw_parts`.
#[inline(always)]
pub(crate) const fn ptr_slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T] {
  debug_assert!(
    core::mem::size_of::<T>().saturating_mul(len) <= isize::MAX as usize,
    "Attempted to create a slice covering at least half the address space!"
  );
  unsafe {
    Repr {
      raw: FatPtr { data, len },
    }
    .rust
  }
}

/// A local `const fn` version of `ptr::slice_from_raw_parts_mut`.
#[inline(always)]
pub(crate) const fn ptr_slice_from_raw_parts_mut<T>(data: *mut T, len: usize) -> *mut [T] {
  debug_assert!(
    core::mem::size_of::<T>().saturating_mul(len) <= isize::MAX as usize,
    "Attempted to create a slice covering at least half the address space!"
  );
  unsafe {
    Repr {
      raw: FatPtr { data, len },
    }
    .rust_mut
  }
}

/// A local `const fn` version of `slice::from_raw_parts`.
#[inline(always)]
pub(crate) const fn slice_from_raw_parts<'a, T>(data: *const T, length: usize) -> &'a [T] {
  // Same code as in the original, except we do the debug assertions in the function that this one
  // calls, rather than the other way around.
  unsafe { &*ptr_slice_from_raw_parts(data, length) }
}

/// A local `const fn` version of `slice::from_raw_parts_mut`.
#[inline(always)]
pub(crate) const fn slice_from_raw_parts_mut<'a, T>(data: *mut T, length: usize) -> &'a mut [T] {
  // Same code as in the original, except we do the debug assertions in the function that this one
  // calls, rather than the other way around.
  unsafe { &mut *ptr_slice_from_raw_parts_mut(data, length) }
}
