use inference_web::{extract_outputs, State};
use tch::IValue;

pub fn main() -> anyhow::Result<()> {
    let model_file_path = "../model_traced.pt";
    let model = tch::CModule::load(model_file_path)?;
    let state: State = [0f32, 0f32, -1f32, 0f32];
    let inference = model
        .forward_is(&[IValue::Tensor(tch::Tensor::of_slice(&state))])
        .map(extract_outputs)
        .unwrap();
    dbg!(inference);
    Ok(())
}
