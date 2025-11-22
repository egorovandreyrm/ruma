use ruma_common::{serde::from_raw_json_value, EventId};
use serde::{de, ser::SerializeStruct, Deserialize, Deserializer, Serialize};
use serde_json::value::RawValue as RawJsonValue;

use super::{
    NewUnstableStreamStartEventContent, NewUnstableStreamStartEventContentWithoutRelation,
    ReplacementUnstableStreamStartEventContent, UnstableStreamStartContentBlock,
    UnstableStreamStartEventContent,
};
use crate::room::message::{deserialize_relation, Relation};

impl<'de> Deserialize<'de> for UnstableStreamStartEventContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json = Box::<RawJsonValue>::deserialize(deserializer)?;

        let mut deserializer = serde_json::Deserializer::from_str(json.get());
        let relates_to: Option<Relation<NewUnstableStreamStartEventContentWithoutRelation>> =
            deserialize_relation(&mut deserializer).map_err(de::Error::custom)?;
        let UnstableStreamStartEventContentDeHelper { stream_start, text } =
            from_raw_json_value(&json)?;

        let c = match relates_to {
            Some(Relation::Replacement(relates_to)) => {
                ReplacementUnstableStreamStartEventContent { stream_start, text, relates_to }.into()
            }
            rel => {
                let stream_start = stream_start
                    .ok_or_else(|| de::Error::missing_field("org.matrix.msc3381.stream.start"))?;
                let relates_to = rel
                    .map(|r| r.try_into().expect("Relation::Replacement has already been handled"));
                NewUnstableStreamStartEventContent { stream_start, text, relates_to }.into()
            }
        };

        Ok(c)
    }
}

#[derive(Debug, Deserialize)]
struct UnstableStreamStartEventContentDeHelper {
    #[serde(rename = "org.matrix.msc3381.stream.start")]
    stream_start: Option<UnstableStreamStartContentBlock>,

    #[serde(rename = "org.matrix.msc1767.text")]
    text: Option<String>,
}

impl Serialize for ReplacementUnstableStreamStartEventContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = 2 + self.stream_start.is_some() as usize + self.text.is_some() as usize;

        let mut state =
            serializer.serialize_struct("ReplacementUnstableStreamStartEventContent", len)?;

        if let Some(stream_start) = &self.stream_start {
            state.serialize_field("org.matrix.msc3381.stream.start", stream_start)?;
        }
        if let Some(text) = &self.text {
            state.serialize_field("org.matrix.msc1767.text", text)?;
        }

        state.serialize_field("m.new_content", &self.relates_to.new_content)?;
        state.serialize_field(
            "m.relates_to",
            &ReplacementRelatesTo { event_id: &self.relates_to.event_id },
        )?;

        state.end()
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "rel_type", rename = "m.replace")]
struct ReplacementRelatesTo<'a> {
    event_id: &'a EventId,
}
