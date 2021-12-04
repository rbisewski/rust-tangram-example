use std::env;

mod heartdisease;
use heartdisease::{convert_to_records, predict};
use tangram::{BinaryClassificationPredictOutput};

use crate::heartdisease::{HeartDiseaseWithDiagnosis, set_diagnosis};

fn main() {

    let args: Vec<String> = env::args().collect();
    let flag: &String;
    let filename: &String;

    match args.len() {
        3 => {
            flag = &args[1];
            filename = &args[2];
        },
        _ => {
            println!("Usage: ./rust-tangram-example --input /path/to/new_data.csv");
            return;
        }
    }

    if flag.is_empty() || filename.is_empty() || flag != "--input" {
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
