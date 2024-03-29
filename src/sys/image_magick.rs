use std::path::Path;
use std::process::Command;

use tempfile::NamedTempFile;

use super::super::errors::Result;

pub fn resize(src: &Path, width: u16, height: u16, target: &Path) -> Result<()> {
    run(&format!(
        "convert -resize {width}x{height}! {src} {target}",
        src = src.display(),
        target = target.display(),
        width = width,
        height = height
    ))
}

pub fn merge(back: &Path, cover: &Path, target: &Path) -> Result<()> {
    let tmp = NamedTempFile::new()?;
    run(&format!(
        "convert -resize $(identify -ping -format '%wx%h!' {back}) {cover} {tmp} && convert {back} -compose over {tmp} -composite {target}",
        cover = cover.display(),
        tmp = tmp.path().display(),
        target = target.display(),
        back = back.display()
    ))
}

pub fn rotate(src: &Path, degrees: i8, target: &Path) -> Result<()> {
    run(&format!(
        "convert -rotate '{degrees}' {src} {target}",
        src = src.display(),
        target = target.display(),
        degrees = degrees
    ))
}

fn run(cmd: &str) -> Result<()> {
    debug!("{}", cmd);
    let out = Command::new("sh").arg("-c").arg(cmd).output()?;
    debug!("{:?}", out);
    Ok(())
}
