use std::cmp::Ordering;
use std::collections::BinaryHeap;
use chrono::{DateTime, Utc};
use super::event::Event;

pub struct EventQueue {
    queue: BinaryHeap<TimedEvent>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct TimedEvent {
    time: DateTime<Utc>,
    event: Event,
}

impl EventQueue {
    pub fn new() -> Self {
        EventQueue {
            queue: BinaryHeap::new(),
        }
    }
    pub fn add_event(&mut self, event: Event, time: DateTime<Utc>) {
        self.queue.push(
            TimedEvent{
                time: time, 
                event: event,
            }
        );
    }
    /// Returns the next event in the queue if it is due 
    pub fn poll_event(&self) -> Option<&Event> {
        self.queue.peek()
            .filter(|te| te.time <= chrono::Utc::now())
            .map(|te| &te.event)
    }
    pub fn time_of_next_event(&self) -> Option<&DateTime<Utc>> {
        self.queue.peek().map(|te| &te.time)
    }
}



// Necessary traits for Binary Heap

impl Ord for TimedEvent {
    fn cmp(&self, other: &TimedEvent) -> Ordering {
        // Flipped order to make it a Min-Heap
        // With event as tie-break
        other.time.cmp(&self.time)
            .then_with(|| self.event.cmp(&other.event))
    }
}
impl PartialOrd for TimedEvent {
    fn partial_cmp(&self, other: &TimedEvent) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


