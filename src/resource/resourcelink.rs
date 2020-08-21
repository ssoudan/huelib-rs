use crate::resource;
use derive_setters::Setters;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/// A resourcelink to group resources in the bridge.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Resourcelink {
    /// Identifier of the resourcelink.
    #[serde(skip)]
    pub id: String,
    /// Name of the resourcelink.
    pub name: String,
    /// Description of the resourcelink.
    pub description: String,
    /// Owner of the resourcelink.
    pub owner: String,
    /// Kind of the resourcelink.
    #[serde(rename = "type")]
    pub kind: Kind,
    /// Class identifier of the resourcelink.
    #[serde(rename = "classid")]
    pub class_id: u16,
    /// Whether the resource is automatically deleted when not referenced anymore.
    pub recycle: bool,
    /// References to resources which are used by this resourcelink.
    pub links: Vec<Link>,
}

impl resource::Resource for Resourcelink {}

impl Resourcelink {
    pub(crate) fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
}

/// Kind of a resourcelink.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Kind {
    /// The only variant.
    Link,
}

/// A reference to a resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Link {
    /// Kind of the resource.
    pub kind: LinkKind,
    /// Identifier of the resource.
    pub id: String,
}

impl<'de> Deserialize<'de> for Link {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value: String = Deserialize::deserialize(deserializer)?;
        let mut values: Vec<&str> = value.split('/').collect();
        let id_str = values
            .pop()
            .ok_or_else(|| D::Error::custom("expected link in the format /<kind>/<id>"))?;
        let kind_str = values
            .pop()
            .ok_or_else(|| D::Error::custom("expected link in the format /<kind>/<id>"))?;
        Ok(Self {
            kind: LinkKind::from_str(kind_str)
                .ok_or_else(|| D::Error::custom(format!("invalid link type '{}'", kind_str)))?,
            id: id_str.to_owned(),
        })
    }
}

impl Serialize for Link {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("/{}/{}", self.kind.to_str(), self.id))
    }
}

/// Kind of a link.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LinkKind {
    Group,
    Light,
    Resourcelink,
    Rule,
    Scene,
    Schedule,
    Sensor,
}

impl LinkKind {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "groups" => Some(Self::Group),
            "lights" => Some(Self::Light),
            "resourcelinks" => Some(Self::Resourcelink),
            "rules" => Some(Self::Rule),
            "scenes" => Some(Self::Scene),
            "schedules" => Some(Self::Schedule),
            "sensors" => Some(Self::Sensor),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::Group => "groups",
            Self::Light => "lights",
            Self::Resourcelink => "resourcelinks",
            Self::Rule => "rules",
            Self::Scene => "scenes",
            Self::Schedule => "schedules",
            Self::Sensor => "sensors",
        }
    }
}

/// Struct for creating a resourcelink.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Creator {
    /// Sets the name of the resourcelink.
    #[setters(skip)]
    pub name: String,
    /// Sets the description of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Sets the owner of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Sets the kind of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// Sets the class id of the resourcelink.
    #[serde(rename = "classid")]
    #[setters(skip)]
    pub class_id: u16,
    /// Sets the whether to recycle the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recycle: Option<bool>,
    /// Sets the links of the resourcelink.
    #[setters(skip)]
    pub links: Vec<Link>,
}

impl resource::Creator for Creator {}

impl Creator {
    /// Creates a new [`Creator`].
    pub fn new(name: String, class_id: u16, links: Vec<Link>) -> Self {
        Self {
            name,
            description: None,
            owner: None,
            kind: None,
            class_id,
            recycle: None,
            links,
        }
    }
}

/// Modifier for a resourcelink.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option, prefix = "with_")]
pub struct Modifier {
    /// Sets the name of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Sets the description of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Sets the class id of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// Sets the kind of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none", rename = "classid")]
    pub class_id: Option<u16>,
    /// Sets the links of the resourcelink.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Link>>,
}

impl resource::Modifier for Modifier {}

impl Modifier {
    /// Creates a new [`Modifier`].
    pub fn new() -> Self {
        Self::default()
    }
}
