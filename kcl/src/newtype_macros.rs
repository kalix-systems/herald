#[macro_export]
macro_rules! newtype_clone {
    ($newtype:ident) => {
        impl Clone for $newtype {
            fn clone(&self) -> $newtype {
                let &$newtype(v) = self;
                $newtype(v)
            }
        }
    };
}

#[macro_export]
macro_rules! newtype_from_slice {
    ($newtype:ident, $len:expr) => {
        /// `from_slice()` creates an object from a byte slice
        ///
        /// This function will fail and return `None` if the length of
        /// the byte-slice isn't equal to the length of the object
        pub fn from_slice(bs: &[u8]) -> Option<$newtype> {
            if bs.len() != $len {
                return None;
            }

            let mut buf = [0; $len];
            unsafe {
                ::std::ptr::copy_nonoverlapping(bs.as_ptr(), buf.as_mut_ptr(), $len);
            }

            Some($newtype(buf))
        }
    };
}

#[macro_export]
macro_rules! newtype_traits {
    ($newtype:ident, $len:expr) => {
        impl ::std::cmp::PartialEq for $newtype {
            fn eq(&self, other: &$newtype) -> bool {
                unsafe {
                    libsodium_sys::sodium_memcmp(
                        self.0.as_ptr() as *const _,
                        other.0.as_ptr() as *const _,
                        $len,
                    ) == 0
                }
            }
        }

        impl ::std::cmp::Eq for $newtype {}

        impl ::kson::ser::Ser for $newtype {
            fn ser(&self, serializer: &mut ::kson::ser::Serializer) {
                serializer.write_bytes(&self.as_ref());
            }
        }

        impl ::kson::de::De for $newtype {
            fn de(d: &mut ::kson::de::Deserializer) -> Result<Self, ::kson::errors::KsonError> {
                use kson::prelude::*;

                let tag = d.read_bytes_tag()?;
                let len = d.read_bytes_len_from_tag(tag)?;

                if len == $len {
                    let bytes = d.read_raw_slice($len)?;
                    let mut buf = [0u8; $len];
                    unsafe {
                        ::std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf.as_mut_ptr(), $len);
                    }
                    Ok(Self(buf))
                } else {
                    e!(
                        CustomError(format!(
                            "failed to read type {}.\
                             Expected bytestring of length {} but found {}",
                            stringify!($newtype),
                            $len,
                            len
                        )),
                        d.data.clone(),
                        d.ix
                    )
                }
            }
        }

        impl AsRef<[u8]> for $newtype {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! public_newtype_traits {
    ($newtype:ident) => {
        impl ::std::cmp::PartialOrd for $newtype {
            #[inline]
            fn partial_cmp(&self, other: &$newtype) -> Option<::std::cmp::Ordering> {
                ::std::cmp::PartialOrd::partial_cmp(self.as_ref(), other.as_ref())
            }
            #[inline]
            fn lt(&self, other: &$newtype) -> bool {
                ::std::cmp::PartialOrd::lt(self.as_ref(), other.as_ref())
            }
            #[inline]
            fn le(&self, other: &$newtype) -> bool {
                ::std::cmp::PartialOrd::le(self.as_ref(), other.as_ref())
            }
            #[inline]
            fn ge(&self, other: &$newtype) -> bool {
                ::std::cmp::PartialOrd::ge(self.as_ref(), other.as_ref())
            }
            #[inline]
            fn gt(&self, other: &$newtype) -> bool {
                ::std::cmp::PartialOrd::gt(self.as_ref(), other.as_ref())
            }
        }
        impl ::std::cmp::Ord for $newtype {
            #[inline]
            fn cmp(&self, other: &$newtype) -> ::std::cmp::Ordering {
                ::std::cmp::Ord::cmp(self.as_ref(), other.as_ref())
            }
        }
        impl ::std::hash::Hash for $newtype {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                ::std::hash::Hash::hash(self.as_ref(), state)
            }
        }
    };
}

/// Macro used for generating newtypes of byte-arrays
///
/// Usage:
/// Generating secret datatypes, e.g. keys
#[macro_export]
macro_rules! new_type {
    ( $(#[$meta:meta])*
      secret $name:ident($bytes:expr)
      ) => {
        $(#[$meta])*
        #[must_use]
        pub struct $name(pub [u8; $bytes]);

        newtype_clone!($name);
        newtype_traits!($name, $bytes);

        impl $name {
            newtype_from_slice!($name, $bytes);
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    libsodium_sys::sodium_memzero(self.0.as_mut_ptr() as *mut _, $bytes);
                }
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                // Hide secrets from debug output.
                write!(formatter, "{}(****)", stringify!($name))
            }
        }
    };

    ( $(#[$meta:meta])*
      public $name:ident($bytes:expr)
      ) => {
        $(#[$meta])*
        #[derive(Copy)]
        #[must_use]
        pub struct $name(pub [u8; $bytes]);

        newtype_clone!($name);
        newtype_traits!($name, $bytes);
        public_newtype_traits!($name);

        impl $name {
            newtype_from_slice!($name, $bytes);
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self,
                   formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "{}({:?})", stringify!($name), self.as_ref())
            }
        }
    };
    ( $(#[$meta:meta])*
      nonce $name:ident($bytes:expr)
      ) => {
        $(#[$meta])*
        #[derive(Copy)]
        #[must_use]
        pub struct $name(pub [u8; $bytes]);

        newtype_clone!($name);
        newtype_traits!($name, $bytes);
        public_newtype_traits!($name);

        impl $name {
            newtype_from_slice!($name, $bytes);

            /// `increment_le()` treats the nonce as an unsigned little-endian number and
            /// returns an incremented version of it.
            ///
            /// WARNING: this method does not check for arithmetic overflow. It is the callers
            /// responsibility to ensure that any given nonce value is only used once.
            /// If the caller does not do that the cryptographic primitives in kcl
            /// will not uphold any security guarantees (i.e. they will break)
            pub fn increment_le(&self) -> $name {
                let mut res = *self;
                res.increment_le_inplace();
                res
            }

            /// `increment_le_inplace()` treats the nonce as an unsigned little-endian number
            /// and increments it.
            ///
            /// WARNING: this method does not check for arithmetic overflow. It is the callers
            /// responsibility to ensure that any given nonce value is only used once.
            /// If the caller does not do that the cryptographic primitives in kcl
            /// will not uphold any security guarantees.
            pub fn increment_le_inplace(&mut self) {
                unsafe {
                    libsodium_sys::sodium_increment(self.0.as_mut_ptr(), $bytes);
                }
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self,
                   formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, "{}({:?})", stringify!($name), self.as_ref())
            }
        }
    };
}
