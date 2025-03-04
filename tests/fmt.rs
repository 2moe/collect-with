use std::io;

use testutils::{
  os_cmd::{Runner, presets::CargoFmt},
  traits::Pipe,
};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  CargoFmt::default()
    .pipe(Runner::from)
    .run()
}
