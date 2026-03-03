//! Logisim-Evolution `.circ` file format support.
//!
//! Logisim-Evolution stores projects in an XML-based `.circ` file format.
//! This crate provides a complete parser and writer for that format.
//!
//! # File format overview
//!
//! ```xml
//! <?xml version="1.0" encoding="UTF-8" standalone="no"?>
//! <project version="1.0">
//!   <lib desc="#Wiring" name="0"/>
//!   <lib desc="#Gates" name="1"/>
//!   ...
//!   <options>
//!     <a name="gateUndefined" val="isolated"/>
//!   </options>
//!   <mappings/>
//!   <toolbar>
//!     <tool lib="0" name="Poke Tool"/>
//!   </toolbar>
//!   <circuit name="main">
//!     <a name="circuit" val="main"/>
//!     <comp lib="1" loc="(160,130)" name="AND Gate">
//!       <a name="inputs" val="2"/>
//!     </comp>
//!     <wire from="(0,130)" to="(160,130)"/>
//!   </circuit>
//! </project>
//! ```

pub mod parser;
pub mod writer;
pub mod error;

pub use error::FileError;
pub use parser::parse_circ;
pub use writer::write_circ;
