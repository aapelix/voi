use ringbuf::traits::Split;
use std::sync::{Arc, Mutex};

type SharedProducer = Arc<Mutex<ringbuf::HeapProd<f32>>>;
type SharedConsumer = Arc<Mutex<ringbuf::HeapCons<f32>>>;

#[derive(Clone)]
pub struct AudioBuffer {
    pub producer: SharedProducer,
    pub consumer: SharedConsumer,
}

impl AudioBuffer {
    pub fn new(size: usize) -> Self {
        let rb = ringbuf::HeapRb::<f32>::new(size);
        let (producer, consumer) = rb.split();

        Self {
            producer: Arc::new(Mutex::new(producer)),
            consumer: Arc::new(Mutex::new(consumer)),
        }
    }
}
