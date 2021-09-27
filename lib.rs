extern crate image;

use image::{GenericImageView, ImageBuffer, Rgba, RgbaImage};

pub fn encode_image(img_path : &str, save_path : &str, message: &str) {
    // reads image in from path
    let img_in = image::open(img_path).unwrap();
                                                
    // gets dimensions
    let (width, height) = img_in.dimensions();

    // instantiates new object of same size
    let mut img_out: RgbaImage = ImageBuffer::new(width, height);

    // instantiate vector to hold char values
    let mut chars = Vec::<u8>::new();

    // push all chars in message to vector
    for c in message.chars() {
        chars.push(c as u8);
    }

    // reverse to put in order
    chars.reverse();

    // iterate through each pixel of both images
    for (pixel, pixel2) in img_in.pixels().zip(img_out.enumerate_pixels_mut()){
        // get char 
        let c = chars.pop();

        // if there is another char to add
        if c.is_some() {
            // set pixel to same color w/ 133+char value on alpha layer
            *pixel2.2 = Rgba([pixel.2[0], pixel.2[1], pixel.2[2], 133+c.unwrap()]);
        } 
        // if there is no other chars
        else {
            // set pixel to same as before
            *pixel2.2 = Rgba([pixel.2[0], pixel.2[1], pixel.2[2], 250]);
        }
    }

    // save output image to path
    img_out.save(save_path).unwrap();
}

pub fn decode_image(img_path: &str) -> String {
    // instantiate new string
    let mut str = String::new();
    
    // open image from path
    let img_in = image::open(img_path).unwrap();

    // for each pixel in image
    for pixel in img_in.pixels() {
        // read in u8 value from pixel's alpha layer
        let pixel_value = pixel.2[3];
        // read in as char - offset 
        let c = ( pixel_value - 133 ) as char;

        // if pixel value isn't default (is a char that was written to alpha)
        if pixel_value != 250  {
            // concat to string
            str += &c.to_string();
        }
    };

    // return str
    str
}

