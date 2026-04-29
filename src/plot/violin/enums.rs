use serde::ser::{Serialize, Serializer};

#[derive(Clone, Debug)]
pub enum ViolinPoints {
    All,
    Outliers,
    SuspectedOutliers,
    False,
}

impl Serialize for ViolinPoints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::Outliers => serializer.serialize_str("outliers"),
            Self::SuspectedOutliers => serializer.serialize_str("suspectedoutliers"),
            Self::False => serializer.serialize_bool(false),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ViolinHoverOn {
    All,
    Violins,
    Points,
    Kde,
    ViolinsPoints,
    ViolinsKde,
    PointsKde,
    ViolinsPointsKde,
    Custom(String),
}

impl Serialize for ViolinHoverOn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::All => "all",
            Self::Violins => "violins",
            Self::Points => "points",
            Self::Kde => "kde",
            Self::ViolinsPoints => "violins+points",
            Self::ViolinsKde => "violins+kde",
            Self::PointsKde => "points+kde",
            Self::ViolinsPointsKde => "violins+points+kde",
            Self::Custom(s) => s.as_str(),
        };
        serializer.serialize_str(s)
    }
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ViolinScaleMode {
    Width,
    Count,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ViolinSide {
    Both,
    Positive,
    Negative,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ViolinSpanMode {
    Soft,
    Hard,
    Manual,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum QuartileMethod {
    Linear,
    Exclusive,
    Inclusive,
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ViolinMode {
    Group,
    Overlay,
}
