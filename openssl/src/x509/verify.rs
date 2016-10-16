use std::marker::PhantomData;
use libc::c_uint;
use ffi;

use error::ErrorStack;

bitflags! {
    pub flags X509CheckFlags: c_uint {
        const X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT = ffi::X509_CHECK_FLAG_ALWAYS_CHECK_SUBJECT,
        const X509_CHECK_FLAG_NO_WILDCARDS = ffi::X509_CHECK_FLAG_NO_WILDCARDS,
        const X509_CHECK_FLAG_NO_PARTIAL_WILDCARDS = ffi::X509_CHECK_FLAG_NO_PARTIAL_WILDCARDS,
        const X509_CHECK_FLAG_MULTI_LABEL_WILDCARDS = ffi::X509_CHECK_FLAG_MULTI_LABEL_WILDCARDS,
        const X509_CHECK_FLAG_SINGLE_LABEL_SUBDOMAINS
            = ffi::X509_CHECK_FLAG_SINGLE_LABEL_SUBDOMAINS,
        #[cfg(feature = "openssl-110")]
        const X509_CHECK_FLAG_NEVER_CHECK_SUBJECT = ffi::X509_CHECK_FLAG_NEVER_CHECK_SUBJECT,
    }
}

pub struct X509VerifyParamRef<'a>(*mut ffi::X509_VERIFY_PARAM, PhantomData<&'a mut ()>);

impl<'a> X509VerifyParamRef<'a> {
    pub unsafe fn from_ptr(ptr: *mut ffi::X509_VERIFY_PARAM) -> X509VerifyParamRef<'a> {
        X509VerifyParamRef(ptr, PhantomData)
    }

    pub fn set_hostflags(&mut self, hostflags: X509CheckFlags) {
        unsafe {
            ffi::X509_VERIFY_PARAM_set_hostflags(self.0, hostflags.bits);
        }
    }

    pub fn set_host(&mut self, host: &str) -> Result<(), ErrorStack> {
        unsafe {
            try_ssl!(ffi::X509_VERIFY_PARAM_set1_host(self.0,
                                                      host.as_ptr() as *const _,
                                                      host.len()))
        }

        Ok(())
    }
}