use std::fs::read_to_string;
use serde::Deserialize;

use tangram::PredictOutput;

#[derive(Debug,Deserialize)]
pub struct HeartDisease {
    age: f32,
    gender: String,
    chest_pain: String,
    resting_blood_pressure: f32,
    cholesterol: f32,
    fasting_blood_sugar_greater_than_120: String,
    resting_ecg_result: String,
    exercise_max_heart_rate: f32,
    exercise_induced_angina: String,
    exercise_st_depression: f32,
    exercise_st_slope: String,
    fluoroscopy_vessels_colored: f32,
    thallium_stress_test: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct HeartDiseaseWithDiagnosis {
    age: f32,
    gender: String,
    chest_pain: String,
    resting_blood_pressure: f32,
    cholesterol: f32,
    fasting_blood_sugar_greater_than_120: String,
    resting_ecg_result: String,
    exercise_max_heart_rate: f32,
    exercise_induced_angina: String,
    exercise_st_depression: f32,
    exercise_st_slope: String,
    fluoroscopy_vessels_colored: f32,
    thallium_stress_test: String,
    diagnosis: String,
    diagnosis_probability: f32,
}

#[allow(clippy::too_many_arguments)]
pub fn new(
    age: f32,
    gender: String,
    chest_pain: String,
    resting_blood_pressure: f32,
    cholesterol: f32,
    fasting_blood_sugar_greater_than_120: String,
    resting_ecg_result: String,
    exercise_max_heart_rate: f32,
    exercise_induced_angina: String,
    exercise_st_depression: f32,
    exercise_st_slope: String,
    fluoroscopy_vessels_colored: f32,
    thallium_stress_test: String,
) -> HeartDisease {

    HeartDisease {
        age,
        gender,
        chest_pain,
        resting_blood_pressure,
        cholesterol,
        fasting_blood_sugar_greater_than_120,
        resting_ecg_result,
        exercise_max_heart_rate,
        exercise_induced_angina,
        exercise_st_depression,
        exercise_st_slope,
        fluoroscopy_vessels_colored,
        thallium_stress_test,
    }
}

pub fn convert_to_records(filename: String) -> Vec<HeartDisease> {
    let mut list_of_records: Vec<HeartDisease> = vec!();

    let contents = match read_to_string(filename) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    
    let lines: Vec<&str> = contents.split('\n').collect();
    let mut clean_lines = String::from("");

    for l in lines {

        // afterwards, attempt to clean each line
        let current_line_elements: Vec<&str> = l.split(',').collect();
        let mut clean_line = String::from("");

        for element in current_line_elements {
            let fixed_element = match element {
                "" => "\"\"".to_string(),
                _ => element.trim().to_string(),
            };
            clean_line = [
                clean_line,
                fixed_element,
                ",".to_string(),
            ].concat();
        }

        clean_lines = [
            clean_lines,
            "\n".to_string(),
            clean_line.trim_end_matches(',').to_string(),
        ].concat();
    }

    let mut rdr = csv::Reader::from_reader(clean_lines.as_bytes());

    let mut num_of_errors = 0;
    for res in rdr.deserialize::<HeartDisease>() {
        let record: HeartDisease = match res {
            Ok(r) => r,
            Err(_) => {
                num_of_errors+=1;
                continue
            },
        };
        list_of_records.push(record);
    }

    if num_of_errors > 0 {
        println!("Number of broken lines that were skipped: {}", num_of_errors);
    }

    list_of_records
}

pub fn predict(model: &tangram::Model, entry: &HeartDisease) -> PredictOutput {
    let input = tangram::predict_input! {
        "age": entry.age,
        "gender": entry.gender.as_str().to_string(),
        "chest_pain": entry.chest_pain.as_str().to_string(),
        "resting_blood_pressure": entry.resting_blood_pressure,
        "cholesterol": entry.cholesterol,
        "fasting_blood_sugar_greater_than_120": entry.fasting_blood_sugar_greater_than_120.as_str().to_string(),
        "resting_ecg_result": entry.resting_ecg_result.as_str().to_string(),
        "exercise_max_heart_rate": entry.exercise_max_heart_rate,
        "exercise_induced_angina": entry.exercise_induced_angina.as_str().to_string(),
        "exercise_st_depression": entry.exercise_st_depression,
        "exercise_st_slope": entry.exercise_st_slope.as_str().to_string(),
        "fluoroscopy_vessels_colored": entry.fluoroscopy_vessels_colored,
        "thallium_stress_test": entry.thallium_stress_test.as_str().to_string(),
    };

    // make a single prediction
    model.predict_one(input, None)
}

pub fn set_diagnosis(entry: &HeartDisease, diagnosis: String, diagnosis_probability: f32) -> HeartDiseaseWithDiagnosis {

    HeartDiseaseWithDiagnosis {
        age: entry.age,
        gender: entry.gender.as_str().to_string(),
        chest_pain: entry.chest_pain.as_str().to_string(),
        resting_blood_pressure: entry.resting_blood_pressure,
        cholesterol: entry.cholesterol,
        fasting_blood_sugar_greater_than_120: entry.fasting_blood_sugar_greater_than_120.as_str().to_string(),
        resting_ecg_result: entry.resting_ecg_result.as_str().to_string(),
        exercise_max_heart_rate: entry.exercise_max_heart_rate,
        exercise_induced_angina: entry.exercise_induced_angina.as_str().to_string(),
        exercise_st_depression: entry.exercise_st_depression,
        exercise_st_slope: entry.exercise_st_slope.as_str().to_string(),
        fluoroscopy_vessels_colored: entry.fluoroscopy_vessels_colored,
        thallium_stress_test: entry.thallium_stress_test.as_str().to_string(),
        diagnosis,
        diagnosis_probability,
    }
}