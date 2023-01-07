pub mod addressbook {
    include!(concat!(env!("OUT_DIR"), "/addressbook.rs"));
}

pub use addressbook::*;
