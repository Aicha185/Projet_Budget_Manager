fn main() {
    println!("cargo:rustc-link-search=native=C:\\Users\\serab\\Desktop\\sqlite");
    println!("cargo:rustc-link-lib=static=sqlite3");
}