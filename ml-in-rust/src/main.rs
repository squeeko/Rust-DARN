extern crate csv;
extern crate ndarray;
extern crate ndarray_csv;

use csv::ReaderBuilder;
use ndarray::{s, Array2};
use ndarray_csv::Array2Reader;
use std::error::Error;
use std::fs::File;


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("/Users/jallgood/D-A-R-N/data/MalwareArtifacts.csv")?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let malware_data: Array2<u32> = reader.deserialize_array2_dynamic()?;

   
    println!("Shape {:?}", malware_data.shape());
    println!("Slices {:?}", malware_data.slice(s![0..137443, ..8])); // https://docs.rs/ndarray/latest/ndarray/macro.s.html
    //println!("second time: {:?}", malware_data.row(1));
    
    Ok(())

    
}
