use std::io;

use testutils::{
  get_pkg_name,
  os_cmd::{Runner, presets::CargoDoc},
  traits::Pipe,
};

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  CargoDoc::default()
    .with_pkg(get_pkg_name!())
    .with_open(false)
    .pipe(Runner::from)
    .run()
}
