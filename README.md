# Freetype Rust

Unsafe rust bindings for freetype. Bindings are generated
using bindgen.

## Example

```rust
extern crate freetype_sys;
use std::path::{Path};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::ffi::CStr;

fn main() {
    unsafe {
        let mut library = ptr::null_mut();
        let error = freetype_sys::FT_Init_FreeType(&mut library);
        assert_eq!(error, freetype_sys::FT_Err_Ok as i32);
        
        let mut face = ptr::null_mut();
        let path = ffi::CString::new("path to font file").unwrap();
        let error = freetype_sys::FT_New_Face(library, path.as_ptr(), 0, &mut face);
        assert_eq!(error, freetype_sys::FT_Err_Ok as i32);
        
        let error = freetype_sys::FT_Set_Pixel_Sizes(face, 0, 15);
        assert_eq!(error, freetype_sys::FT_Err_Ok as i32);
        
        // ...
    }
}
```

## License
Apache License 2.0. See `LICENSE.md`.