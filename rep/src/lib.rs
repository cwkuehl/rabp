pub mod models;
//pub mod models_ext;
//mod res;
mod reps;
pub mod revision;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate serde;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
