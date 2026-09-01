use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

pub struct Bucket {
    pub sum: f64,
    pub count: usize,
}

pub struct SharedState{
    pub map: HashMap<usize, Bucket>,
    pub last_averages: Vec<Average>,
}

pub struct Aggregator {
    shared_state: Arc<Mutex<SharedState>>,
    running: Arc<AtomicBool>,
    worker_handle: Option<JoinHandle<()>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Average {
    pub sensor_id: usize,
    pub reference_time: Instant,        //indica l'istante temporale in cui è stata calcolata la media
    pub average_temperature: f64,
}

impl Aggregator {
    pub fn new(sample_time_millis: u64) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            map: HashMap::new(),
            last_averages: Vec::new(),
        }));
        let running = Arc::new(AtomicBool::new(true));
        let worker_handle = {
            let shared_state = Arc::clone(&shared_state);
            let running = Arc::clone(&running);
            std::thread::spawn(move || {
                while running.load(Ordering::SeqCst) {
                    std::thread::sleep(Duration::from_millis(sample_time_millis));
                    let mut state = shared_state.lock().unwrap();
                    let now = Instant::now();
                    let mut averages = Vec::new();
                    for (sensor_id, bucket) in state.map.iter() {
                        if bucket.count > 0 {
                            let average_temperature = bucket.sum / bucket.count as f64;
                            averages.push(Average {
                                sensor_id: *sensor_id,
                                reference_time: now,
                                average_temperature,
                            });
                        }
                    }
                    state.last_averages = averages;
                    state.map.clear();
                }
            })
        };

        Aggregator {
            shared_state,
            running,
            worker_handle: Some(worker_handle),
        }
    }

    pub fn add_measure(&self, sensor_id: usize, temperature: f64) {
        // aggiunge una misura di temperatura per il sensore con id `sensor_id`
        // e temperatura `temperature`. Le misure sono automaticamente etichettate
        // con l'istante temporale in cui sono comunicate.
        let mut state = self.shared_state.lock().unwrap();
        let bucket = state.map.entry(sensor_id).or_insert(Bucket { sum: 0.0, count: 0 });
        bucket.sum += temperature;
        bucket.count += 1;
    }

    pub fn get_averages(&self) -> Vec<Average> {
        // restituisce un vettore che riporta la temperatura media di ciascun sensore,
        // calcolata durante l'ultimo periodo di campionamento.
        // Sono presenti solo i sensori che hanno inviato almeno una misura.
        let state = self.shared_state.lock().unwrap();
        state.last_averages.clone()
    }
}

impl Drop for Aggregator {
    fn drop(&mut self) {
        // Segnaliamo al worker di fermarsi e aspettiamo che finisca
        // (join), così quando l'Aggregator viene distrutto non resta
        // nessun thread "orfano" in esecuzione.
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
use super::*;

    #[test]
    fn when_no_measures_are_sent_an_empty_state_is_returned() {
        let aggregator = Aggregator::new(10);
        let averages = aggregator.get_averages();
        assert!(averages.is_empty());
    }

    #[test]
    fn when_a_single_measure_is_sent_it_is_returned() {
        let aggregator = Aggregator::new(20);
        std::thread::sleep(std::time::Duration::from_millis(1));
        aggregator.add_measure(1, 1.0);
        assert!(aggregator.get_averages().is_empty());
        std::thread::sleep(Duration::from_millis(25));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(),  1);
        assert!(matches!(averages.get(0), Some(&Average{ sensor_id:1, average_temperature:1.0, .. })));
    }
    #[test]
    fn when_two_measures_are_sent_their_average_is_returned() {
        let aggregator = Aggregator::new(100);
        aggregator.add_measure(1, 1.0);
        aggregator.add_measure(1, 2.0);
        std::thread::sleep(Duration::from_millis(110));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(),  1);
        assert!(matches!(averages.get(0), Some(&Average{ sensor_id:1, average_temperature:1.5, .. })));
    }
    #[test]
    fn when_two_measures_are_sent_from_different_sensors_their_average_is_returned() {
        let aggregator = Aggregator::new(100);
        aggregator.add_measure(1, 1.0);
        aggregator.add_measure(2, 2.0);
        aggregator.add_measure(2, 1.0);
        aggregator.add_measure(1, 2.0);
        std::thread::sleep(Duration::from_millis(110));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(),  2);
        let timestamp = averages.get(0).unwrap().reference_time;
        assert!(averages.contains(&Average{ sensor_id:1, average_temperature:1.5, reference_time: timestamp }));
        assert!(averages.contains(&Average{ sensor_id:2, average_temperature:1.5, reference_time: timestamp }));
    }

    #[test]
    fn more_threads_may_send_data() {
        let aggregator = Aggregator::new(100);
        std::thread::scope(|s| {
            s.spawn(|| {
                aggregator.add_measure(1, 1.0);
                std::thread::sleep(Duration::from_millis(5));
                aggregator.add_measure(1, 3.0);
            });
            s.spawn(|| {
                aggregator.add_measure(2, 2.0);
                std::thread::sleep(Duration::from_millis(5));
                aggregator.add_measure(2, 8.0);
            });
        });
        std::thread::sleep(Duration::from_millis(110));
        let averages = aggregator.get_averages();
        assert_eq!(averages.len(),  2);
        let timestamp = averages.get(0).unwrap().reference_time;
        assert!(averages.contains(&Average{ sensor_id:1, average_temperature:2.0, reference_time: timestamp }));
        assert!(averages.contains(&Average{ sensor_id:2, average_temperature:5.0, reference_time: timestamp }));
    }
    #[test]
    fn an_aggregator_shuts_down_cleanly() {
        {
            let _aggregator = Aggregator::new(10);
        }
        assert!(true);
    }
}

fn main() {
    // implementazione del main
}
