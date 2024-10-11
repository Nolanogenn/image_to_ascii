#![allow(warnings)]
use std::io::Write;
use std::fs::File;
use std::io::Cursor;
use image::{self, imageops::*};
use imageproc::{filter::*};

fn print_type<T>(_: &T) -> String {
   return std::any::type_name::<T>().into() 
}

fn main(img: image::DynamicImage) -> std::io::Result<()>{
    let mut gray_ramp_70: Vec<&str> = vec![
        "$",",","@","B","%","8","&","W",
        "M","#","*","o","a","h","k","b",
        "d","p","q","w","m","Z","O","0",
        "Q","L","C","J","U","Y","X","z",
        "c","v","u","n","x","r","j","f",
        "t","/",r"\","|","(",")","1","{",
        "}","[","]","?","-","_","+","~",
        "<",">","i","!","l","I",";",":",
        ",","^","`","`","'",".",];
    let mut gray_ramp_10: Vec<&str> = vec![
        "@","?","0","P", "o", "x", ":", ".", "`"," "  
    ];
    gray_ramp_10.reverse(); 
    let h = img.height();
    let w = img.width();
    let mut img = img.grayscale();
    let mut img = img.resize(w/15, h/15, FilterType::Triangle);
    let h = img.height();
    let w = img.width();
    let h_usize = img.height() as usize;
    let w_usize = img.width() as usize;
    let mut ascii_str = String::new();
    let pixel_values = &img.into_bytes();
    for i in 0..pixel_values.len() {
        let mut ascii_i = pixel_values[i] as f64 /255.0;
        let mut ascii_i = (ascii_i * 10.0)-1.0;
        let mut ascii_i = ascii_i.ceil();
        let ascii_char = gray_ramp_10[ascii_i as usize];
        ascii_str.push_str(ascii_char);
        if (i as u32) % w == 0 {
            ascii_str.push_str("\n");
        }
    };
    let mut file = File::create("src/imgs/ascii.txt")?; 
    file.write_all(ascii_str.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let path: String = "src/imgs/iomoka.jpg".into();
        let img = image::open(path).unwrap();
        let res = main(img);
    }
}
