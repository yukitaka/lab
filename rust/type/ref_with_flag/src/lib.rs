pub mod ref_with_flag {
    use std::marker::PhantomData;
    use std::mem::align_of;

    pub struct RefWithFlag<'a, T: 'a> {
        ptr_and_bit: usize,
        behaves_like: PhantomData<&'a T>,
    }

    impl<'a, T: 'a> RefWithFlag<'a, T> {
        pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
            assert_eq!(align_of::<T>() % 2, 0);
            RefWithFlag {
                ptr_and_bit: ptr as *const T as usize | flag as usize,
                behaves_like: PhantomData,
            }
        }

        pub fn get_ref(&self) -> &'a T {
            unsafe {
                let ptr = (self.ptr_and_bit & !1) as *const T;
                &*ptr
            }
        }

        pub fn get_flag(&self) -> bool {
            self.ptr_and_bit & 1 != 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_size() {
        assert_eq!(std::mem::size_of::<i32>(), 4);
        assert_eq!(std::mem::size_of::<ref_with_flag::RefWithFlag<i32>>(), 8);

        assert_eq!(std::mem::size_of::<Vec<i32>>(), 24);
        assert_eq!(
            std::mem::size_of::<ref_with_flag::RefWithFlag<Vec<i32>>>(),
            8
        );
    }
}
