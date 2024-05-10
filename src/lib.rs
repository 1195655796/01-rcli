pub mod cli;
pub mod process;
pub use cli::{Base64Format, Base64SubCommand, Opts, SubCommand};
pub use process::process_csv;
pub use process::process_genpass;
