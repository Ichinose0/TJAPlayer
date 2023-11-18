extern crate gl_generator;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut wgl = File::create(&Path::new(&dest).join("wgl.rs")).unwrap();
    let mut wgl_extra = File::create(&Path::new(&dest).join("wgl_extra_bindings.rs")).unwrap();
    let mut gl = File::create(&Path::new(&dest).join("gl.rs")).unwrap();

    Registry::new(Api::Wgl, (1, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(gl_generator::StaticGenerator, &mut wgl)
        .unwrap();
    Registry::new(
        Api::Wgl,
        (1, 0),
        Profile::Core,
        Fallbacks::All,
        [
            "WGL_ARB_context_flush_control",
            "WGL_ARB_create_context",
            "WGL_ARB_create_context_no_error",
            "WGL_ARB_create_context_profile",
            "WGL_ARB_create_context_robustness",
            "WGL_ARB_extensions_string",
            "WGL_ARB_framebuffer_sRGB",
            "WGL_ARB_multisample",
            "WGL_ARB_pixel_format",
            "WGL_ARB_pixel_format_float",
            "WGL_EXT_create_context_es2_profile",
            "WGL_EXT_extensions_string",
            "WGL_EXT_framebuffer_sRGB",
            "WGL_EXT_swap_control",
        ],
    )
    .write_bindings(gl_generator::StructGenerator, &mut wgl_extra)
    .unwrap();
    Registry::new(Api::Gl, (4, 6), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut gl)
        .unwrap();
}
