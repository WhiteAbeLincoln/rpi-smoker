use evalexpr::build_operator_tree;
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ParseableExpr {
    pub str: String,
    pub data: evalexpr::Node,
}
impl Serialize for ParseableExpr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.str)
    }
}
struct ParseableExprVisitor;
impl<'de> Deserialize<'de> for ParseableExpr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ParseableExprVisitor)
    }
}
impl<'de> Visitor<'de> for ParseableExprVisitor {
    type Value = ParseableExpr;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a string-encoded expression")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match build_operator_tree(v) {
            Ok(node) => Ok(ParseableExpr {
                str: v.to_string(),
                data: node,
            }),
            Err(error) => Err(E::custom(error)),
        }
    }
}

pub type Temp = f64;
