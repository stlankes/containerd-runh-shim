/*
   Copyright The containerd Authors.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

// Forked from https://github.com/pwFoo/rust-runc/blob/313e6ae5a79b54455b0a242a795c69adf035141a/src/events.rs

/*
 * Copyright 2020 fsyncd, Berlin, Germany.
 * Additional material, copyright of the containerd authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Event type generated by runh
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum EventType {
    /// Statistics
    Stats,
    /// Out of memory
    Oom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub id: String,
    #[serde(rename = "data")]
    pub stats: Option<Stats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub cpu: Cpu,
    pub memory: Memory,
    pub pids: Pids,
    #[serde(rename = "blkio")]
    pub block_io: BlkIO,
    #[serde(rename = "hugetlb")]
    pub huge_tlb: HugeTLB,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HugeTLB {
    pub usage: Option<u64>,
    pub max: Option<u64>,
    #[serde(rename = "failcnt")]
    pub fail_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlkIOEntry {
    pub major: Option<u64>,
    pub minor: Option<u64>,
    pub op: Option<String>,
    pub value: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlkIO {
    /// Number of bytes transferred to and from the disk
    #[serde(rename = "ioServiceBytesRecursive")]
    pub io_service_bytes_recursive: Option<Vec<BlkIOEntry>>,
    /// Number of io requests issued to the disk
    #[serde(rename = "ioServicedRecursive")]
    pub io_serviced_recursive: Option<Vec<BlkIOEntry>>,
    /// Number of queued disk io requests
    #[serde(rename = "ioQueueRecursive")]
    pub io_queued_recursive: Option<Vec<BlkIOEntry>>,
    /// Amount of time io requests took to service
    #[serde(rename = "ioServiceTimeRecursive")]
    pub io_service_time_recursive: Option<Vec<BlkIOEntry>>,
    /// Amount of time io requests spent waiting in the queue
    #[serde(rename = "ioWaitTimeRecursive")]
    pub io_wait_time_recursive: Option<Vec<BlkIOEntry>>,
    /// Number of merged io requests
    #[serde(rename = "ioMergedRecursive")]
    pub io_merged_recursive: Option<Vec<BlkIOEntry>>,
    /// Disk time allocated the device
    #[serde(rename = "ioTimeRecursive")]
    pub io_time_recursive: Option<Vec<BlkIOEntry>>,
    /// Number of sectors transferred to and from the io device
    #[serde(rename = "sectorsRecursive")]
    pub sectors_recursive: Option<Vec<BlkIOEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pids {
    /// Number of pids in the cgroup
    pub current: Option<u64>,
    /// Active pids hard limit
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Throttling {
    /// Number of periods with throttling active
    pub periods: Option<u64>,
    #[serde(rename = "throttledPeriods")]
    /// Number of periods when the container hit its throttling limit
    pub throtted_periods: Option<u64>,
    /// Aggregate time the container was throttled for in nanoseconds
    #[serde(rename = "throttledTime")]
    pub throtted_time: Option<u64>,
}

/// Each members represents time in nanoseconds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuUsage {
    /// Total CPU time consumed
    pub total: Option<u64>,
    /// Total CPU time consumed per core
    pub per_cpu: Option<Vec<u64>>,
    /// Total CPU time consumed in kernel mode
    pub kernel: u64,
    /// Total CPU time consumed in user mode
    pub user: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cpu {
    pub usage: Option<u64>,
    pub throttling: Option<Throttling>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Memory limit in bytes
    pub limit: u64,
    /// Usage in bytes
    pub usage: Option<u64>,
    /// Maximum usage in bytes
    pub max: Option<u64>,
    /// Count of memory allocation failures
    #[serde(rename = "failcnt")]
    pub fail_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Memory usage for cache
    pub cache: Option<u64>,
    /// Overall memory usage, excluding swap
    pub usage: Option<MemoryEntry>,
    /// Overall memory usage, including swap
    pub swap: Option<MemoryEntry>,
    /// Kernel usage of memory
    pub kernel: Option<MemoryEntry>,
    /// Kernel TCP of memory
    #[serde(rename = "kernelTCP")]
    pub kernel_tcp: Option<MemoryEntry>,
    /// Raw stats of memory
    pub raw: Option<HashMap<String, u64>>,
}
