
use clap::Parser;
use std::{
  io,
  env,
  sync::LazyLock,
  process::Command,
};


#[derive(Parser)]
struct App {
  #[arg(long)]
  release: bool,
  #[arg(long)]
  flash: bool,
  #[arg(long)]
  dry_run: bool,
}

static TARGET: &str="xtensa-esp8266-none-elf";
static RUSTFLAGS: LazyLock<String>=LazyLock::new(|| match env::var("RUSTFLAGS") {
  Err(_)=> "-C link-arg=-nostartfiles -C link-arg=-Wl,-Tlink.x".to_owned(),
  Ok(var)=> var
});

fn main()-> io::Result<()> {
  let args=env::args().collect::<Vec<_>>();
  let trailing_idx=idx_of(&args,"--");
  let args=match trailing_idx {
    Some(trailing)=> &args[1..trailing],
    _=> &args[1..]
  };

  let app=App::parse_from(args);
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
  if let Some(trailing_idx)=trailing_idx {
    if trailing_idx+1<args.len() {
      process.args(&args[trailing_idx+1..]);
    }
  }

  if app.dry_run {
    println!("{:#?}",process);
    return Ok(());
  }

  let status=process.status()?;
  if !status.success() {
    let code=status.code().unwrap_or(1);
    return Err(io::Error::from_raw_os_error(code));
  }

  Ok(())
}

fn idx_of<T,U: PartialEq<T>>(slice: &[T],element: U)-> Option<usize> {
  let (idx,_)=slice.iter()
  .enumerate()
  .find(|(_,entity)| element.eq(entity))?;

  Some(idx)
}






