use crate::macros::const_while;

const TABLE_SIZE: usize = 1 << 23;

const fn get_index(hash: u64) -> usize {
    hash as usize & (TABLE_SIZE - 1)
}

pub(super) fn create() -> Table {
    let table = vec![0u8; TABLE_SIZE];
    table.into_boxed_slice()
}

pub(super) const fn read(table: &Table, hash: u64) -> u8 {
    table[get_index(hash)]
}

pub(super) const fn increment(table: &mut Table, hash: u64) {
    table[get_index(hash)] += 1;
}

pub(super) const fn decrement(table: &mut Table, hash: u64) {
    table[get_index(hash)] -= 1;
}

pub(super) const fn reset(table: &mut Table) {
    const_while!(i, 0, TABLE_SIZE, {
        table[i] = 0;
    });
}

pub(super) type Table = Box<[u8]>;
