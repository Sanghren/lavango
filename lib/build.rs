use std::env;
use std::path::PathBuf;

fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("rpcchainvm.bin"))
        .compile(&["proto/vm.proto"], &["proto"])
        .unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld.bin"))
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();

}
