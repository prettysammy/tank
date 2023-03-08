pub mod math;

pub use math::Val;
pub use math::cal_vector_by_two_points;
use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Pool<T>(pub Vec<T>);


