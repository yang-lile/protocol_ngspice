/* automatically generated by rust-bindgen 0.65.1 */
#[warn(non_camel_case_types)]
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const NGSPICE_PACKAGE_VERSION: &[u8; 3usize] = b"39\0";
pub const HAS_NG_BOOL: u32 = 1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ngcomplex {
    pub cx_real: f64,
    pub cx_imag: f64,
}
#[test]
fn bindgen_test_layout_ngcomplex() {
    const UNINIT: ::std::mem::MaybeUninit<ngcomplex> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ngcomplex>(),
        16usize,
        concat!("Size of: ", stringify!(ngcomplex))
    );
    assert_eq!(
        ::std::mem::align_of::<ngcomplex>(),
        8usize,
        concat!("Alignment of ", stringify!(ngcomplex))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cx_real) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ngcomplex),
            "::",
            stringify!(cx_real)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cx_imag) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ngcomplex),
            "::",
            stringify!(cx_imag)
        )
    );
}
pub type ngcomplex_t = ngcomplex;
pub type NG_BOOL = bool;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vector_info {
    pub v_name: *mut ::std::os::raw::c_char,
    pub v_type: ::std::os::raw::c_int,
    pub v_flags: ::std::os::raw::c_short,
    pub v_realdata: *mut f64,
    pub v_compdata: *mut ngcomplex_t,
    pub v_length: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_vector_info() {
    const UNINIT: ::std::mem::MaybeUninit<vector_info> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vector_info>(),
        40usize,
        concat!("Size of: ", stringify!(vector_info))
    );
    assert_eq!(
        ::std::mem::align_of::<vector_info>(),
        8usize,
        concat!("Alignment of ", stringify!(vector_info))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v_name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vector_info),
            "::",
            stringify!(v_name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v_type) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vector_info),
            "::",
            stringify!(v_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v_flags) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(vector_info),
            "::",
            stringify!(v_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v_realdata) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(vector_info),
            "::",
            stringify!(v_realdata)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v_compdata) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(vector_info),
            "::",
            stringify!(v_compdata)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v_length) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(vector_info),
            "::",
            stringify!(v_length)
        )
    );
}
pub type pvector_info = *mut vector_info;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vecvalues {
    pub name: *mut ::std::os::raw::c_char,
    pub creal: f64,
    pub cimag: f64,
    pub is_scale: NG_BOOL,
    pub is_complex: NG_BOOL,
}
#[test]
fn bindgen_test_layout_vecvalues() {
    const UNINIT: ::std::mem::MaybeUninit<vecvalues> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vecvalues>(),
        32usize,
        concat!("Size of: ", stringify!(vecvalues))
    );
    assert_eq!(
        ::std::mem::align_of::<vecvalues>(),
        8usize,
        concat!("Alignment of ", stringify!(vecvalues))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vecvalues),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).creal) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vecvalues),
            "::",
            stringify!(creal)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cimag) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(vecvalues),
            "::",
            stringify!(cimag)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).is_scale) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(vecvalues),
            "::",
            stringify!(is_scale)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).is_complex) as usize - ptr as usize },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(vecvalues),
            "::",
            stringify!(is_complex)
        )
    );
}
pub type pvecvalues = *mut vecvalues;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vecvaluesall {
    pub veccount: ::std::os::raw::c_int,
    pub vecindex: ::std::os::raw::c_int,
    pub vecsa: *mut pvecvalues,
}
#[test]
fn bindgen_test_layout_vecvaluesall() {
    const UNINIT: ::std::mem::MaybeUninit<vecvaluesall> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vecvaluesall>(),
        16usize,
        concat!("Size of: ", stringify!(vecvaluesall))
    );
    assert_eq!(
        ::std::mem::align_of::<vecvaluesall>(),
        8usize,
        concat!("Alignment of ", stringify!(vecvaluesall))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).veccount) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vecvaluesall),
            "::",
            stringify!(veccount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vecindex) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(vecvaluesall),
            "::",
            stringify!(vecindex)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vecsa) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vecvaluesall),
            "::",
            stringify!(vecsa)
        )
    );
}
pub type pvecvaluesall = *mut vecvaluesall;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vecinfo {
    pub number: ::std::os::raw::c_int,
    pub vecname: *mut ::std::os::raw::c_char,
    pub is_real: NG_BOOL,
    pub pdvec: *mut ::std::os::raw::c_void,
    pub pdvecscale: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_vecinfo() {
    const UNINIT: ::std::mem::MaybeUninit<vecinfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vecinfo>(),
        40usize,
        concat!("Size of: ", stringify!(vecinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<vecinfo>(),
        8usize,
        concat!("Alignment of ", stringify!(vecinfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).number) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vecinfo),
            "::",
            stringify!(number)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vecname) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vecinfo),
            "::",
            stringify!(vecname)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).is_real) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(vecinfo),
            "::",
            stringify!(is_real)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pdvec) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(vecinfo),
            "::",
            stringify!(pdvec)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pdvecscale) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(vecinfo),
            "::",
            stringify!(pdvecscale)
        )
    );
}
pub type pvecinfo = *mut vecinfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vecinfoall {
    pub name: *mut ::std::os::raw::c_char,
    pub title: *mut ::std::os::raw::c_char,
    pub date: *mut ::std::os::raw::c_char,
    pub type_: *mut ::std::os::raw::c_char,
    pub veccount: ::std::os::raw::c_int,
    pub vecs: *mut pvecinfo,
}
#[test]
fn bindgen_test_layout_vecinfoall() {
    const UNINIT: ::std::mem::MaybeUninit<vecinfoall> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vecinfoall>(),
        48usize,
        concat!("Size of: ", stringify!(vecinfoall))
    );
    assert_eq!(
        ::std::mem::align_of::<vecinfoall>(),
        8usize,
        concat!("Alignment of ", stringify!(vecinfoall))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vecinfoall),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).title) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vecinfoall),
            "::",
            stringify!(title)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).date) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(vecinfoall),
            "::",
            stringify!(date)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(vecinfoall),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).veccount) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(vecinfoall),
            "::",
            stringify!(veccount)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).vecs) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(vecinfoall),
            "::",
            stringify!(vecs)
        )
    );
}
pub type pvecinfoall = *mut vecinfoall;
pub type SendChar = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type SendStat = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type ControlledExit = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: NG_BOOL,
        arg3: NG_BOOL,
        arg4: ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type SendData = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: pvecvaluesall,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type SendInitData = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: pvecinfoall,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type BGThreadRunning = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: NG_BOOL,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type GetVSRCData = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f64,
        arg2: f64,
        arg3: *mut ::std::os::raw::c_char,
        arg4: ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type GetISRCData = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut f64,
        arg2: f64,
        arg3: *mut ::std::os::raw::c_char,
        arg4: ::std::os::raw::c_int,
        arg5: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type GetSyncData = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: f64,
        arg2: *mut f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
        arg7: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn ngSpice_Init(
        printfcn: SendChar,
        statfcn: SendStat,
        ngexit: ControlledExit,
        sdata: SendData,
        sinitdata: SendInitData,
        bgtrun: BGThreadRunning,
        userData: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ngSpice_Init_Sync(
        vsrcdat: GetVSRCData,
        isrcdat: GetISRCData,
        syncdat: GetSyncData,
        ident: *mut ::std::os::raw::c_int,
        userData: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ngSpice_Command(command: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ngGet_Vec_Info(vecname: *mut ::std::os::raw::c_char) -> pvector_info;
}
extern "C" {
    pub fn ngSpice_Circ(circarray: *mut *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ngSpice_CurPlot() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ngSpice_AllPlots() -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ngSpice_AllVecs(
        plotname: *mut ::std::os::raw::c_char,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ngSpice_running() -> NG_BOOL;
}
extern "C" {
    pub fn ngSpice_SetBkpt(time: f64) -> NG_BOOL;
}
