use std::io;
use std::process::ExitCode;

use rs_fs_ext2int_lossy::stdin2paths2exts2ints2stdout_default;

fn sub() -> Result<(), io::Error> {
    stdin2paths2exts2ints2stdout_default()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
