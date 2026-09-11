use std::collections::VecDeque;
use uuid::Uuid;
use crate::notification::{Notification, NotificationLevel, NotificationCategory};

pub struct NotificationManager {
    notifications: VecDeque<Notification>,
    max_size: usize,
}

impl NotificationManager {
    pub fn new() -> Self { Self { notifications: VecDeque::new(), max_size: 200 } }
    pub fn with_max_size(max_size: usize) -> Self { Self { notifications: VecDeque::new(), max_size } }

    pub fn notify(&mut self, title: &str, message: &str, level: NotificationLevel, category: NotificationCategory) -> Uuid {
        let notification = Notification::new(title, message, level, category);
        let id = notification.id;
        self.notifications.push_front(notification);
        if self.notifications.len() > self.max_size { self.notifications.pop_back(); }
        id
    }

    pub fn notify_with_action(&mut self, title: &str, message: &str, level: NotificationLevel, category: NotificationCategory, action_label: &str, action_id: &str) -> Uuid {
        let mut notification = Notification::new(title, message, level, category);
        let id = notification.id;
        notification.action_label = Some(action_label.to_string());
        notification.action_id = Some(action_id.to_string());
        self.notifications.push_front(notification);
        if self.notifications.len() > self.max_size { self.notifications.pop_back(); }
        id
    }

    pub fn mark_read(&mut self, id: &Uuid) {
        if let Some(n) = self.notifications.iter_mut().find(|n| n.id == *id) { n.read = true; }
    }

    pub fn mark_all_read(&mut self) {
        for n in self.notifications.iter_mut() { n.read = true; }
    }

    pub fn dismiss(&mut self, id: &Uuid) {
        if let Some(n) = self.notifications.iter_mut().find(|n| n.id == *id) { n.dismissed = true; }
    }

    pub fn remove(&mut self, id: &Uuid) { self.notifications.retain(|n| n.id != *id); }
    pub fn clear_all(&mut self) { self.notifications.clear(); }
    pub fn clear_dismissed(&mut self) { self.notifications.retain(|n| !n.dismissed); }

    pub fn all(&self) -> Vec<&Notification> { self.notifications.iter().collect() }
    pub fn unread(&self) -> Vec<&Notification> { self.notifications.iter().filter(|n| !n.read).collect() }
    pub fn unread_count(&self) -> usize { self.notifications.iter().filter(|n| !n.read).count() }
    pub fn by_category(&self, category: &NotificationCategory) -> Vec<&Notification> {
        self.notifications.iter().filter(|n| n.category == *category).collect()
    }
    pub fn by_level(&self, level: &NotificationLevel) -> Vec<&Notification> {
        self.notifications.iter().filter(|n| n.level == *level).collect()
    }
    pub fn recent(&self, count: usize) -> Vec<&Notification> {
        self.notifications.iter().take(count).collect()
    }
    pub fn count(&self) -> usize { self.notifications.len() }
    pub fn has_unread_errors(&self) -> bool {
        self.notifications.iter().any(|n| !n.read && matches!(n.level, NotificationLevel::Error))
    }
}
