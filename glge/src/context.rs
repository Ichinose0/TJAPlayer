use raw_window_handle::HasWindowHandle;
use std::ffi::{c_void, CStr, CString, OsStr};
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{addr_of, null, null_mut};
use windows_sys::core::PCSTR;
use windows_sys::Win32::Graphics::Gdi::GetDC;
use windows_sys::Win32::Graphics::OpenGL::{
    ChoosePixelFormat, SetPixelFormat, SwapBuffers, GL_TRUE, PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW,
    PFD_MAIN_PLANE, PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA,
};
use windows_sys::Win32::System::LibraryLoader::{LoadLibraryA, LoadLibraryW};

use crate::wgl;
use crate::wgl::types::{BYTE, PIXELFORMATDESCRIPTOR, WORD};
use crate::wgl::*;

#[derive(Clone, Copy, Debug)]
pub struct Context {
    func: WGLARBFunctions,
    hdc: wgl::extra::types::HDC,
    ctx: wgl::extra::types::HGLRC,
}

impl Context {
    pub fn init(window: &impl HasWindowHandle) -> Self {
        info!("Starts the creation of an OpenGL context");
        let handle = window.window_handle().unwrap();
        match handle.as_raw() {
            raw_window_handle::RawWindowHandle::UiKit(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::AppKit(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::Orbital(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::Xlib(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::Xcb(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::Wayland(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::Drm(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::Gbm(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                let pfd = PIXELFORMATDESCRIPTOR {
                    nSize: size_of::<PIXELFORMATDESCRIPTOR>() as WORD,
                    nVersion: 1,
                    dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER, // Flags
                    iPixelType: PFD_TYPE_RGBA,
                    cColorBits: 32,
                    cRedBits: 0,
                    cRedShift: 0,
                    cGreenBits: 0,
                    cGreenShift: 0,
                    cBlueBits: 0,
                    cBlueShift: 0,
                    cAlphaBits: 0,
                    cAlphaShift: 0,
                    cAccumBits: 0,
                    cAccumRedBits: 0,
                    cAccumGreenBits: 0,
                    cAccumBlueBits: 0,
                    cAccumAlphaBits: 0,
                    cDepthBits: 24,
                    cStencilBits: 8,
                    cAuxBuffers: 0,
                    iLayerType: PFD_MAIN_PLANE as BYTE,
                    bReserved: 0,
                    dwLayerMask: 0,
                    dwVisibleMask: 0,
                    dwDamageMask: 0,
                };
                let attrib_list = [
                    WGL_DRAW_TO_WINDOW_ARB,
                    GL_TRUE as i32,
                    WGL_SUPPORT_OPENGL_ARB,
                    GL_TRUE as i32,
                    WGL_DOUBLE_BUFFER_ARB,
                    GL_TRUE as i32,
                    WGL_PIXEL_TYPE_ARB,
                    WGL_TYPE_RGBA_ARB,
                    WGL_COLOR_BITS_ARB,
                    32,
                    WGL_DEPTH_BITS_ARB,
                    24,
                    WGL_STENCIL_BITS_ARB,
                    8,
                    0,
                ];
                let func;
                let hdc;
                let ctx;
                unsafe {
                    hdc = GetDC(windows_sys::Win32::Foundation::HWND::from(handle.hwnd));
                    let pixel_format = ChoosePixelFormat(
                        hdc,
                        addr_of!(pfd)
                            as *const windows_sys::Win32::Graphics::OpenGL::PIXELFORMATDESCRIPTOR,
                    );

                    SetPixelFormat(
                        hdc,
                        pixel_format,
                        addr_of!(pfd)
                            as *const windows_sys::Win32::Graphics::OpenGL::PIXELFORMATDESCRIPTOR,
                    );

                    let old_ctx = wgl::CreateContext(hdc as wgl::types::HDC);
                    wgl::MakeCurrent(hdc as wgl::types::HDC, old_ctx);
                    let att = [
                        wgl::extra::CONTEXT_MAJOR_VERSION_ARB,
                        3,
                        wgl::extra::CONTEXT_MINOR_VERSION_ARB,
                        1,
                        wgl::extra::CONTEXT_FLAGS_ARB,
                        0,
                        wgl::extra::CONTEXT_PROFILE_MASK_ARB,
                        wgl::extra::CONTEXT_CORE_PROFILE_BIT_ARB,
                        0,
                    ];
                    func = WGLARBFunctions::load();
                    ctx = (func.wglCreateContextAttribsARB)(
                        hdc as wgl::extra::types::HDC,
                        null_mut(),
                        &att,
                    );
                    info!("Created context");
                    wgl::DeleteContext(old_ctx);
                    wgl::MakeCurrent(hdc as wgl::types::HDC, ctx);
                    info!("Loading OpenGL function pointer");
                    gl::load_with(|s| {
                        debug!("Function: {}", s);
                        let addr = CString::new(s.as_bytes()).unwrap();
                        let addr = addr.as_ptr();

                        let name = OsStr::new("opengl32.dll")
                            .encode_wide()
                            .chain(Some(0).into_iter())
                            .collect::<Vec<_>>();

                        let lib = unsafe { LoadLibraryW(name.as_ptr()) };

                        unsafe {
                            let p = wgl::GetProcAddress(addr) as *const core::ffi::c_void;
                            if !p.is_null() {
                                return p;
                            }
                            windows_sys::Win32::System::LibraryLoader::GetProcAddress(
                                lib,
                                addr as PCSTR,
                            )
                            .unwrap() as *const _
                        }
                    });
                    info!("Successfully loaded OpenGL function pointer");

                    (func.wglSwapIntervalEXT)(1);
                    info!("VSYNC is enabled.");

                    let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
                    let renderer = CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8);
                    let vendor = CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8);
                    info!("OpenGL {}", version.to_str().unwrap());
                    info!("{}", renderer.to_str().unwrap());
                    info!("{}", vendor.to_str().unwrap());

                    Self {
                        func,
                        hdc: hdc as wgl::extra::types::HDC,
                        ctx,
                    }
                }
            }
            raw_window_handle::RawWindowHandle::WinRt(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::Web(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::WebCanvas(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::WebOffscreenCanvas(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::AndroidNdk(_) => {
                panic!("このプラットフォームには対応していません")
            }
            raw_window_handle::RawWindowHandle::Haiku(_) => {
                panic!("このプラットフォームには対応していません")
            }
            _ => panic!("このプラットフォームには対応していません"),
        }
    }

    pub fn swap(&self) {
        unsafe {
            SwapBuffers(self.hdc as windows_sys::Win32::Graphics::Gdi::HDC);
        }
    }
}
