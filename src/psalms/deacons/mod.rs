pub mod file;

pub mod prelude {
    pub use crate::psalms::deacons::file::{
        destination::FileDeacon,
        destination::FileDestination,
        source::{FileSource, Source, FileSourceDeacon},
    };
}

// prepare for future use
// not sure yet about the structure/function of deacons
trait Deacon {
    fn has_changed() -> bool;
}
