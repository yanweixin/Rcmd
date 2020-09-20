#![allow(non_camel_case_types)]

pub type c_void = ::std::os::raw::c_void;

#[link(name = "library")]
extern {
    pub fn hello() -> c_void;
}