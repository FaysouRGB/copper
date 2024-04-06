pub mod database;
pub mod entry;
pub mod errors;
pub mod interface;
pub mod lsm_tree;
pub mod memtable;
pub mod settings;
pub mod sql_parser;
pub mod table;
pub mod wal;

fn main() {
    // Start the interface loop.
    interface::interface_loop();
}
