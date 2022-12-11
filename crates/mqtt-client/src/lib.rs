#![cfg_attr(provide_any, feature(provide_any))]
#![cfg_attr(provide_any, feature(error_generic_member_access))]

pub mod availability;
pub mod client;
pub mod entity;
pub mod options;
pub mod router;
pub mod topics;
pub mod tracking;

pub mod error;
pub mod mqtt;

pub use client::{HassMqttClient, Message};
pub use hass_mqtt_provider::QosLevel;
pub use hass_mqtt_types as types;
pub use options::{HassMqttOptions, MqttOptionsError, MqttPersistenceError};
pub use entity::{EntityTopic, StateTopic, CommandTopic};
