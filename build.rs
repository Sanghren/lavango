use std::env;
use std::path::PathBuf;

fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("lavango.bin"))
        .compile(&["proto/vm.proto"], &["proto"])
        .unwrap();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("genesisencoder.bin"))
        .compile(&["proto/genesisencoder.proto"], &["proto"])
        .unwrap();

}
