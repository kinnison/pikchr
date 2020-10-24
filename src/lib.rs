/// Pikchr binding
use libc::{c_char, c_int, c_uint, c_void, free};
use std::ffi::{CStr, CString};
use std::fmt;
use std::ops::Deref;

pub mod raw {
    use libc::{c_char, c_int, c_uint};

    extern "C" {
        /// The main interface.  Invoke this routine to translate PIKCHR source
        /// text into SVG. The SVG is returned in a buffer obtained from malloc().
        /// The caller is responsible for freeing the buffer.
        ///
        /// If an error occurs, *pnWidth is filled with a negative number and
        /// the return buffer contains error message text instead of SVG.  By
        /// default, the error message is HTML encoded.  However, error messages
        /// come out as plaintext if the PIKCHR_PLAINTEXT_ERRORS flag is included
        /// as one of the bits in the mFlags parameter.
        ///
        /// - `zText`: Input PIKCHR source text.  zero-terminated
        /// - `zClass`: Add class="%s" to <svg> markup
        /// - `mFlags`: Flags used to influence rendering behavior
        /// - `pnWidth`: OUT: Write width of <svg> here, if not NULL
        /// - `pnHeight`: OUT: Write height here, if not NULL
        #[allow(non_snake_case)]
        pub fn pikchr(
            zText: *const c_char,
            zClass: *const c_char,
            mFlags: c_uint,
            pnWidth: *mut c_int,
            pnHeight: *mut c_int,
        ) -> *mut c_char;
    }

    /// Include PIKCHR_PLAINTEXT_ERRORS among the bits of mFlags on the 3rd
    /// argument to pikchr() in order to cause error message text to come out
    /// as text/plain instead of as text/html
    pub const PIKCHR_PLAINTEXT_ERRORS: c_uint = 0x0001;
}

/// Flags for converting pikchr source
///
/// You can construct a default set of flags using the [`std::default::Default`] trait
///
/// The default flags will generate plain text errors
#[derive(Copy, Clone)]
pub struct PikchrFlags {
    plain_errors: bool,
}

impl PikchrFlags {
    /// Return whether or not plain text errors will be generated
    ///
    /// ```
    /// # use pikchr::PikchrFlags;
    /// let flags = PikchrFlags::default();
    /// assert!(flags.plain_errors())
    /// ```
    pub fn plain_errors(&self) -> bool {
        self.plain_errors
    }

    /// Request plain text errors be generated
    ///
    /// ```
    /// # use pikchr::PikchrFlags;
    /// let mut flags = PikchrFlags::default();
    /// flags.generate_plain_errors();
    /// assert!(flags.plain_errors());
    /// ```
    pub fn generate_plain_errors(&mut self) -> &mut PikchrFlags {
        self.plain_errors = true;
        self
    }

    /// Request help encoded errors be generated
    ///
    /// ```
    /// # use pikchr::PikchrFlags;
    /// let mut flags = PikchrFlags::default();
    /// flags.generate_html_errors();
    /// assert!(!flags.plain_errors());
    /// ```
    pub fn generate_html_errors(&mut self) -> &mut PikchrFlags {
        self.plain_errors = false;
        self
    }
}

impl Into<c_uint> for PikchrFlags {
    fn into(self) -> c_uint {
        if self.plain_errors {
            raw::PIKCHR_PLAINTEXT_ERRORS
        } else {
            0
        }
    }
}

impl std::default::Default for PikchrFlags {
    fn default() -> Self {
        Self { plain_errors: true }
    }
}

/// A rendered pikchr diagram
///
/// Pikchr renders diagrams as SVG.  This SVG is a given width
/// and height.  The Pikchr derefs to the SVG string, or you
/// can access it explicitly.  The width and height are accessible
/// as plain numbers.
pub struct Pikchr {
    rendered: *const c_char,
    width: c_int,
    height: c_int,
}

impl Drop for Pikchr {
    fn drop(&mut self) {
        if self.rendered.is_null() {
            unsafe {
                free(self.rendered as *mut c_void);
            }
            self.rendered = std::ptr::null();
        }
    }
}

impl Deref for Pikchr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        // We're assuming a Pikchr instance can only
        // be constructed from valid utf8 and thus can
        // only contain valid utf8
        unsafe {
            let cstr = CStr::from_ptr(self.rendered);
            std::str::from_utf8_unchecked(cstr.to_bytes())
        }
    }
}

impl fmt::Display for Pikchr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self)
    }
}

impl Pikchr {
    /// Render some input pikchr source as an SVG
    ///
    /// You can convert arbitrary pikchr source into an SVG using this function.
    /// The class name is optional, and the flags field controls the generation
    /// of errors.  Since pikchr does not have a structured error format, the
    /// returned error is simply a string.
    ///
    /// ```
    /// # use pikchr::{Pikchr, PikchrFlags};
    /// let image = Pikchr::render(r#"
    /// arrow right 200% "Markdown" "Source"
    /// box rad 10px "Markdown" "Formatter" "(markdown.c)" fit
    /// arrow right 200% "HTML+SVG" "Output"
    /// arrow <-> down 70% from last box.s
    /// box same "Pikchr" "Formatter" "(pikchr.c)" fit"#,
    ///      None, PikchrFlags::default())
    ///     .unwrap();
    /// assert!(image.contains("<svg"))
    /// ```
    pub fn render(source: &str, class: Option<&str>, flags: PikchrFlags) -> Result<Pikchr, String> {
        let mut width: c_int = 0;
        let mut height: c_int = 0;
        let source = CString::new(source).map_err(|e| format!("{:?}", e))?;
        let res: *mut c_char = unsafe {
            raw::pikchr(
                source.as_ptr() as *const c_char,
                class
                    .map(|s| s.as_ptr() as *const c_char)
                    .unwrap_or(std::ptr::null()),
                flags.into(),
                &mut width as *mut c_int,
                &mut height as *mut c_int,
            )
        };
        if width < 0 {
            let err = unsafe { CStr::from_ptr(res) };
            let err = err.to_bytes();
            let err = String::from_utf8_lossy(err).into_owned();
            unsafe {
                free(res as *mut c_void);
            }
            Err(err)
        } else {
            Ok(Pikchr {
                rendered: res,
                width,
                height,
            })
        }
    }

    /// Retrieve the width of this Pikchr
    ///
    /// ```
    /// # use pikchr::{Pikchr, PikchrFlags};
    /// # let pic = Pikchr::render(r#"arrow right 200% "Markdown" "Source""#,
    /// #     None, PikchrFlags::default()).unwrap();
    /// println!("Picture is {} pixels wide", pic.width());
    /// ```
    pub fn width(&self) -> isize {
        self.width as isize
    }

    /// Retrieve the height of this Pikchr
    ///
    /// ```
    /// # use pikchr::{Pikchr, PikchrFlags};
    /// # let pic = Pikchr::render(r#"arrow right 200% "Markdown" "Source""#,
    /// #     None, PikchrFlags::default()).unwrap();
    /// println!("Picture is {} pixels tall", pic.height());
    /// ```
    pub fn height(&self) -> isize {
        self.height as isize
    }

    /// Retrieve the rendered pikchr (same as dereferencing)
    ///
    /// ```
    /// # use pikchr::{Pikchr, PikchrFlags};
    /// # let pic = Pikchr::render(r#"arrow right 200% "Makdown" "Source""#,
    /// #     None, PikchrFlags::default()).unwrap();
    /// println!("Picture content:\n{}", pic.rendered());
    /// ```
    pub fn rendered(&self) -> &str {
        &*self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validate_diagram() {
        const SOURCE: &str = r#"arrow right 200% "Markdown" "Source""#;
        const OUTPUT: &str = r#"<svg xmlns='http://www.w3.org/2000/svg' viewBox="0 0 152.64 47.88">
<polygon points="146,23 134,28 134,19" style="fill:rgb(0,0,0)"/>
<path d="M2,23L140,23"  style="fill:none;stroke-width:2.16;stroke:rgb(0,0,0);" />
<text x="74" y="12" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">Markdown</text>
<text x="74" y="35" text-anchor="middle" fill="rgb(0,0,0)" dominant-baseline="central">Source</text>
</svg>
"#;
        let flags = PikchrFlags::default();
        let p = Pikchr::render(SOURCE, None, flags).unwrap();
        assert_eq!(OUTPUT, p.rendered());
    }
}
