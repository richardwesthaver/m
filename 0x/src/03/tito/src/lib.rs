// TITO 0.0.1
#![feature(never_type, generic_associated_types, allocator_api, generator_trait)]
#![allow(dead_code, incomplete_features)]

pub mod alloc;
pub mod coll;
pub mod graph;
pub mod id;
pub mod imp;
pub mod mag;
pub mod opt;

fn always<T>(id: T) {
  Some(id);
}

#[cfg(test)]
mod tests;
