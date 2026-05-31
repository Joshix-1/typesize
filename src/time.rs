use super::sizeof_impl;
use crate::TypeSize;

sizeof_impl!(core::time::Duration);

#[cfg(feature = "std")]
sizeof_impl!(std::time::Instant, std::time::SystemTime);
