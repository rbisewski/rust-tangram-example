#[macro_use]
extern crate clap;

use std::path::Path;

mod heartdisease;
use heartdisease::{convert_to_records, predict, HeartDiseaseWithDiagnosis, set_diagnosis};

use tangram::BinaryClassificationPredictOutput;

fn main() {
    let arguments = clap_app!(Rust_Tangram_Example =>
        (version: "1.0.0")
        (author: "Robert Bisewski <contact@ibiscybernetics.com>")
        (about: "Uses tangram trained models to make diagnosis predictions")
        (@arg INPUT: -i --input +required +takes_value "Sets the input file to use")
    ).get_matches();

    // parse the --input value
    let filename = match arguments.value_of("INPUT") {
        Some(v) => v,
        _ => {
            panic!("Empty or invalid file.");
        }
    };

    // safety check, ensure the file actually exists
    if !Path::new(filename).exists() {
        println!("No such file exists: {}", filename);
        return;
    }

    // load model
	let resulting_model = tangram::Model::from_path(
        "heart_disease_training_data.tangram", 
        None
    );

    let model = match resulting_model {
        Ok(r) => r,
        Err(e) => panic!("{}",e),
    };

    // set an example heart disease entry
    let entry = heartdisease::new(
        63.0,
        "male".to_string(),
        "typical angina".to_string(),
        145.0,
        233.0,
        "true".to_string(),
        "probable or definite left ventricular hypertrophy".to_string(),
        150.0,
        "no".to_string(),
        2.3,
        "downsloping".to_string(),
        0.0,
        "fixed defect".to_string(),
    );
    let output = predict(&model, &entry);
    println!("Single prediction\n{:?}", output);

    let records = convert_to_records(filename.to_string());
    let mut records_with_predictions: Vec<HeartDiseaseWithDiagnosis> = vec!();

    for r in records {
        let prediction_output = predict(&model, &r);
        let binary_output: BinaryClassificationPredictOutput = BinaryClassificationPredictOutput::from(prediction_output);

        let record_with_diagnosis = set_diagnosis(
            &r,
            binary_output.class_name,
            binary_output.probability,
        );

        records_with_predictions.push(record_with_diagnosis);
    }

    println!("Multiple predictions\n{:#?}", records_with_predictions);
}
