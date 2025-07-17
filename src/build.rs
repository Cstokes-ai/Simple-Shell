// this is to build the appplcation

fn main() {
    let mut config = winres::WindowsResource::new();
    config.set_icon("phot.ico");
    config.compile().unwrap();

}