fn main() {
    match slint_build::compile("ui/appwindow.slint"){
        Ok(_)=>println!("Build OK"),
        Err(e)=>println!("{:#?}",e),
    }
}
