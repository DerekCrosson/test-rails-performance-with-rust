#[macro_use]
extern crate helix;

ruby! {
    class ImageStamper {
        def hello() {
            println!("Hello from image_stamper!");
        }
    }
}
