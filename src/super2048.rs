use super::Game;

pub struct Super2048 {
	desk: Vec<Vec<uint>>,
    score: uint
}

impl Game for Super2048 {
	fn new() -> Super2048 {
        let mut rng = task_rng();
        let mut vec = Vec::from_fn(4, |_| Vec::from_elem(4, 0u));
        
        
        Super2048 {desk: vec, score: 0 }
    }

	fn is_finished(&self) -> bool {
		false
	}

	fn window_size(&self) -> (uint, uint) {
		(4, 16)
	}

	fn score(&self) -> uint {
		self.score
	}
}
