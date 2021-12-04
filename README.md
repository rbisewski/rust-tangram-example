# Rust tangram example

This repo exists to detail how to use the tangram CLI tool alongside Rust to first train and then predict using the generated model.

### Train a model

First, install tangram:

```bash
yay -S tangram
```

Second, take CSV datafile and train on this data using tangram:

```bash
tangram train --file heart_disease.csv --target diagnosis
```

### Predict

With the model created, you can

```bash
make run
```