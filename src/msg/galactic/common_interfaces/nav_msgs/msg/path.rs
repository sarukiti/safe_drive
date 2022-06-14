// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn nav_msgs__msg__Path__init(msg: *mut Path) -> bool;
    fn nav_msgs__msg__Path__fini(msg: *mut Path);
    fn nav_msgs__msg__Path__Sequence__init(msg: *mut PathSequence, size: usize) -> bool;
    fn nav_msgs__msg__Path__Sequence__fini(msg: *mut PathSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__Path() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Path {
    pub header: std_msgs::msg::Header,
    pub poses: geometry_msgs::msg::PoseStampedSequence,
}

impl Path {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__msg__Path__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { nav_msgs__msg__Path__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PathSequence {
    data: *mut Path,
    size: usize,
    capacity: usize,
}

impl PathSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { nav_msgs__msg__Path__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Path]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Path]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for PathSequence {
    fn drop(&mut self) {
        unsafe { nav_msgs__msg__Path__Sequence__fini(self) };
    }
}

impl TopicMsg for Path {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__Path()
        }
    }
}