// TODOs
// - find protos automatically

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir : "src/protos",
        input : &["protocols/Common.proto", "protocols/AuthServer.proto"],
        includes : &[ "protocols" ],
        customize : protoc_rust::Customize {
            ..Default::default()
        },
    }).expect("protoc run panic!");
}