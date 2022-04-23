pub use self::{dashmap::DashMapTable, flurry::FlurryTable, scc::SccTable};

mod dashmap;
mod flurry;
mod scc;

type Value = u32;
