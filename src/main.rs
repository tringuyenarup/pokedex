use repositories::pokemon::InMemoryRepository;
use std::sync::Arc;

mod api;
mod domain;
mod repositories;

#[macro_use]
extern crate rouille;
extern crate serde;

fn main() {
    let repo = Arc::new(InMemoryRepository::new());
    api::serve("localhost:8000", repo);
}
