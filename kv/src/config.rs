use std::time::Duration;

pub struct Config {
    pub store_addr: String,
    pub raft: bool,
    pub scheduler_addr: String,
    pub log_level: String,

    pub dbpath: String,

    pub raft_base_tick_interval: Duration,
    pub raft_heartbeat_ticks: i32,
    pub raft_election_timeout_ticks: i32,

    pub raft_log_gc_tick_interval: Duration,
    pub raft_log_gc_count_limit: u64,

    pub split_region_check_tick_interval: Duration,
    pub scheduler_heartbeat_tick_interval: Duration,
    pub scheduler_store_heartbeat_tick_interval: Duration,

    pub region_max_size: u64,
    pub region_split_size: u64,
}

impl Config {
    pub fn default() -> Self {
        Config {
            scheduler_addr: String::from("127.0.0.1:2379"),
            store_addr: String::from("127.0.0.1:20160"),
            log_level: String::from("info"),
            raft: true,
            dbpath: String::from("/tmp/badger"),
            raft_base_tick_interval: Duration::from_secs(1),
            raft_heartbeat_ticks: 2,
            raft_election_timeout_ticks: 10,
            raft_log_gc_tick_interval: Duration::from_secs(10),
            // Assume the average size of entries is 1k.
            raft_log_gc_count_limit: 128000,
            split_region_check_tick_interval: Duration::from_secs(10),
            scheduler_heartbeat_tick_interval: Duration::from_secs(10),
            scheduler_store_heartbeat_tick_interval: Duration::from_secs(10),
            region_max_size: 144 * 1024 * 1024,
            region_split_size: 96 * 1024 * 1024,
        }
    }
}