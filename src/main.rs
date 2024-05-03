use std::io;

use qrcode::QrCode;
use image::Luma;

fn main() -> anyhow::Result<()> {
    let mut line = String::new();

    io::stdin().read_line(&mut line)?;
    
    let code = QrCode::new(line.trim_end())?;
    let image = code.render::<Luma<u8>>().build();
    
    image.save("qrcode.png")?;
    
    Ok(())
}
