#![forbid(unsafe_code)]

use std::io;

pub trait Id {
    fn id(&self) -> String;
}

pub trait Store {
    fn store<T: io::Write>(&mut self, writer: T) -> io::Result<()>;
}

pub trait Load {
    fn load<R: io::Read>(reader: R) -> io::Result<Self> where Self: std::marker::Sized;

    // This is currently only used from index_store_mapped_file, so propose a
    // default implementation which fails as a simplification for those not
    // interested in using mapped files at all.
    #[allow(unused_variables)]
    fn load_slice(from: &[u8]) -> io::Result<Self> where Self: std::marker::Sized { unimplemented!(); }
}
