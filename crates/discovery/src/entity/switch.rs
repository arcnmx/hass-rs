use crate::{
  exts::ValidateContextExt,
  entity::{Entity, EntityInvalidity},
  device_class::DeviceClass, payload::Payload, template::Template, topic::Topic,
};
use semval::{context::Context, Validate};
use serde::{Deserialize, Serialize};

/// The mqtt switch platform lets you control your MQTT enabled switches.
///
/// See: <https://www.home-assistant.io/integrations/switch.mqtt/>
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Switch<'a> {
  #[serde(borrow, flatten)]
  pub entity: Entity<'a>,

	#[serde(borrow, default, skip_serializing_if = "Option::is_none")]
	pub command_topic: Option<Topic<'a>>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub device_class: Option<DeviceClass>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub optimistic: Option<bool>,
	#[serde(borrow, default, skip_serializing_if = "Option::is_none")]
	pub payload_off: Option<Payload<'a>>,
	#[serde(borrow, default, skip_serializing_if = "Option::is_none")]
	pub payload_on: Option<Payload<'a>>,

	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub retain: Option<bool>,

	#[serde(borrow, default, skip_serializing_if = "Option::is_none")]
	pub state_off: Option<Payload<'a>>,
	#[serde(borrow, default, skip_serializing_if = "Option::is_none")]
	pub state_on: Option<Payload<'a>>,
	#[serde(borrow, default, skip_serializing_if = "Option::is_none")]
	pub state_topic: Option<Topic<'a>>,
	#[serde(borrow, default, skip_serializing_if = "Option::is_none")]
	pub value_template: Option<Template<'a>>,
}

impl<'a> Validate for Switch<'a> {
  type Invalidity = EntityInvalidity;

  fn validate(&self) -> semval::Result<Self::Invalidity> {
    Context::new()
      .validate_with(&self.entity, |v| v)
      .validate_with_opt(&self.command_topic, EntityInvalidity::Topic)
      .validate_with_opt(&self.payload_on, EntityInvalidity::Payload)
      .validate_with_opt(&self.payload_off, EntityInvalidity::Payload)
      .validate_with_opt(&self.state_topic, EntityInvalidity::Topic)
      .validate_with_opt(&self.state_on, EntityInvalidity::Payload)
      .validate_with_opt(&self.state_off, EntityInvalidity::Payload)
      .validate_with_opt(&self.value_template, EntityInvalidity::Template)
      .into()
  }
}
