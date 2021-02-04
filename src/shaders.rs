macro_rules! declare_shaders {
    ($name: ident) => {
        pub mod $name {
            pub const VERTEX: &str =
                include_str!(concat!("shaders/", stringify!($name), "/vertex.ocl"));
            pub const FRAGMENT: &str =
                include_str!(concat!("shaders/", stringify!($name), "/fragment.ocl"));
            pub const GEOMETRY: Option<&str> = None;
        }
    };
    ($name: ident +geometry) => {
        pub mod $name {
            pub const VERTEX: &str =
                include_str!(concat!("shaders/", stringify!($name), "/vertex.ocl"));
            pub const FRAGMENT: &str =
                include_str!(concat!("shaders/", stringify!($name), "/fragment.ocl"));
            pub const GEOMETRY: Option<&str> = Some(include_str!(concat!(
                "shaders/",
                stringify!($name),
                "/geometry.ocl"
            )));
        }
    };
}

declare_shaders!(ray);
declare_shaders!(outline);
