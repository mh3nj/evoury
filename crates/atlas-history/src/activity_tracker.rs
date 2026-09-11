use atlas_core::{ActivityEvent, ActivityEventType};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ActivityTracker {
    pub events: Vec<ActivityEvent>,
}

impl ActivityTracker {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn record_event(
        &mut self,
        asset_id: Uuid,
        event_type: ActivityEventType,
        details: &str,
    ) -> ActivityEvent {
        let event = ActivityEvent::new(asset_id, event_type, details);
        self.events.push(event.clone());
        event
    }

    pub fn get_events_for_asset(&self, asset_id: &Uuid, limit: usize) -> Vec<&ActivityEvent> {
        self.events
            .iter()
            .rev()
            .filter(|e| &e.asset_id == asset_id)
            .take(limit)
            .collect()
    }

    pub fn get_all_events(&self, limit: usize, offset: usize) -> Vec<&ActivityEvent> {
        self.events
            .iter()
            .rev()
            .skip(offset)
            .take(limit)
            .collect()
    }

    pub fn get_events_by_type(
        &self,
        event_type: &ActivityEventType,
        limit: usize,
    ) -> Vec<&ActivityEvent> {
        self.events
            .iter()
            .rev()
            .filter(|e| std::mem::discriminant(&e.event_type) == std::mem::discriminant(event_type))
            .take(limit)
            .collect()
    }

    pub fn get_events_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<&ActivityEvent> {
        self.events
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    pub fn recent_activity(&self, hours: i64) -> Vec<&ActivityEvent> {
        let cutoff = Utc::now() - Duration::hours(hours);
        self.events
            .iter()
            .rev()
            .filter(|e| e.timestamp >= cutoff)
            .collect()
    }

    pub fn count_events_for_asset(&self, asset_id: &Uuid) -> usize {
        self.events
            .iter()
            .filter(|e| &e.asset_id == asset_id)
            .count()
    }

    pub fn clear_before(&mut self, timestamp: DateTime<Utc>) {
        self.events.retain(|e| e.timestamp >= timestamp);
    }

    pub fn most_viewed(&self, limit: usize) -> Vec<(Uuid, usize)> {
        let mut counts: HashMap<Uuid, usize> = HashMap::new();
        for event in &self.events {
            if std::mem::discriminant(&event.event_type)
                == std::mem::discriminant(&ActivityEventType::View)
            {
                *counts.entry(event.asset_id).or_insert(0) += 1;
            }
        }
        let mut sorted: Vec<(Uuid, usize)> = counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.truncate(limit);
        sorted
    }

    pub fn last_opened(&self, asset_id: &Uuid) -> Option<DateTime<Utc>> {
        self.events
            .iter()
            .rev()
            .find(|e| {
                &e.asset_id == asset_id
                    && std::mem::discriminant(&e.event_type)
                        == std::mem::discriminant(&ActivityEventType::Open)
            })
            .map(|e| e.timestamp)
    }
}
