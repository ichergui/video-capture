//! Prints detailed device information.

use std::{env, path::Path};
use anyhow::anyhow;
use linuxvideo::Device;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut args = env::args_os().skip(1);
    let path = args.next().ok_or_else(|| anyhow!("usage: info <device>"))?;
    let device = Device::open(Path::new(&path))?;
    let caps = device.capabilities()?;

    println!("card: {}", caps.card());
    println!("driver: {}", caps.driver());
    println!("bus info: {}", caps.bus_info());
    println!("all capabilities:    {:?}", caps.all_capabilities());
    println!("avail. capabilities: {:?}", caps.device_capabilities());

    Ok(())
}
