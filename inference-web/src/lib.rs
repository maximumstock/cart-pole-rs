use serde::Serialize;
use tch::IValue;

pub fn get_model_file_path() -> String {
    format!(
        "{}/../models/CartPole-v1/model_traced.pt",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    )
}

/// At every time step, you can observe its position (x), velocity (x_dot), angle (theta), and angular velocity (theta_dot)
pub type InferenceInput = [f32; 4];

#[derive(Debug, Clone, Serialize)]
pub struct Inference {
    pub left: f64,
    pub right: f64,
    pub reward: f64,
}

pub fn infer(model: &tch::CModule, state: &InferenceInput) -> Result<Inference, tch::TchError> {
    model
        .forward_is(&[IValue::Tensor(tch::Tensor::of_slice(state))])
        .map(extract_outputs)
}

pub fn extract_outputs(output: IValue) -> Inference {
    if let IValue::Tuple(list) = output {
        let actions = list.get(0).unwrap();
        let reward = list.get(1).unwrap();
        let (left, right) = match actions {
            IValue::Tensor(t) => (t.double_value(&[0]), t.double_value(&[1])),
            _ => (0f64, 0f64),
        };
        let reward = match reward {
            IValue::Tensor(t) => t.double_value(&[0]),
            _ => 0f64,
        };
        return Inference {
            left,
            right,
            reward,
        };
    }

    unreachable!()
}
