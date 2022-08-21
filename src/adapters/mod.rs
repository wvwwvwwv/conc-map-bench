pub use self::{dashmap::DashMapTable, flurry::FlurryTable, scc::SccIndex, scc::SccMap};

mod dashmap;
mod flurry;
mod scc;

type Value = u32;
