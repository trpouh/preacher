pub mod file;

// prepare for future use
// not sure yet about the structure/function of deacons
trait Deacon {
    fn has_changed() -> bool;
}