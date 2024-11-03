pub use self::{
    async_dashmap::AsyncDashMapTable,
    btreemap::ParkingLotRwLockBTreeMapTable,
    btreemap::StdRwLockBTreeMapTable,
    chashmap::CHashMapTable,
    contrie::ContrieTable,
    crossbeam_skiplist::CrossbeamSkipMapTable,
    dashmap::DashMapTable,
    /* evmap::EvmapTable, */ flurry::FlurryTable,
    scc::SccMapTable,
    std::ParkingLotRwLockStdHashMapTable,
    std::StdRwLockStdHashMapTable,
    whirlwind::WhirlwindShardedMapTable,
    // whirlwind_custom::WhirlwindCustomMapTable,
};

mod btreemap;
mod chashmap;
mod contrie;
mod crossbeam_skiplist;
mod dashmap;
// mod evmap;
mod async_dashmap;
mod flurry;
mod scc;
mod std;
mod whirlwind;
mod whirlwind_custom;

type Value = u32;
