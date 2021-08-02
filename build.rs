#[allow(unused_imports)]
use config_struct::{Error, StructOptions, create_struct, SerdeSupport, DynamicLoading};
use config_struct::{IntSize, FloatSize};

fn main() -> Result<(), Error> {
    config_struct::create_struct(
        "rnginx_conf.toml",
        "src/config_parser/config_structs.rs",
        &StructOptions {
            serde_support: SerdeSupport::Yes,
            generate_load_fns: true,
            dynamic_loading:DynamicLoading::Always,
            default_int_size:IntSize::I32,
            default_float_size:FloatSize::F32,
            .. StructOptions::default()
        })
}