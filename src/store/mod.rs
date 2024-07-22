use lazy_static::lazy_static;
use async_once::AsyncOnce;
use tantivy::{
    schema::Schema,
    index::Index,
};

pub mod utils;

pub mod people;

lazy_static! {
    pub static ref PERSON_SCHEMA: Schema = people::build_schema();
    pub static ref PEOPLE_INDEX: AsyncOnce<Index> = AsyncOnce::new(async {
        match people::open_index(PERSON_SCHEMA.clone()).await {
            Ok(index) => index,
            Err(err) => {
                panic!("Failed to open people index: {:?}", err);
            },
        }
    });
}
