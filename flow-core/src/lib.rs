/*!
This crate contains the foundation of memflow's physical memory introspection.

You will almost always import this module when working with memflow.

It contains abstractions over [memory addresses](address/index.html),
[the underlying system architecture](arch/index.html),
[abstractions for reading memory](mem/index.html) and
[abstractions over processes and modules](process/index.html).
*/

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate smallvec;

extern crate either;

pub mod error;
pub use error::{Error, Result};

#[macro_use]
pub mod types;
pub use types::{Address, Length, Offset, Page, PageType, PhysicalAddress, Pointer32, Pointer64};

pub mod architecture;
pub use architecture::{Architecture, Endianess};

pub mod mem;
pub use mem::*;

pub mod process;
pub use process::*;

pub mod iter;
pub use iter::*;
