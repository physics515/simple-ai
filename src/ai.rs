use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use bigdecimal::ToPrimitive;
use bigdecimal::Zero;
use rand::Rng;
use std::str::FromStr;

const COUNT: usize = 13;

#[derive(Debug)]
pub struct AI {
	neurons: [BigDecimal; COUNT],
	connections_weights: [[BigDecimal; COUNT]; COUNT],
        signals: [[BigDecimal; COUNT]; COUNT],
	nuerons_fired: [bool; COUNT],
}

impl AI {

        /// initialize a new AI with random values
	pub fn init() -> AI {
		let mut ai = AI {
			neurons: (0..COUNT).map(|_| BigDecimal::zero()).collect::<Vec<_>>().try_into().unwrap(),
			connections_weights: (0..COUNT).map(|_| (0..COUNT).map(|_| BigDecimal::zero()).collect::<Vec<_>>().try_into().unwrap()).collect::<Vec<_>>().try_into().unwrap(),
                        signals: (0..COUNT).map(|_| (0..COUNT).map(|_| BigDecimal::zero()).collect::<Vec<_>>().try_into().unwrap()).collect::<Vec<_>>().try_into().unwrap(),
			nuerons_fired: [false; COUNT],
		};
		for i in 0..COUNT {
			let mut rng = rand::thread_rng();
			let rand_f64: f64 = rng.gen_range(-(COUNT as i64)..COUNT as i64) as f64;
			//println!("rand number: {}", rand_f64);
			ai.neurons[i] = BigDecimal::from_str(&rand_f64.to_string()).unwrap();
			for j in 0..COUNT {
				let mut rng = rand::thread_rng();
				let rand_f64: f64 = rng.gen();
				ai.connections_weights[i][j] = BigDecimal::from_f64(rand_f64).unwrap();
			}
		}
		ai
	}

        /// fire the input neuron
        /// 1. when a neuron fires, it sends its value to all other neurons via connections
        ///     a. the connection weights is a multiplier for the neuorn signal
        ///     b. the neuron signal is the value of the neuron that fired multiplied by the connection weight
        /// 2. when a neuron receives a signal, if the signal is greater than the value of the neuron, the neuron fires
        ///     a. when a neuron fires, it resets all of the signals it has received to zero
        ///     b. when a neuron fires, it sends its value to all other neurons via connections
        /// 3. the function returns when no more neurons fire
        /// 4. the function returns the sum of the last set of neurons that fired
	pub fn fire(&mut self, value: BigDecimal) -> BigDecimal {
		let input_neuron_index = 0;
                self.neurons[input_neuron_index] = value;
                self.nuerons_fired = [false; COUNT];
                self.nuerons_fired[input_neuron_index] = true;
                self.signals.iter_mut().for_each(|row| row.iter_mut().for_each(|cell| *cell = BigDecimal::zero()));
                let mut nth_neurons_fired: [bool; COUNT] = [false; COUNT];
                let mut last_neurons_fired: [bool; COUNT];
                let mut fired = [false; COUNT];

                // loop until no more neurons fire
                let mut run_count = 0;
                //println!("run: {}, neurons fired: {:?}", run_count, self.nuerons_fired);
                while self.nuerons_fired.iter().any(|fired| *fired) {
                        nth_neurons_fired = [false; COUNT];

                        // update signals for all neurons
                        for i in 0..COUNT {
                                if self.nuerons_fired[i] {
                                        for j in 0..COUNT {
                                                self.signals[i][j] = self.neurons[i].clone() * self.connections_weights[i][j].clone();
                                        }
                                }

                                if fired[i] {
                                        for j in 0..COUNT {
                                                if self.signals[i][j] > self.neurons[j] {
                                                        self.signals[i].iter_mut().for_each(|cell| *cell = BigDecimal::zero());
                                                }
                                        }
                                }
                        }

                        // update neurons_fired for all neurons
                        for i in 0..COUNT {
                                if self.nuerons_fired[i] && !fired[i] {
                                        for j in 0..COUNT {
                                                if self.signals[i][j] > self.neurons[j] {
                                                        nth_neurons_fired[j] = true;
                                                        self.signals.iter_mut().for_each(|row| row[j] = BigDecimal::zero());
                                                }
                                        }
                                }
                        }

                        // reset neurons_fired for all neurons
                        if self.nuerons_fired == nth_neurons_fired {
                                break;
                        }
                        //println!("run: {}, neurons fired: {:?} nth neurons fired: {:?}", run_count, self.nuerons_fired, nth_neurons_fired);
                        if nth_neurons_fired.iter().any(|fired| *fired) {
                                break;
                        }
                        self.nuerons_fired = nth_neurons_fired;

                        // update fired for all neurons
                        // fired is true if the neuron has ever fired
                        for i in 0..COUNT {
                                fired[i] = fired[i] || nth_neurons_fired[i];
                        }

                        // increment run_count
                        run_count += 1;
                        //println!("run: {}, neurons fired: {:?}", run_count, nth_neurons_fired);
                }

                //println!("run: {}, neurons fired: {:?}", run_count + 1, nth_neurons_fired);

                // return the sum of the last set of neurons that fired
                nth_neurons_fired.iter().zip(self.neurons.iter()).filter(|(fired, _)| **fired).map(|(_, neuron)| neuron.clone()).sum()
	}


        /// train the AI
        /// 1. the AI is trained by running the AI with a set of inputs and expected outputs
        /// 
	pub fn train(&mut self, dataset: Vec<(BigDecimal, BigDecimal)>) {
                todo!("train the AI");
	}
}
