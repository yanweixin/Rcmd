#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub type c_void = ::std::os::raw::c_void;
pub type c_char = ::std::os::raw::c_char;
pub type c_int = ::std::os::raw::c_int;
pub type c_long = ::std::os::raw::c_long;
pub type time_t = ::std::os::raw::c_long;
pub type size_t = ::std::os::raw::c_ulong;
pub type c_ulong = ::std::os::raw::c_ulong;
pub type c_ushort = ::std::os::raw::c_ushort;
pub type c_uint = ::std::os::raw::c_uint;

pub type CURL = ::std::os::raw::c_void;
pub type CURLM = ::std::os::raw::c_void;
pub type CURLSH = ::std::os::raw::c_void;
pub type CURLcode = ::std::os::raw::c_uint;
pub type CURLoption = ::std::os::raw::c_uint;
pub type CURLFORMcode = ::std::os::raw::c_uint;
pub type CURLINFO = ::std::os::raw::c_uint;
pub type CURLMSG = ::std::os::raw::c_uint;
pub type CURLSHcode = ::std::os::raw::c_uint;
pub type CURLSHoption = ::std::os::raw::c_uint;
pub type CURLversion = ::std::os::raw::c_uint;
pub type CURLMoption = ::std::os::raw::c_uint;
pub type CURLMcode = ::std::os::raw::c_int;
pub type curl_off_t = ::std::os::raw::c_long;
pub type curl_socket_t = ::std::os::raw::c_int;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curl_version_info_data {
    pub age: CURLversion,
    pub version: *const c_char,
    pub version_num: c_uint,
    pub host: *const c_char,
    pub features: c_int,
    pub ssl_version: *const c_char,
    pub ssl_version_num: c_long,
    pub libz_version: *const c_char,
    pub protocols: *const *const c_char,
    pub ares: *const c_char,
    pub ares_num: c_int,
    pub libidn: *const c_char,
    pub iconv_ver_num: c_int,
    pub libssh_version: *const c_char,
    pub brotli_ver_num: c_uint,
    pub brotli_version: *const c_char,
    pub nghttp2_ver_num: c_uint,
    pub nghttp2_version: *const c_char,
    pub quic_version: *const c_char,
    pub cainfo: *const c_char,
    pub capath: *const c_char,
    pub zstd_ver_num: c_uint,
    pub zstd_version: *const c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curl_httppost {
    pub next: *mut curl_httppost,
    pub name: *mut c_char,
    pub namelength: c_long,
    pub contents: *mut c_char,
    pub contentslength: c_long,
    pub buffer: *mut c_char,
    pub bufferlength: c_long,
    pub contenttype: *mut c_char,
    pub contentheader: *mut curl_slist,
    pub more: *mut curl_httppost,
    pub flags: c_long,
    pub showfilename: *mut c_char,
    pub userp: *mut c_void,
    pub contentlen: curl_off_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curl_slist {
    pub data: *mut c_char,
    pub next: *mut curl_slist,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curl_blob {
    pub data: *mut c_void,
    pub len: size_t,
    pub flags: c_uint,
}
pub type curl_formget_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg: *mut c_void,
        buf: *const c_char,
        len: size_t,
    ) -> size_t,
>;
pub type curl_malloc_callback =
    ::std::option::Option<unsafe extern "C" fn(size: size_t) -> *mut c_void>;
pub type curl_free_callback =
    ::std::option::Option<unsafe extern "C" fn(ptr: *mut c_void)>;
pub type curl_realloc_callback = ::std::option::Option<
    unsafe extern "C" fn(
        ptr: *mut c_void,
        size: size_t,
    ) -> *mut c_void,
>;
pub type curl_strdup_callback = ::std::option::Option<
    unsafe extern "C" fn(str_: *const c_char) -> *mut c_char,
>;
pub type curl_calloc_callback = ::std::option::Option<
    unsafe extern "C" fn(nmemb: size_t, size: size_t) -> *mut c_void,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct curl_waitfd {
    pub fd: curl_socket_t,
    pub events: ::std::os::raw::c_short,
    pub revents: ::std::os::raw::c_short,
}
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CURLMsg {
    pub msg: CURLMSG,
    pub easy_handle: *mut CURL,
    pub data: CURLMsg__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CURLMsg__bindgen_ty_1 {
    pub whatever: *mut ::std::os::raw::c_void,
    pub result: CURLcode,
    _bindgen_union_align: u64,
}

#[link(name = ":libcurl.so.4", kind = "dylib")]
extern "C" {
    pub fn curl_formadd(
        httppost: *mut *mut curl_httppost,
        last_post: *mut *mut curl_httppost,
        ...
    ) -> CURLFORMcode;
    pub fn curl_formget(
        form: *mut curl_httppost,
        arg: *mut c_void,
        append: curl_formget_callback,
    ) -> c_int;
    pub fn curl_formfree(form: *mut curl_httppost);

    pub fn curl_version() -> *mut c_char;

    pub fn curl_easy_escape(handle: *mut CURL, string: *const c_char, length: c_int)
        -> *mut c_char;
    pub fn curl_easy_unescape(
        handle: *mut CURL,
        string: *const c_char,
        length: c_int,
        outlength: *mut c_int,
    ) -> *mut c_char;
    pub fn curl_free(p: *mut c_void);

    pub fn curl_global_init(flags: c_long) -> CURLcode;
    pub fn curl_global_init_mem(
        flags: c_long,
        m: curl_malloc_callback,
        f: curl_free_callback,
        r: curl_realloc_callback,
        s: curl_strdup_callback,
        c: curl_calloc_callback,
    ) -> CURLcode;
    pub fn curl_global_cleanup();

    pub fn curl_slist_append(list: *mut curl_slist, val: *const c_char) -> *mut curl_slist;
    pub fn curl_slist_free_all(list: *mut curl_slist);

    pub fn curl_getdate(p: *const c_char, _: *const time_t) -> time_t;

    pub fn curl_share_init() -> *mut CURLSH;
    pub fn curl_share_setopt(sh: *mut CURLSH, opt: CURLSHoption, ...) -> CURLSHcode;
    pub fn curl_share_cleanup(sh: *mut CURLSH) -> CURLSHcode;

    pub fn curl_version_info(t: CURLversion) -> *mut curl_version_info_data;

    pub fn curl_easy_strerror(code: CURLcode) -> *const c_char;
    pub fn curl_share_strerror(code: CURLSHcode) -> *const c_char;
    pub fn curl_easy_pause(handle: *mut CURL, bitmask: c_int) -> CURLcode;

    pub fn curl_easy_init() -> *mut CURL;
    pub fn curl_easy_setopt(curl: *mut CURL, option: CURLoption, ...) -> CURLcode;
    pub fn curl_easy_perform(curl: *mut CURL) -> CURLcode;
    pub fn curl_easy_cleanup(curl: *mut CURL);
    pub fn curl_easy_getinfo(curl: *mut CURL, info: CURLINFO, ...) -> CURLcode;
    pub fn curl_easy_duphandle(curl: *mut CURL) -> *mut CURL;
    pub fn curl_easy_reset(curl: *mut CURL);
    pub fn curl_easy_recv(
        curl: *mut CURL,
        buffer: *mut c_void,
        buflen: size_t,
        n: *mut size_t,
    ) -> CURLcode;
    pub fn curl_easy_send(
        curl: *mut CURL,
        buffer: *const c_void,
        buflen: size_t,
        n: *mut size_t,
    ) -> CURLcode;
    pub fn curl_easy_upkeep(curl: *mut CURL) -> CURLcode;

    pub fn curl_multi_init() -> *mut CURLM;
    pub fn curl_multi_add_handle(multi_handle: *mut CURLM, curl_handle: *mut CURL) -> CURLMcode;
    pub fn curl_multi_remove_handle(multi_handle: *mut CURLM, curl_handle: *mut CURL) -> CURLMcode;
    pub fn curl_multi_fdset(
        multi_handle: *mut CURLM,
        read_fd_set: *mut fd_set,
        write_fd_set: *mut fd_set,
        exc_fd_set: *mut fd_set,
        max_fd: *mut c_int,
    ) -> CURLMcode;
    pub fn curl_multi_wait(
        multi_handle: *mut CURLM,
        extra_fds: *mut curl_waitfd,
        extra_nfds: c_uint,
        timeout_ms: c_int,
        ret: *mut c_int,
    ) -> CURLMcode;
    pub fn curl_multi_perform(multi_handle: *mut CURLM, running_handles: *mut c_int) -> CURLMcode;
    pub fn curl_multi_cleanup(multi_handle: *mut CURLM) -> CURLMcode;
    pub fn curl_multi_info_read(
        multi_handle: *mut CURLM,
        msgs_in_queue: *mut c_int,
    ) -> *mut CURLMsg;
    pub fn curl_multi_strerror(code: CURLMcode) -> *const c_char;
    pub fn curl_multi_socket(
        multi_handle: *mut CURLM,
        s: curl_socket_t,
        running_handles: *mut c_int,
    ) -> CURLMcode;
    pub fn curl_multi_socket_action(
        multi_handle: *mut CURLM,
        s: curl_socket_t,
        ev_bitmask: c_int,
        running_handles: *mut c_int,
    ) -> CURLMcode;
    pub fn curl_multi_socket_all(
        multi_handle: *mut CURLM,
        running_handles: *mut c_int,
    ) -> CURLMcode;
    pub fn curl_multi_timeout(multi_handle: *mut CURLM, milliseconds: *mut c_long) -> CURLMcode;
    pub fn curl_multi_setopt(multi_handle: *mut CURLM, option: CURLMoption, ...) -> CURLMcode;
    pub fn curl_multi_assign(
        multi_handle: *mut CURLM,
        sockfd: curl_socket_t,
        sockp: *mut c_void,
    ) -> CURLMcode;
}
