// mod builders;
// use builders::make_arrays;
mod dynamic_types;
use dynamic_types::infer_types;

fn main() {
    // infer_schema();
    // make_arrays();
    match infer_types() {
        Ok(record_batch) => println!("{:?}", record_batch),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
