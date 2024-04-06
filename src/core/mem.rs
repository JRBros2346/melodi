#![allow(non_upper_case_globals)]
use std::{alloc::Layout, sync::Mutex};
use strum::{EnumCount, IntoEnumIterator};

#[repr(usize)]
#[derive(
    PartialEq, Eq, Clone, Copy, strum::FromRepr, strum::EnumCount, strum::EnumIter, strum::AsRefStr,
)]
pub enum MemoryTag {
    Unknown,
    Array,
    Vector,
    Map,
    CircularQueue,
    BinarySearchTree,
    String,
    Application,
    Job,
    Texture,
    Material,
    Renderer,
    Game,
    Transform,
    Entity,
    EntityNode,
    Scene,
}

pub fn init() {}
pub fn close() {}

lazy_static::lazy_static! {
    static ref total_allocation: Mutex<u128> = Mutex::new(0);
    static ref tagged_allocation: Mutex<[u128; MemoryTag::COUNT]> =
        Mutex::new([0; MemoryTag::COUNT]);
}

pub unsafe fn alloc(size: usize, tag: MemoryTag) -> *mut u8 {
    if tag == MemoryTag::Unknown {
        crate::warn!("`alloc` called using `MemorTag::Unknown`. Re-class this allocation.");
    }
    *total_allocation.lock().unwrap() += size as u128;
    tagged_allocation.lock().unwrap()[tag as usize] += size as u128;
    std::alloc::alloc_zeroed(Layout::from_size_align(size, 1).unwrap())
}
pub unsafe fn dealloc(ptr: *mut u8, size: usize, tag: MemoryTag) {
    if tag == MemoryTag::Unknown {
        crate::warn!("`dealloc` called using `MemorTag::Unknown`. Re-class this allocation.");
    }
    *total_allocation.lock().unwrap() -= size as u128;
    tagged_allocation.lock().unwrap()[tag as usize] -= size as u128;
    std::alloc::dealloc(ptr, Layout::from_size_align(size, 1).unwrap())
}
pub unsafe fn realloc(
    ptr: &mut *mut u8,
    old_size: usize,
    new_size: usize,
    tag: MemoryTag,
) {
    if tag == MemoryTag::Unknown {
        crate::warn!("`dealloc` called using `MemorTag::Unknown`. Re-class this allocation.");
    }
    *total_allocation.lock().unwrap() -= old_size as u128;
    tagged_allocation.lock().unwrap()[tag as usize] -= new_size as u128;
    *total_allocation.lock().unwrap() += old_size as u128;
    tagged_allocation.lock().unwrap()[tag as usize] += new_size as u128;
    *ptr = std::alloc::realloc(*ptr, Layout::from_size_align(old_size, 1).unwrap(), new_size)
}
pub fn format_bytes(bytes: u128) -> String {
    #![allow(non_upper_case_globals)]
    const KiB: u128 = 1024;
    const MiB: u128 = KiB * 1024;
    const GiB: u128 = MiB * 1024;
    const TiB: u128 = GiB * 1024;
    const PiB: u128 = TiB * 1024;
    const EiB: u128 = PiB * 1024;
    const ZiB: u128 = EiB * 1024;
    const YiB: u128 = ZiB * 1024;
    match bytes {
        bytes if bytes >= YiB => {
            format!("{} YiB", bytes as f64 / YiB as f64)
        }
        bytes if bytes >= ZiB => {
            format!("{} ZiB", bytes as f64 / ZiB as f64)
        }
        bytes if bytes >= EiB => {
            format!("{} EiB", bytes as f64 / EiB as f64)
        }
        bytes if bytes >= PiB => {
            format!("{} PiB", bytes as f64 / PiB as f64)
        }
        bytes if bytes >= TiB => {
            format!("{} TiB", bytes as f64 / TiB as f64)
        }
        bytes if bytes >= GiB => {
            format!("{} GiB", bytes as f64 / GiB as f64)
        }
        bytes if bytes >= MiB => {
            format!("{} MiB", bytes as f64 / MiB as f64)
        }
        bytes if bytes >= KiB => {
            format!("{} KiB", bytes as f64 / KiB as f64)
        }
        bytes => {
            format!("{} B", bytes)
        }
    }
}
pub fn get_memory_usage() -> String {
    let align: usize = MemoryTag::iter()
        .map(|t| t.as_ref().chars().count())
        .fold("Total".chars().count(), std::cmp::max);
    let line_length = format!("\t{}: {}\n", " ".repeat(align), format_bytes(u128::MAX))
        .chars()
        .count();
    let mut out = String::from("System mempory use (tagged):\n");
    out.reserve(line_length * (MemoryTag::COUNT + 2));
    let lock = tagged_allocation.lock().unwrap();
    for tag in MemoryTag::iter() {
        out += &format!(
            "\t{:>align$}: {}\n",
            tag.as_ref(),
            format_bytes(lock[tag as usize])
        );
    }
    out += &(format!("\t{}\n", "-".repeat(line_length - 2)));
    out += &format!(
        "\t{:>align$}: {}\n",
        "Total",
        total_allocation.lock().unwrap()
    );
    out
}
