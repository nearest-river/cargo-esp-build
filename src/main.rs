
use clap::Parser;
use std::{
  env, io, process::{self, Command}, sync::LazyLock
};


#[derive(Parser)]
struct App {
  #[arg(long)]
  release: bool,
  #[arg(long)]
  flash: bool,
  #[arg(long)]
  dry_run: bool
}

static TARGET: &str="xtensa-esp8266-none-elf";
static RUSTFLAGS: LazyLock<String>=LazyLock::new(|| match env::var("RUSTFLAGS") {
  Ok(var)=> var,
  _=> "-C link-arg=-nostartfiles -C link-arg=-Wl,-Tlink.x".to_owned()
});

fn main()-> io::Result<()> {
  let app=App::parse();
  let mut process=Command::new("rustup");

  process.env("RUSTFLAGS",&*RUSTFLAGS);
  process.args(["run","esp","cargo"]);

  if app.flash {
    process.args(["espflash","flash"]);
  } else {
    process.arg("build");
  }

  if app.release {
    process.arg("--release");
  }

  process.args(["--target",TARGET]);

  let status=process.status()?;
  if status.success() {
    return Ok(());
  }

  process::exit(status.code().unwrap_or(1))
}





