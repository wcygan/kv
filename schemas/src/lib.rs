pub use tonic;

pub mod kv {
    include!(concat!(env!("OUT_DIR"), "/kv.rs"));
}
