#![deny(warnings)]
fn main() {
    for path in glob::glob("services/KRPC.*.json").unwrap().filter_map(Result::ok) {
        println!("cargo:rerun-if-changed={}", path.display());

    }

    krpc_mars_terraformer::run("services/", "src/")
        .expect("Could not terraform Mars :(");
}
