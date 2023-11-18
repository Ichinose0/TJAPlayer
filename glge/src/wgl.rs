use crate::wgl::extra::types::HDC;
use std::intrinsics::transmute;

pub const WGL_DRAW_TO_WINDOW_ARB:i32 = 0x2001;
pub const WGL_SUPPORT_OPENGL_ARB:i32 = 0x2010;
pub const WGL_DOUBLE_BUFFER_ARB:i32 = 0x2011;
pub const WGL_PIXEL_TYPE_ARB:i32 = 0x2013;
pub const WGL_TYPE_RGBA_ARB:i32 = 0x202B;
pub const WGL_COLOR_BITS_ARB:i32 = 0x2014;
pub const WGL_DEPTH_BITS_ARB:i32 = 0x2022;
pub const WGL_STENCIL_BITS_ARB:i32 = 0x2023;

pub type WGLCREATECONTEXTATTRIBSARBPROC = fn(
    hDc: extra::types::HDC,
    hShareContext: extra::types::HGLRC,
    attribs: &[extra::types::GLenum],
) -> extra::types::HGLRC;

pub type WGLCHOOSEPIXELFORMATARBPROC = fn(
    hDc: extra::types::HDC,
    piAttribIList: *const i32,
    pfAttribFList: *const f64,
    nMaxFormats: u32,
    piFormats: *mut i32,
    nNumFormats: *mut u32
) -> extra::types::BOOL;
pub type WGLSWAPINTERVALEXTPROC = fn(i: u32);
pub struct WGLARBFunction {
    wglCreateContextAttribsARB: WGLCREATECONTEXTATTRIBSARBPROC,
    wglChoosePixelFormatARB: WGLCHOOSEPIXELFORMATARBPROC,
    wglSwapIntervalEXT: WGLSWAPINTERVALEXTPROC,
}

impl WGLARBFunction {
    pub fn load() -> Self {
        let wglCreateContextAttribsARB: WGLCREATECONTEXTATTRIBSARBPROC = unsafe {
            transmute(GetProcAddress(
                "wglCreateContextAttribsARB\0".as_ptr() as types::LPCSTR
            ))
        };
        let wglChoosePixelFormatARB: WGLCHOOSEPIXELFORMATARBPROC = unsafe {
          transmute(GetProcAddress(
              "wglChoosePixelFormatARB\0".as_ptr() as types::LPCSTR
          ))
        };
        let wglSwapIntervalEXT: WGLSWAPINTERVALEXTPROC = unsafe {
            transmute(GetProcAddress(
                "wglSwapIntervalEXT\0".as_ptr() as types::LPCSTR
            ))
        };
        Self {
            wglCreateContextAttribsARB,
            wglChoosePixelFormatARB,
            wglSwapIntervalEXT,
        }
    }

    pub unsafe fn CreateContextAttribsARB(&self,hDc: extra::types::HDC,hShareContext: extra::types::HGLRC,attribs: &[extra::types::GLenum]) -> extra::types::HGLRC {
        (self.wglCreateContextAttribsARB)(hDc,hShareContext,attribs)
    }

    pub unsafe fn ChoosePixelFormatARB(&self,hdc: HDC,piAttribIList: *const i32,pfAttribFList: *const f64,nMaxFormats: u32,piFormats: *mut i32,nNumFormats: *mut u32) {
        (self.wglChoosePixelFormatARB)(hdc,piAttribIList,pfAttribFList,nMaxFormats,piFormats,nNumFormats);
    }
}

include!(concat!(env!("OUT_DIR"), "/wgl.rs"));

pub(crate) mod extra {
    include!(concat!(env!("OUT_DIR"), "/wgl_extra_bindings.rs"));
}