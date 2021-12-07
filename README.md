# Rust tangram example

This repo exists to detail how to use the tangram CLI tool alongside Rust to first train and then predict using the generated model.

### Train a model

First, install tangram:

```bash
yay -S tangram-bin
```

Second, take CSV datafile and train on this data using tangram:

```bash
tangram train --file heart_disease_training_data.csv --target diagnosis
```

### Predict

With the model created, you can feed in some user data, in the form of a CSV, like so:

```bash
make && ./target/release/rust-tangram-example --input heart_disease_user_data.csv
```

Afterwards, the program will print a list of predictions of the expected predictions, in the form of:

```
HeartDiseaseWithDiagnosis {
    age: 99.0,
    gender: "female",
    chest_pain: "asymptomatic",
    resting_blood_pressure: 138.0,
    cholesterol: 234.0,
    fasting_blood_sugar_greater_than_120: "false",
    resting_ecg_result: "probable or definite left ventricular hypertrophy",
    exercise_max_heart_rate: 160.0,
    exercise_induced_angina: "no",
    exercise_st_depression: 0.0,
    exercise_st_slope: "upsloping",
    fluoroscopy_vessels_colored: 0.0,
    thallium_stress_test: "normal",
    diagnosis: "Negative",
    diagnosis_probability: 0.79605556,
},
```

For the above example, the predicted diagnosis is "Negative" with a probability of 79.6%