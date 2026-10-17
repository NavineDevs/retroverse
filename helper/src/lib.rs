//! RetroVerse helper DLL by Navine. A minimal library with a few exported C functions.

/// Returns the helper's version as a number, for checking the DLL loads.
#[no_mangle]
pub extern "C" fn retroverse_helper_version() -> u32 {
    1
}

/// Adds two numbers. Placeholder export to show calls cross the DLL boundary.
#[no_mangle]
pub extern "C" fn retroverse_helper_add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}
