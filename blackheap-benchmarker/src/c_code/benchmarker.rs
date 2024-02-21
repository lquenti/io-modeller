/* automatically generated by rust-bindgen 0.69.4 */

pub const MEMINFO: &[u8; 14] = b"/proc/meminfo\0";
pub const DROP_PAGE_CACHE: &[u8; 25] = b"/proc/sys/vm/drop_caches\0";
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    const UNINIT: ::std::mem::MaybeUninit<max_align_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce2) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
pub const access_pattern_ACCESS_PATTERN_CONST: access_pattern = 0;
pub const access_pattern_ACCESS_PATTERN_SEQUENTIAL: access_pattern = 1;
pub const access_pattern_ACCESS_PATTERN_RANDOM: access_pattern = 2;
pub type access_pattern = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct benchmark_config {
    pub filepath: *const ::std::os::raw::c_char,
    pub memory_buffer_in_bytes: usize,
    pub file_size_in_bytes: usize,
    pub access_size_in_bytes: usize,
    pub number_of_io_op_tests: usize,
    pub access_pattern_in_memory: access_pattern,
    pub access_pattern_in_ile: access_pattern,
    pub is_read_operation: bool,
    pub prepare_file_size: bool,
    pub use_o_direct: bool,
    pub drop_cache_first: bool,
    pub do_reread: bool,
    pub delete_afterwards: bool,
    pub restrict_free_ram_to: usize,
}
#[test]
fn bindgen_test_layout_benchmark_config() {
    const UNINIT: ::std::mem::MaybeUninit<benchmark_config> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<benchmark_config>(),
        64usize,
        concat!("Size of: ", stringify!(benchmark_config))
    );
    assert_eq!(
        ::std::mem::align_of::<benchmark_config>(),
        8usize,
        concat!("Alignment of ", stringify!(benchmark_config))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).filepath) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(filepath)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).memory_buffer_in_bytes) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(memory_buffer_in_bytes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).file_size_in_bytes) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(file_size_in_bytes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).access_size_in_bytes) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(access_size_in_bytes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).number_of_io_op_tests) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(number_of_io_op_tests)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).access_pattern_in_memory) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(access_pattern_in_memory)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).access_pattern_in_ile) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(access_pattern_in_ile)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).is_read_operation) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(is_read_operation)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).prepare_file_size) as usize - ptr as usize },
        49usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(prepare_file_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).use_o_direct) as usize - ptr as usize },
        50usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(use_o_direct)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).drop_cache_first) as usize - ptr as usize },
        51usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(drop_cache_first)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).do_reread) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(do_reread)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).delete_afterwards) as usize - ptr as usize },
        53usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(delete_afterwards)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).restrict_free_ram_to) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_config),
            "::",
            stringify!(restrict_free_ram_to)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct benchmark_results {
    pub length: usize,
    pub durations: *mut f64,
}
#[test]
fn bindgen_test_layout_benchmark_results() {
    const UNINIT: ::std::mem::MaybeUninit<benchmark_results> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<benchmark_results>(),
        16usize,
        concat!("Size of: ", stringify!(benchmark_results))
    );
    assert_eq!(
        ::std::mem::align_of::<benchmark_results>(),
        8usize,
        concat!("Alignment of ", stringify!(benchmark_results))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).length) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_results),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).durations) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(benchmark_results),
            "::",
            stringify!(durations)
        )
    );
}
pub const error_codes_SUCCESS: error_codes = 0;
pub const error_codes_MALLOC_FAILED: error_codes = 1;
pub const error_codes_OPEN_FAILED: error_codes = 2;
pub const error_codes_READ_FAILED: error_codes = 3;
pub const error_codes_WRITE_FAILED: error_codes = 4;
pub const error_codes_CLOSE_FAILED: error_codes = 5;
pub const error_codes_LSEEK_FAILED: error_codes = 6;
pub const error_codes_FSYNC_FAILED: error_codes = 7;
pub const error_codes_FSTAT_FAILED: error_codes = 8;
pub const error_codes_IO_OP_FAILED: error_codes = 9;
pub const error_codes_REMOVE_FAILED: error_codes = 10;
pub type error_codes = ::std::os::raw::c_uint;
extern "C" {
    pub fn benchmark_file(config: *const benchmark_config) -> benchmark_results;
}
