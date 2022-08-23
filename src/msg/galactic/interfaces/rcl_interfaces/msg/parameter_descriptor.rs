// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn rcl_interfaces__msg__ParameterDescriptor__init(msg: *mut ParameterDescriptor) -> bool;
    fn rcl_interfaces__msg__ParameterDescriptor__fini(msg: *mut ParameterDescriptor);
    fn rcl_interfaces__msg__ParameterDescriptor__are_equal(
        lhs: *const ParameterDescriptor,
        rhs: *const ParameterDescriptor,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterDescriptor__Sequence__init(
        msg: *mut ParameterDescriptorSeqRaw,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterDescriptor__Sequence__fini(
        msg: *mut ParameterDescriptorSeqRaw,
    );
    fn rcl_interfaces__msg__ParameterDescriptor__Sequence__are_equal(
        lhs: *const ParameterDescriptorSeqRaw,
        rhs: *const ParameterDescriptorSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterDescriptor(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct ParameterDescriptor {
    pub name: crate::msg::RosString<0>,
    pub type_: u8,
    pub description: crate::msg::RosString<0>,
    pub additional_constraints: crate::msg::RosString<0>,
    pub read_only: bool,
    pub dynamic_typing: bool,
    pub floating_point_range: FloatingPointRangeSeq<1>,
    pub integer_range: IntegerRangeSeq<1>,
}

impl ParameterDescriptor {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__ParameterDescriptor__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for ParameterDescriptor {
    fn drop(&mut self) {
        unsafe { rcl_interfaces__msg__ParameterDescriptor__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct ParameterDescriptorSeqRaw {
    data: *mut ParameterDescriptor,
    size: usize,
    capacity: usize,
}

/// Sequence of ParameterDescriptor.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct ParameterDescriptorSeq<const N: usize> {
    data: *mut ParameterDescriptor,
    size: usize,
    capacity: usize,
}

impl<const N: usize> ParameterDescriptorSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: ParameterDescriptorSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__init(&mut msg, size) } {
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
        let msg: ParameterDescriptorSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[ParameterDescriptor] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [ParameterDescriptor] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ParameterDescriptor> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, ParameterDescriptor> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for ParameterDescriptorSeq<N> {
    fn drop(&mut self) {
        let mut msg = ParameterDescriptorSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for ParameterDescriptorSeq<N> {}
unsafe impl<const N: usize> Sync for ParameterDescriptorSeq<N> {}

impl TopicMsg for ParameterDescriptor {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterDescriptor()
        }
    }
}

impl PartialEq for ParameterDescriptor {
    fn eq(&self, other: &Self) -> bool {
        unsafe { rcl_interfaces__msg__ParameterDescriptor__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for ParameterDescriptorSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = ParameterDescriptorSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = ParameterDescriptorSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            rcl_interfaces__msg__ParameterDescriptor__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
