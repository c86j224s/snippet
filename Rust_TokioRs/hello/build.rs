extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir : "src/protos",
        input : &["protos/sample.proto", "protos/ping.proto"],
        includes : &["protos"],
        customize : protoc_rust::Customize {
            ..Default::default()
        },
    }).expect("protoc run panic");
}