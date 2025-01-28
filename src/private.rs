use crate::chan::{Samp8, Samp16, Samp24, Samp32, Samp64};

pub trait Sealed {}
impl Sealed for Samp8 {}
impl Sealed for Samp16 {}
impl Sealed for Samp24 {}
impl Sealed for Samp32 {}
impl Sealed for Samp64 {}
