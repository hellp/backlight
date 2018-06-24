#[macro_use]
extern crate structopt;

mod lib;

use lib::Brightness;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "backlight", about = "Set the backlight to a certain percentage.")]
struct Opt {
    /// new brightness
    #[structopt(default_value = "75")]
    brightness: i32,
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();
    let br: Brightness = Default::default();
    if opt.brightness < 1 || opt.brightness > 100 {
        panic!("invalid range");
    }
    br.set_percent(opt.brightness)?;
    Ok(())
}
