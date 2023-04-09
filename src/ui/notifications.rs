use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct NotificationDisplay {
    notifications: Vec<Notification>,
    current_duration: Duration,
}

impl NotificationDisplay {
    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    fn progress_notification(&mut self, duration: Duration) {
        if let Some(current_notification) = self.notifications.get(0) {
            self.current_duration += duration;
            if self.current_duration >= current_notification.duration {
                self.current_duration = Duration::from_secs(0);
                self.notifications.remove(0);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub text: String,
    pub duration: Duration,
}

const DEFAULT_NOTIFICATION_DURATION: Duration = Duration::from_secs(2);

impl Notification {
    pub fn new(text: String) -> Self {
        Self {
            text,
            duration: DEFAULT_NOTIFICATION_DURATION,
        }
    }
}

pub struct CleanNotificationQueue;

#[derive(Component)]
pub struct NotificationLine;

pub fn add_notifications_ui(child_builder: &mut ChildBuilder, text_style: &TextStyle) {
    child_builder
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(80.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    color: Color::WHITE,
                    ..text_style.clone()
                },
            ));
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(3.0)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .insert(NotificationLine);
        })
        .insert(NotificationDisplay::default());
}

pub fn update_notifications(
    mut query: Query<(&mut NotificationDisplay, &Children)>,
    mut text_q: Query<&mut Text>,
    mut line_q: Query<&mut Style, With<NotificationLine>>,
    time: Res<Time>,
) {
    for (mut shower, children) in query.iter_mut() {
        shower.progress_notification(time.delta());
        if let Some(current_notification) = shower.notifications.get(0) {
            for child in children.iter() {
                if let Ok(mut text) = text_q.get_mut(*child) {
                    text.sections[0].value = current_notification.text.to_string();
                }
                if let Ok(mut style) = line_q.get_mut(*child) {
                    style.size.width = Val::Percent(
                        (1. - shower.current_duration.as_secs_f32()
                            / current_notification.duration.as_secs_f32())
                            * 100.0,
                    );
                }
            }
        } else {
            for child in children.iter() {
                if let Ok(mut text) = text_q.get_mut(*child) {
                    text.sections[0].value = "".to_string();
                }
                if let Ok(mut style) = line_q.get_mut(*child) {
                    style.size.width = Val::Percent(0.0);
                }
            }
        }
    }
}

pub fn display_notifications(
    mut event_reader: EventReader<Notification>,
    mut notification_display_q: Query<&mut NotificationDisplay>,
) {
    for notification in event_reader.iter() {
        for mut display in notification_display_q.iter_mut() {
            display.add_notification(notification.clone());
        }
    }
}

pub fn clean_notifications(
    mut event_reader: EventReader<CleanNotificationQueue>,
    mut notification_display_q: Query<&mut NotificationDisplay>,
) {
    if !event_reader.is_empty() {
        event_reader.clear();
        for mut display in notification_display_q.iter_mut() {
            display.notifications.clear();
            display.current_duration = Duration::from_secs(0);
        }
    }
}
