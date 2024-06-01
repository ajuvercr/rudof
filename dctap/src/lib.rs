//! DCTAP Processor
//!
//! This module contains a simple [DCTAP](https://www.dublincore.org/specifications/dctap/) processor
//! 
//! 
//! DCTAP (Dublin Core Tabular Application Profiles) is a simple model that can be used to specify data models
//! 
pub mod dctap;
pub mod dctap_error;

pub use dctap::*;
pub use dctap_error::*;
