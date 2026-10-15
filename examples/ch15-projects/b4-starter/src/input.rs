#[derive(Debug)]
pub struct Input {
    pub input_index: usize,
    pub original: String,
}

#[derive(Debug)]
pub enum InputError {
    Empty,
}

// --8<-- [start:prepare]
pub fn prepare_inputs(text: &str) -> Result<Vec<Input>, InputError> {
    let mut inputs = Vec::new();
    for (input_index, line) in text.lines().enumerate() {
        if !line.trim().is_empty() {
            inputs.push(Input {
                input_index,
                original: line.to_owned(),
            });
        }
    }
    if inputs.is_empty() {
        Err(InputError::Empty)
    } else {
        Ok(inputs)
    }
}
// --8<-- [end:prepare]
