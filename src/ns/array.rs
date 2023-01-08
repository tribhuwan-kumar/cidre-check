use std::{
    ffi::c_void,
    marker::PhantomData,
    mem::transmute,
    ops::{Deref, Index, IndexMut},
};

use crate::{arc, msg_send, ns, objc};

#[derive(Debug)]
#[repr(transparent)]
pub struct Array<T>(ns::Id, PhantomData<T>)
where
    T: objc::Obj;

impl<T> objc::Obj for Array<T> where T: objc::Obj {}

#[derive(Debug)]
#[repr(transparent)]
pub struct ArrayMut<T>(ns::Array<T>)
where
    T: objc::Obj;

impl<T> objc::Obj for ArrayMut<T> where T: objc::Obj {}

impl<T> Deref for Array<T>
where
    T: objc::Obj,
{
    type Target = ns::Id;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Deref for ArrayMut<T>
where
    T: objc::Obj,
{
    type Target = Array<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Array<T>
where
    T: objc::Obj,
{
    #[inline]
    pub fn new() -> arc::R<Self> {
        unsafe { transmute(NSArray_array()) }
    }

    #[inline]
    pub fn from_slice(objs: &[&T]) -> arc::R<Self> {
        unsafe { transmute(NSArray_withObjs(objs.as_ptr() as _, objs.len())) }
    }

    #[inline]
    pub fn count(&self) -> usize {
        msg_send!("ns", self, ns_count)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.count()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn iter(&self) -> ns::FEIterator<Self, T> {
        ns::FastEnumeration::iter(self)
    }

    #[inline(never)]
    pub fn foo() {
        let one = ns::Number::with_i32(5);
        let arr: &[&ns::Number] = &[&one];
        let array = ns::Array::from_slice(&arr);

        let mut k = 0;
        for i in array.iter() {
            k += 1;
        }
    }
}

impl<T> Index<usize> for Array<T>
where
    T: objc::Obj,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        msg_send!("ns", self, ns_objectAtIndex_index, index)
    }
}

impl<T> IndexMut<usize> for Array<T>
where
    T: objc::Obj,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        msg_send!("ns", self, ns_objectAtIndex_index, index)
    }
}

// impl<'a, T, const N: usize> ExactSizeIterator for ns::FEIterator<'a, Array<T>, T, N>
// where
//     T: objc::Obj + 'a,
// {
//     fn len(&self) -> usize {
//         self.enu.len() - self.index
//     }
// }

impl<T> ns::FastEnumeration<T> for Array<T> where T: objc::Obj {}
impl<T> ns::FastEnumeration<T> for ArrayMut<T> where T: objc::Obj {}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSArray_withObjs(
        objects: *const c_void,
        count: ns::UInteger,
    ) -> arc::Retained<Array<ns::Id>>;

    fn NSArray_array() -> arc::Retained<Array<ns::Id>>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let one = ns::Number::with_i32(5);
        let arr: &[&ns::Number] = &[&one];
        let array = ns::Array::from_slice(&arr);
        assert_eq!(1, array.len());
        assert_eq!(5, array[0].as_i32());

        let mut k = 0;
        for i in array.iter() {
            k += 1;
            //            println!("{:?}", i);
        }

        assert_eq!(1, k);
    }
}
