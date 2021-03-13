//! xflags is a bit jank still, but pretty nice compared to pico_args, and
//! compiles a billion times faster than clap/structopt.
//!
//! When you modify stuff in the args_parser block, you'll get compile errors.
//! to fix these:
//! 1. run `env XFLAGS_DUMP= cargo build` in the shell.
//! 2. this will error, but should spit out a block of code delimited by
//!    "generated start" and "generated end" comments. (if not, you probably
//!    have an error message about your usage of the macro, fix and retry)
//! 3. Copy the generated code, and replace the bottom of this file with it.
//!
//! Also, rust-analyzer says this file has a "unresolved macro call". I don't
//! know why, just ignore it — it goes away if you close this file.

#![allow(dead_code)]

xflags::args_parser! {
    /// Run custom build command.
    cmd xtask {
        optional -v, --verbose

        default cmd help {
            /// Print help information.
            optional -h, --help
        }
        /// Run lints the way we'd run it in CI
        cmd lint {}
        /// Run tests the way we'd run them in CI
        cmd test {}

        /// produce bindings using bindgen (must have bindgen installed)
        cmd bindgen {
            optional --cimgui-path cimgui_path: String
            optional --output-path output_path: String
            optional --wasm-import-name wasm_import_name: String
        }
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env XFLAGS_DUMP= cargo build` to regenerate.
#[derive(Debug)]
pub struct Xtask {
    pub verbose: bool,
    pub subcommand: XtaskCmd,
}

#[derive(Debug)]
pub enum XtaskCmd {
    Help(Help),
    Lint(Lint),
    Test(Test),
    Bindgen(Bindgen),
}

#[derive(Debug)]
pub struct Help {
    pub help: bool,
}

#[derive(Debug)]
pub struct Lint {}

#[derive(Debug)]
pub struct Test {}

#[derive(Debug)]
pub struct Bindgen {
    pub cimgui_path: Option<String>,
    pub output_path: Option<String>,
    pub wasm_import_name: Option<String>,
}

impl Xtask {
    pub const HELP: &'static str = Self::_HELP;

    pub fn from_env() -> xflags::Result<Self> {
        let mut p = xflags::rt::Parser::new_from_env();
        Self::_parse(&mut p)
    }

    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        let mut p = xflags::rt::Parser::new(args);
        Self::_parse(&mut p)
    }
}
// generated end
