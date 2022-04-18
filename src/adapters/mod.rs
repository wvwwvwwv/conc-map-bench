pub use self::{
    btreemap::RwLockBTreeMapTable, chashmap::CHashMapTable, contrie::ContrieTable,
    crossbeam_skiplist::CrossbeamSkipMapTable, dashmap::DashMapTable, evmap::EvmapTable,
    flurry::FlurryTable, scc::SccTable, std::RwLockStdHashMapTable,
};

mod btreemap;
mod chashmap;
mod contrie;
mod crossbeam_skiplist;
mod dashmap;
mod evmap;
mod flurry;
mod scc;
mod std;

type Value = u32;
