use inference_web::{infer, State, CARTPOLE_MODEL_FILE_PATH};

pub fn main() -> anyhow::Result<()> {
    let model = tch::CModule::load(CARTPOLE_MODEL_FILE_PATH)?;
    let state: State = [0f32, 0f32, -1f32, 0f32];
    let inference = infer(&model, &state).unwrap();
    dbg!(inference);
    Ok(())
}
