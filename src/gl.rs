use x11rb::protocol::xproto::ConnectionExt;

use crate::platform::gl::*;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct GlConfig {
    pub version: (u8, u8),
    pub profile: Profile,
    pub red_bits: u8,
    pub blue_bits: u8,
    pub green_bits: u8,
    pub alpha_bits: u8,
    pub depth_bits: u8,
    pub stencil_bits: u8,
    pub samples: Option<u8>,
    pub srgb: bool,
    pub double_buffer: bool,
    pub vsync: bool,
}

impl Default for GlConfig {
    fn default() -> Self {
        GlConfig {
            version: (3, 2),
            profile: Profile::Core,
            red_bits: 8,
            blue_bits: 8,
            green_bits: 8,
            alpha_bits: 8,
            depth_bits: 24,
            stencil_bits: 8,
            samples: None,
            srgb: true,
            double_buffer: true,
            vsync: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    Compatibility,
    Core,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum GlError {
    VersionNotSupported,
    CreationFailed(CreationFailedError),
}

#[derive(Clone)]
pub struct GlContext {
    inner: crate::platform::gl::GlContext,
    // To make sure this is !Send, !Sync, and !UnwindSafe on all platforms
    phantom: PhantomData<(*mut (), &'static mut ())>,
}

impl GlContext {
    pub(crate) fn new(context: crate::platform::gl::GlContext) -> GlContext {
        {
            let p = &context as *const crate::platform::gl::GlContext;
            dbg!(p);
        }

        GlContext { inner: context, phantom: PhantomData }
    }

    pub unsafe fn make_current(&self) {
        dbg!("p_x11_gl_mc");
        {
            let p = self as *const GlContext;
            dbg!(p);

            // let weak_count = std::rc::Rc::<_>::weak_count(&self.inner);
            // let weak_count = std::rc::Rc::<_>::weak_count(&self.inner);
            // dbg!(weak_count);
            // let strong_count = std::rc::Rc::<_>::strong_count(&self.inner);
            // dbg!(strong_count);
            let p = &self.inner as *const Rc<GlContextInner>;
            dbg!(p);
            let p = &*self.inner as *const GlContextInner;
            dbg!(p);
        }
        self.inner.make_current();
    }

    pub unsafe fn make_not_current(&self) {
        dbg!("p_x11_gl_mnc");
        self.inner.make_not_current();
    }

    pub fn get_proc_address(&self, symbol: &str) -> *const c_void {
        dbg!("p_x11_gl_gpa");
        self.inner.get_proc_address(symbol)
        // todo!()
    }

    pub fn swap_buffers(&self) {
        dbg!("p_x11_gl_sb");
        self.inner.swap_buffers();
    }
}
