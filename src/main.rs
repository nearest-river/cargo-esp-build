
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
  let app=App::parse_from(&args[1..]);
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
  if let Some(args)=extract_trailing_args(&args) {
    process.args(args);
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


fn extract_trailing_args<S: AsRef<str>>(args: &[S])-> Option<&[S]> {
  let (idx,_)=args.iter()
  .enumerate()
  .find(|(_,arg)| arg.as_ref()=="--")?;

  Some(&args[idx+1..])
}





