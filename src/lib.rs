pub mod opts;
pub mod process;
pub use opts::{Opts, SubCommand};
pub use process::process_csv;
pub use process::process_genpass;
