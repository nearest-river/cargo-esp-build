use std::{
  io,
  env,
  sync::LazyLock,
  process::Command,
};


static TARGET: &str="xtensa-esp8266-none-elf";
static RUSTFLAGS: LazyLock<String>=LazyLock::new(|| match env::var("RUSTFLAGS") {
  Err(_)=> "-C link-arg=-nostartfiles -C link-arg=-Wl,-Tlink.x".to_owned(),
  Ok(var)=> var
});

fn main()-> io::Result<()> {
  let args=Args::new();
  let mut process=Command::new("rustup");

  process.env("RUSTFLAGS",&*RUSTFLAGS);
  process.args(["run","esp","cargo"]);

  if args.flash {
    process.args(["espflash","flash"]);
  } else {
    process.arg("build");
  }

  process.args(args.args);
  process.args(["--target",TARGET]);

  let status=process.status()?;
  if !status.success() {
    let code=status.code().unwrap_or(io::ErrorKind::Other as _);
    return Err(io::Error::from_raw_os_error(code));
  }

  Ok(())
}

#[derive(Default)]
struct Args {
  flash: bool,
  args: Vec<String>
}

impl Args {
  fn new()-> Self {
    let mut this=Self::default();
    let mut subcommand_occured=false;

    for arg in env::args().skip(1) {
      match arg.as_str() {
        "--flash"=> this.flash=true,
        arg if arg=="esp-build" && !subcommand_occured=> subcommand_occured=true,
        _=> this.args.push(arg),
      }
    }

    this
  }
}




