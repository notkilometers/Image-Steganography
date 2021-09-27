# Image-Steganography
hiding string inside alpha layer of image

## Usage:
```Rust
  fn main() {
    encoder::encode_image("C:/Users/Name/12345.jpg", "C:/Users/Name/out.png", "Sample Text");
    let msg = encoder::decode_image("C:/Users/Name/out.png");
    println!("{}", msg);
  }
```
