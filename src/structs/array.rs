use std::alloc::{Layout, alloc, dealloc};
use std::ptr;

pub struct Arr<T> {
    ptr: *mut T, // pointer to allocated memory
    len: usize,  // number of elements
    size: usize, // array size
}


#[allow(dead_code)]
impl<T> Arr<T> {
    /// Allocate memory.
    pub fn malloc(size: usize) -> Option<Self> {
        // empty == none
        if size == 0 {
            return None;
        }

        // the layout 'size' number of T's
        let layout = Layout::array::<T>(size).expect("Failed to create layout.");

        // allocate memory
        let ptr = unsafe {
            alloc(layout)
        } as *mut T;

        // in case memory allocation fails
        if ptr.is_null() {
            return None;
        }

        // instantiate obeject and return it
        let arr = Self {
            ptr,
            len: 0,
            size
        };

        return Some(arr);
    }


    /// Insert a new element into the array.
    pub fn insert(&mut self, value: T) {
        if self.len >= self.size{
            println!("Array size exceeded.");
            return;
        }

        // write the value into the allocated memory at the next index
        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }

        self.len += 1;
    }


    /// Returns immutable reference to element at the given index.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe {
                return Some(&*self.ptr.add(index));
            }
        }

        else {
            return None;
        }
    }

    /// Returns mutable reference to the element at the given index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            unsafe {
                return Some(&mut *self.ptr.add(index));
            }
        }
        else {
            return None;
        }
    }

    pub fn len(&self) -> usize {
        return self.len;
    }
}


impl<T> Drop for Arr<T> {
    // clean-up allocated memory when array goes out of scope
    fn drop(&mut self) {
        if self.size > 0 {
            unsafe {
                // drop elements
                for i in 0..self.len {
                    ptr::drop_in_place(self.ptr.add(i));
                }
                // deallocate memory
                let layout = Layout::array::<T>(self.size).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
        println!("Array dropped!");
    }
}
