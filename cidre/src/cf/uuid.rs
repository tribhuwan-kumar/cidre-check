use crate::{arc, cf, define_cf_type};

define_cf_type!(
    #[doc(alias = "CFUUID")]
    Uuid(cf::Type)
);

impl Uuid {
    #[doc(alias = "CFUUIDGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFUUIDGetTypeID() }
    }

    #[doc(alias = "CFUUIDCreate")]
    #[inline]
    pub fn new_in(alloc: Option<&cf::Allocator>) -> Option<arc::R<Uuid>> {
        unsafe { CFUUIDCreate(alloc) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let uuid = cf::Uuid::new();
    /// uuid.show();
    /// let uuid2 = cf::Uuid::new();
    /// assert!(!uuid.equal(&uuid2));
    /// ```
    #[doc(alias = "CFUUIDCreate")]
    #[inline]
    pub fn new() -> arc::R<Uuid> {
        unsafe { Self::new_in(None).unwrap_unchecked() }
    }

    #[doc(alias = "CFUUIDCreateFromString")]
    #[inline]
    pub fn from_cf_string_in(
        uuid_str: &cf::String,
        alloc: Option<&cf::Allocator>,
    ) -> Option<arc::R<Uuid>> {
        unsafe { CFUUIDCreateFromString(alloc, uuid_str) }
    }

    #[doc(alias = "CFUUIDCreateString")]
    #[inline]
    pub fn to_cf_string_in(&self, alloc: Option<&cf::Allocator>) -> Option<arc::R<cf::String>> {
        unsafe { CFUUIDCreateString(alloc, self) }
    }

    #[doc(alias = "CFUUIDCreateString")]
    #[inline]
    pub fn to_cf_string(&self) -> arc::R<cf::String> {
        unsafe { self.to_cf_string_in(None).unwrap_unchecked() }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C-unwind" {
    fn CFUUIDGetTypeID() -> cf::TypeId;
    fn CFUUIDCreate(alloc: Option<&cf::Allocator>) -> Option<arc::R<Uuid>>;
    fn CFUUIDCreateFromString(
        alloc: Option<&cf::Allocator>,
        uuid_str: &cf::String,
    ) -> Option<arc::R<Uuid>>;
    fn CFUUIDCreateString(alloc: Option<&cf::Allocator>, uuid: &Uuid)
    -> Option<arc::R<cf::String>>;
}

#[cfg(test)]
mod tests {
    use crate::cf;

    #[test]
    fn basics() {
        let uuid1 = cf::Uuid::new();
        let str1 = uuid1.to_cf_string();
        assert!(!str1.is_empty());
        let str2 = uuid1.to_cf_string();
        assert!(str1.equal(&str2));

        let str3 = cf::Uuid::new().to_cf_string();
        assert!(!str3.is_empty());

        assert!(!str1.equal(&str3));
    }
}
