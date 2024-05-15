use once_cell::sync::Lazy;
use portable_atomic::{AtomicU128, Ordering};
use std::mem::MaybeUninit;
use strum::{EnumCount, IntoEnumIterator};

#[repr(usize)]
#[derive(
    PartialEq, Eq, Clone, Copy, strum::FromRepr, strum::EnumCount, strum::EnumIter, strum::AsRefStr,
)]
pub enum MemoryTag {
    Unknown = 0,
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

static TOTAL_ALLOCATION: AtomicU128 = AtomicU128::new(0);
static TAGGED_ALLOCATION: Lazy<[AtomicU128; MemoryTag::COUNT]> =
    Lazy::new(|| std::array::from_fn(|_| AtomicU128::new(0)));
// {
//     const ARRAY_REPEAT_VALUE: Mutex<u128> = Mutex::new(0);
//     let mut arr = [ARRAY_REPEAT_VALUE; MemoryTag::COUNT];
//     for i in 0..MemoryTag::COUNT {
//         arr[i] = Mutex::new(0);
//     }
//     arr
// };

pub fn total_allocation() -> u128 {
    TOTAL_ALLOCATION.load(Ordering::Relaxed)
}
pub fn tagged_allocation(tag: MemoryTag) -> u128 {
    TAGGED_ALLOCATION[tag as usize].load(Ordering::Relaxed)
}

pub unsafe fn alloc<T>(size: usize, tag: MemoryTag) -> Box<[MaybeUninit<T>]> {
    if tag == MemoryTag::Unknown {
        crate::warn!("`alloc` called using `MemoryTag::Unknown`. Re-class this allocation.");
    }
    TOTAL_ALLOCATION.fetch_add(
        size as u128 * std::mem::size_of::<T>() as u128,
        Ordering::Relaxed,
    );
    TAGGED_ALLOCATION[tag as usize].fetch_add(
        size as u128 * std::mem::size_of::<T>() as u128,
        Ordering::Relaxed,
    );
    (0..size)
        .map(|_| MaybeUninit::zeroed())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}
pub unsafe fn dealloc<T>(boxed: Box<[MaybeUninit<T>]>, tag: MemoryTag) {
    if tag == MemoryTag::Unknown {
        crate::warn!("`dealloc` called using `MemoryTag::Unknown`. Re-class this allocation.");
    }
    TOTAL_ALLOCATION.fetch_sub(
        boxed.len() as u128 * std::mem::size_of::<T>() as u128,
        Ordering::Relaxed,
    );
    TAGGED_ALLOCATION[tag as usize].fetch_sub(
        boxed.len() as u128 * std::mem::size_of::<T>() as u128,
        Ordering::Relaxed,
    );
    drop(boxed)
}
pub unsafe fn realloc<T>(boxed: &mut Box<[MaybeUninit<T>]>, new_size: usize, tag: MemoryTag) {
    if tag == MemoryTag::Unknown {
        crate::warn!("`realloc` called using `MemoryTag::Unknown`. Re-class this allocation.");
    }
    TOTAL_ALLOCATION.fetch_sub(
        boxed.len() as u128 * std::mem::size_of::<T>() as u128,
        Ordering::Relaxed,
    );
    TAGGED_ALLOCATION[tag as usize].fetch_sub(
        boxed.len() as u128 * std::mem::size_of::<T>() as u128,
        Ordering::Relaxed,
    );
    TOTAL_ALLOCATION.fetch_add(
        new_size as u128 * std::mem::size_of::<T>() as u128,
        Ordering::Relaxed,
    );
    TAGGED_ALLOCATION[tag as usize].fetch_add(
        new_size as u128 * std::mem::size_of::<T>() as u128,
        Ordering::Relaxed,
    );
    let mut new_boxed = (0..new_size)
        .map(|_| MaybeUninit::zeroed())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    std::ptr::copy_nonoverlapping(
        boxed.as_ptr(),
        new_boxed.as_mut_ptr(),
        boxed.len().min(new_size),
    );
    *boxed = new_boxed;
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
    for tag in MemoryTag::iter() {
        out += &format!(
            "\t{:>align$}: {}\n",
            tag.as_ref(),
            format_bytes(tagged_allocation(tag))
        );
    }
    out += &(format!("\t{}\n", "-".repeat(line_length - 2)));
    out += &format!(
        "\t{:>align$}: {}\n",
        "Total",
        format_bytes(total_allocation())
    );
    out
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore = "Race Condition"]
    fn test_alloc_dealloc() {
        let size = 5;
        let tag = MemoryTag::Unknown;
        let allocation = unsafe { alloc::<u32>(size, tag) };
        crate::info!("{}", get_memory_usage());
        assert_eq!(
            total_allocation(),
            size as u128 * std::mem::size_of::<u32>() as u128
        );
        assert_eq!(
            tagged_allocation(tag),
            size as u128 * std::mem::size_of::<u32>() as u128
        );

        unsafe { dealloc(allocation, tag) };
        crate::info!("{}", get_memory_usage());
        assert_eq!(total_allocation(), 0);
        assert_eq!(tagged_allocation(tag), 0);
    }

    #[test]
    #[ignore = "Race Condition"]
    fn test_realloc() {
        let mut allocation = unsafe { alloc::<u32>(5, MemoryTag::Unknown) };

        let new_size = 10;
        let tag = MemoryTag::Unknown;

        unsafe { realloc(&mut allocation, new_size, tag) };

        assert_eq!(allocation.len(), new_size);
        assert_eq!(
            total_allocation(),
            new_size as u128 * std::mem::size_of::<u32>() as u128
        );
        assert_eq!(
            tagged_allocation(tag),
            new_size as u128 * std::mem::size_of::<u32>() as u128
        );
    }
}
