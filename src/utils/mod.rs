pub mod math;

pub use math::Val;
pub use math::cal_vector_by_two_points;
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};


#[derive(Debug, Deserialize, Serialize)]
pub struct Pool<T>(pub Vec<T>);

impl<T> Pool<T> {
    fn get_pool_item_numbers(&self) -> usize {
        self.0.len()
    }

    pub fn get_random_item_from_pool(&self) -> &T {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..self.get_pool_item_numbers());
        
        self.0.get(index).unwrap()
    }
}


