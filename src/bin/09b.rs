use std::{collections::BTreeSet, error, io};

const INPUT: &str = include_str!("../input/09.txt");

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum DiskBlock {
    Unused,
    Used(DiskId),
}

#[derive(Clone, Copy, Debug)]
enum ImportState {
    File,
    FreeSpace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FreeList {
    offset: usize,
    len: usize,
}

impl PartialOrd for FreeList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FreeList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.offset
            .cmp(&other.offset)
            .then_with(|| self.len.cmp(&other.len))
    }
}

#[derive(Clone, Copy, Debug)]
struct UsedList {
    id: DiskId,
    len: usize,
    offset: usize,
}

fn get_input() -> Result<(usize, BTreeSet<FreeList>, Vec<UsedList>), Box<dyn error::Error>> {
    let mut state = ImportState::File;
    let mut current_id = 0;
    let mut freelist_output = BTreeSet::new();
    let mut todo_output = Vec::new();
    let mut offset = 0;
    for character in INPUT.chars() {
        if character == '\n' {
            continue;
        }

        const BASE: u32 = 10;
        let len = character
            .to_digit(BASE)
            .ok_or_else(|| io::Error::other("failed to parse number"))? as _;

        match state {
            ImportState::File => {
                todo_output.push(UsedList {
                    id: DiskId::from(current_id),
                    len,
                    offset,
                });
                current_id += 1;
                state = ImportState::FreeSpace;
            }
            ImportState::FreeSpace => {
                freelist_output.insert(FreeList { offset, len });
                state = ImportState::File;
            }
        }

        offset += len;
    }
    Ok((offset, freelist_output, todo_output))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let (total_elements, mut freelist, todo_list) = get_input()?;

    // dbg!(&freelist);
    // dbg!(&todo_list);
    let mut done_list = Vec::new();
    for el in todo_list.into_iter().rev() {
        let free_space = (|| {
            for l in &freelist {
                if l.offset >= el.offset {
                    break;
                }
                if l.len < el.len {
                    continue;
                }
                return Some(*l);
            }
            None
        })();

        match free_space {
            Some(l) => {
                let new_el = UsedList {
                    id: el.id,
                    len: el.len,
                    offset: l.offset,
                };
                done_list.push(new_el);
                let _res = freelist.remove(&l);

                assert!(_res);

                let leftover_len = l.len - el.len;
                if leftover_len > 0 {
                    let new_offset = l.offset + el.len;
                    let new_entry = FreeList {
                        offset: new_offset,
                        len: leftover_len,
                    };
                    freelist.insert(new_entry);
                }
            }
            None => done_list.push(el),
        };
    }

    let mut disk = vec![DiskBlock::Unused; total_elements];

    for el in done_list {
        for i in 0..el.len {
            let off = el.offset + i;
            disk[off] = DiskBlock::Used(DiskId::from(el.id));
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
