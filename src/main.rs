use tangram::PredictOutput;

struct HeartDisease {
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

fn predict(model: tangram::Model, entry: HeartDisease) -> PredictOutput {
    let input = tangram::predict_input! {
        "age": entry.age,
        "gender": entry.gender,
        "chest_pain": entry.chest_pain,
        "resting_blood_pressure": entry.resting_blood_pressure,
        "cholesterol": entry.cholesterol,
        "fasting_blood_sugar_greater_than_120": entry.fasting_blood_sugar_greater_than_120,
        "resting_ecg_result": entry.resting_ecg_result,
        "exercise_max_heart_rate": entry.exercise_max_heart_rate,
        "exercise_induced_angina": entry.exercise_induced_angina,
        "exercise_st_depression": entry.exercise_st_depression,
        "exercise_st_slope": entry.exercise_st_slope,
        "fluoroscopy_vessels_colored": entry.fluoroscopy_vessels_colored,
        "thallium_stress_test": entry.thallium_stress_test,
    };

    // make a single prediction
    model.predict_one(input, None)
}

fn main() {
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
    let entry = HeartDisease{
        age: 63.0,
        gender: "male".to_string(),
        chest_pain: "typical angina".to_string(),
        resting_blood_pressure: 145.0,
        cholesterol: 233.0,
        fasting_blood_sugar_greater_than_120: "true".to_string(),
        resting_ecg_result: "probable or definite left ventricular hypertrophy".to_string(),
        exercise_max_heart_rate: 150.0,
        exercise_induced_angina: "no".to_string(),
        exercise_st_depression: 2.3,
        exercise_st_slope: "downsloping".to_string(),
        fluoroscopy_vessels_colored: 0.0,
        thallium_stress_test: "fixed defect".to_string(),
    };

    let output = predict(model, entry);

    println!("Output: {:?}", output);
}
