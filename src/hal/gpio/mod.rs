pub mod button;

// ORANGE PI: http://www.orangepi.org/Docs/WiringPi.html
// NANO PI: http://wiki.friendlyarm.com/wiki/index.php/WiringNP:_NanoPi_NEO/NEO2/Air_GPIO_Programming_with_C
// RASPBERRY PI: http://wiringpi.com/

use super::super::errors::Result;

pub fn set(_id: u8, _on: bool) -> Result<()> {
    // TODO
    Ok(())
}

pub fn get(_id: u8) -> Result<bool> {
    // TODO
    Ok(false)
}
