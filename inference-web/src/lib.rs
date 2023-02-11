use tch::IValue;

/// At every time step, you can observe its position (x), velocity (x_dot), angle (theta), and angular velocity (theta_dot)
pub type State = [f32; 4];

#[derive(Debug, Clone)]
pub struct Inference {
    pub(crate) left: f64,
    pub(crate) right: f64,
    pub(crate) reward: f64,
}

pub fn extract_outputs(output: IValue) -> Option<Inference> {
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
        return Some(Inference {
            left,
            right,
            reward,
        });
    }

    None
}
