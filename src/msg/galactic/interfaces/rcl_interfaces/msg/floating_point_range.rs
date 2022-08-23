// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn rcl_interfaces__msg__FloatingPointRange__init(msg: *mut FloatingPointRange) -> bool;
    fn rcl_interfaces__msg__FloatingPointRange__fini(msg: *mut FloatingPointRange);
    fn rcl_interfaces__msg__FloatingPointRange__are_equal(
        lhs: *const FloatingPointRange,
        rhs: *const FloatingPointRange,
    ) -> bool;
    fn rcl_interfaces__msg__FloatingPointRange__Sequence__init(
        msg: *mut FloatingPointRangeSeqRaw,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__FloatingPointRange__Sequence__fini(msg: *mut FloatingPointRangeSeqRaw);
    fn rcl_interfaces__msg__FloatingPointRange__Sequence__are_equal(
        lhs: *const FloatingPointRangeSeqRaw,
        rhs: *const FloatingPointRangeSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__FloatingPointRange(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct FloatingPointRange {
    pub from_value: f64,
    pub to_value: f64,
    pub step: f64,
}

impl FloatingPointRange {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__FloatingPointRange__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for FloatingPointRange {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__msg__FloatingPointRange__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct FloatingPointRangeSeqRaw {
    data: *mut FloatingPointRange,
    size: usize,
    capacity: usize,
}

/// Sequence of FloatingPointRange.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct FloatingPointRangeSeq<const N: usize> {
    data: *mut FloatingPointRange,
    size: usize,
    capacity: usize,
}

impl<const N: usize> FloatingPointRangeSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: FloatingPointRangeSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__FloatingPointRange__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: FloatingPointRangeSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[FloatingPointRange] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [FloatingPointRange] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, FloatingPointRange> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, FloatingPointRange> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for FloatingPointRangeSeq<N> {
    fn drop(&mut self) {
        let mut msg = FloatingPointRangeSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { rcl_interfaces__msg__FloatingPointRange__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for FloatingPointRangeSeq<N> {}
unsafe impl<const N: usize> Sync for FloatingPointRangeSeq<N> {}

impl TopicMsg for FloatingPointRange {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__FloatingPointRange()
        }
    }
}

impl PartialEq for FloatingPointRange {
    fn eq(&self, other: &Self) -> bool {
        unsafe { rcl_interfaces__msg__FloatingPointRange__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for FloatingPointRangeSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = FloatingPointRangeSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = FloatingPointRangeSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            rcl_interfaces__msg__FloatingPointRange__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
