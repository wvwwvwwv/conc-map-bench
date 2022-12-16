pub use self::{dashmap::DashMapTable, scc::SccIndex, scc::SccMap};

mod dashmap;
mod scc;

type Value = u32;
