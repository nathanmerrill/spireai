use rand::{
    prelude::{IteratorRandom, SliceRandom, ThreadRng},
    Rng,
};

#[derive(Clone, Debug)]
pub struct Probability {
    pub probability: f64,
    rng: ThreadRng,
}

impl PartialEq for Probability {
    fn eq(&self, other: &Probability) -> bool {
        self.probability == other.probability
    }
}

impl Probability {
    pub fn choose<T>(&mut self, choices: Vec<T>) -> Option<T> {
        let resolved_count = choices.len();
        if resolved_count != 0 {
            self.probability /= resolved_count as f64;
        }

        choices.into_iter().choose(&mut self.rng)
    }

    pub fn choose_percentage(&mut self, percentage: f64) -> bool 
    {
        let result = self.rng.gen_bool(percentage);

        if result {
            self.probability *= percentage;
        } else {
            self.probability *= 1.0 - percentage;
        }

        result
    }

    pub fn range(&mut self, max: usize) -> usize {
        if max == 0 {
            return 0;
        }

        self.probability /= max as f64;
        self.rng.gen_range(0..max)
    }

    pub fn choose_weighted<'a, T>(&mut self, choices: &'a [(T, u8)]) -> Option<&'a T> {
        if choices.is_empty() {
            None
        } else {
            let choice_sum: u8 = choices.iter().map(|(_, a)| a).sum();

            let selection = choices.choose_weighted(&mut self.rng, |(_, a)| *a).unwrap();

            self.probability *= selection.1 as f64 / choice_sum as f64;

            Some(&selection.0)
        }
    }

    pub fn choose_multiple<T>(&mut self, choices: Vec<T>, count: usize) -> Vec<T> {
        let resolved_count = choices.len();

        let selection = choices.into_iter().choose_multiple(&mut self.rng, count);

        self.probability /= num_integer::binomial(resolved_count, selection.len()) as f64;

        selection
    }

    pub fn combine<T>(&mut self, rhs: (T, Self)) -> T {
        self.probability *= rhs.1.probability;
        rhs.0
    }

    pub fn new() -> Probability {
        Probability {
            rng: ThreadRng::default(),
            probability: 1.0,
        }
    }
}
