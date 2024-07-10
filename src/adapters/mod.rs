pub use self::{dashmap::DashMapTable, papaya::PapayaMap, scc::SccIndex, scc::SccMap};

mod dashmap;
mod papaya;
mod scc;

type Value = u32;
