use std::{error, io};

const INPUT: &str = include_str!("../input/09.txt");

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
struct DiskId(usize);

impl From<usize> for DiskId {
    fn from(value: usize) -> Self {
        DiskId(value)
    }
}

impl From<DiskId> for usize {
    fn from(value: DiskId) -> Self {
        value.0
    }
}

impl DiskId {
    fn id(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
enum DiskBlock {
    Unused,
    Used(DiskId),
}

#[derive(Clone, Copy)]
enum ImportState {
    File,
    FreeSpace,
}

fn get_input() -> Result<Vec<DiskBlock>, Box<dyn error::Error>> {
    let mut state = ImportState::File;
    let mut current_id = 0;
    let mut output = Vec::new();
    for character in INPUT.chars() {
        if character == '\n' {
            continue;
        }
        const BASE: u32 = 10;
        let c = character
            .to_digit(BASE)
            .ok_or_else(|| io::Error::other("failed to parse number"))?;
        match state {
            ImportState::File => {
                for _ in 0..c {
                    output.push(DiskBlock::Used(DiskId::from(current_id)));
                }
                current_id += 1;
                state = ImportState::FreeSpace;
            }
            ImportState::FreeSpace => {
                for _ in 0..c {
                    output.push(DiskBlock::Unused);
                }
                state = ImportState::File;
            }
        }
    }
    Ok(output)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut disk = get_input()?;

    let mut disk_forward_index = 0;

    // backwards index is saved as +1 so that i can check for 0 as a break condition
    let mut disk_backwards_index = disk.len();

    // move loop
    while disk_backwards_index > disk_forward_index {
        // SAFETY: Index is known to always be inbounds, due to checks when incrementing/decrementing.
        unsafe {
            if matches!(
                disk.get_unchecked(disk_backwards_index - 1),
                DiskBlock::Unused
            ) {
                disk_backwards_index -= 1;
                if disk_backwards_index == 0 {
                    break;
                }
                continue;
            }
            if matches!(disk.get_unchecked(disk_forward_index), DiskBlock::Used(_)) {
                disk_forward_index += 1;
                if disk_forward_index == disk.len() {
                    break;
                }
                continue;
            }
            *disk.get_unchecked_mut(disk_forward_index) =
                *disk.get_unchecked(disk_backwards_index - 1);
            *disk.get_unchecked_mut(disk_backwards_index - 1) = DiskBlock::Unused;
            disk_forward_index += 1;
            disk_backwards_index -= 1;
            if disk_forward_index == disk.len() || disk_backwards_index == 0 {
                break;
            }
        }
    }

    let result = disk
        .iter()
        .enumerate()
        .filter_map(|(offset, el)| match el {
            DiskBlock::Used(id) => Some(offset * id.id()),
            DiskBlock::Unused => None,
        })
        .fold(0, |acc, el| acc + el);

    println!("total result: {}", result);

    Ok(())
}
