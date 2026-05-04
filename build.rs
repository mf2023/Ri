
fn main() {
    println!("cargo:warning=Checking features...");
    
    if cfg!(feature = "oqs") {
        println!("cargo:warning=oqs feature is enabled!");
    } else {
        println!("cargo:warning=oqs feature is NOT enabled!");
    }
    
    if cfg!(feature = "protocol") {
        println!("cargo:warning=protocol feature is enabled!");
    } else {
        println!("cargo:warning=protocol feature is NOT enabled!");
    }
    
    if cfg!(feature = "sm-crypto") {
        println!("cargo:warning=sm-crypto feature is enabled!");
    } else {
        println!("cargo:warning=sm-crypto feature is NOT enabled!");
    }
}
