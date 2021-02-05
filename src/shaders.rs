macro_rules! declare_shaders {
    ($name: ident) => {
        pub mod $name {
            pub const VERTEX: &str =
                include_str!(concat!("shaders/", stringify!($name), "/vertex.glsl"));
            pub const FRAGMENT: &str =
                include_str!(concat!("shaders/", stringify!($name), "/fragment.glsl"));
            pub const GEOMETRY: Option<&str> = None;
        }
    };
    ($name: ident +geometry) => {
        pub mod $name {
            pub const VERTEX: &str =
                include_str!(concat!("shaders/", stringify!($name), "/vertex.glsl"));
            pub const FRAGMENT: &str =
                include_str!(concat!("shaders/", stringify!($name), "/fragment.glsl"));
            pub const GEOMETRY: Option<&str> = Some(include_str!(concat!(
                "shaders/",
                stringify!($name),
                "/geometry.glsl"
            )));
        }
    };
}

declare_shaders!(ray + geometry);
declare_shaders!(outline);
