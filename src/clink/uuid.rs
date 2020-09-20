#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub const UUID_VARIANT_NCS: u32 = 0;
pub const UUID_VARIANT_DCE: u32 = 1;
pub const UUID_VARIANT_MICROSOFT: u32 = 2;
pub const UUID_VARIANT_OTHER: u32 = 3;
pub const UUID_VARIANT_SHIFT: u32 = 5;
pub const UUID_VARIANT_MASK: u32 = 7;
pub const UUID_TYPE_DCE_TIME: u32 = 1;
pub const UUID_TYPE_DCE_SECURITY: u32 = 2;
pub const UUID_TYPE_DCE_MD5: u32 = 3;
pub const UUID_TYPE_DCE_RANDOM: u32 = 4;
pub const UUID_TYPE_DCE_SHA1: u32 = 5;
pub const UUID_TYPE_SHIFT: u32 = 4;
pub const UUID_TYPE_MASK: u32 = 15;
pub const UUID_STR_LEN: u32 = 37;

pub type size_t = ::std::os::raw::c_ulong;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type c_int = ::std::os::raw::c_int;
pub type c_char = ::std::os::raw::c_char;
pub type uuid_t = [::std::os::raw::c_uchar; 16usize];

#[link(name = "uuid", kind = "dylib")]
extern "C" {
    pub fn uuid_clear(uu: *mut uuid_t);
    pub fn uuid_compare(
        uu1: *mut uuid_t,
        uu2: *mut uuid_t,
    ) -> c_int;
    pub fn uuid_copy(dst: *mut uuid_t, src: *mut uuid_t);
    pub fn uuid_generate(out: *mut uuid_t);
    pub fn uuid_generate_random(out: *mut uuid_t);
    pub fn uuid_generate_time(out: *mut uuid_t);
    pub fn uuid_generate_time_safe(out: *mut uuid_t) -> c_int;
    pub fn uuid_generate_md5(
        out: *mut uuid_t,
        ns: *mut uuid_t,
        name: *const c_char,
        len: size_t,
    );
    pub fn uuid_generate_sha1(
        out: *mut uuid_t,
        ns: *mut uuid_t,
        name: *const c_char,
        len: size_t,
    );
    pub fn uuid_is_null(uu: *mut uuid_t) -> c_int;
    pub fn uuid_parse(
        in_: *const c_char,
        uu: *mut uuid_t,
    ) -> c_int;
    pub fn uuid_unparse(uu: *mut uuid_t, out: *mut c_char);
    pub fn uuid_unparse_lower(uu: *mut uuid_t, out: *mut c_char);
    pub fn uuid_unparse_upper(uu: *mut uuid_t, out: *mut c_char);
    //     pub fn uuid_time(uu: *mut ::std::os::raw::c_uchar, ret_tv: *mut timeval) -> time_t;
    pub fn uuid_type(uu: *mut uuid_t) -> c_int;
    pub fn uuid_variant(uu: *mut uuid_t) -> c_int;
    pub fn uuid_get_template(alias: *const c_char) -> *const uuid_t;
}

