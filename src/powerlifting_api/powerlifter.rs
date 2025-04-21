use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct PowerlifterInline(
    u32,
    u32,
    String,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    String,
    String,
    String,
    Option<String>,
    String,
    String,
    String,
    String,
    Option<String>,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
);

#[derive(Clone, Debug, PartialEq)]
pub struct Powerlifter {
    pub sorted_index: u32,
    pub rank: u32,
    pub name: String,
    pub username: String,
    pub instagram: Option<String>,
    pub color: Option<String>,
    pub lifter_country: Option<String>,
    pub lifter_state: Option<String>,
    pub federation: String,
    pub date: String,
    pub meet_country: String,
    pub meet_state: Option<String>,
    pub path: String,
    pub sex: String,
    pub equipment: String,
    pub age: String,
    pub division: Option<String>,
    pub bodyweight: String,
    pub weightclass: String,
    pub squat: String,
    pub bench: String,
    pub deadlift: String,
    pub total: String,
    pub points: String,
}

impl Into<Powerlifter> for PowerlifterInline {
    fn into(self) -> Powerlifter {
        Powerlifter {
            sorted_index: self.0,
            rank: self.1,
            name: self.2,
            username: self.3,
            instagram: self.4,
            color: self.5,
            lifter_country: self.6,
            lifter_state: self.7,
            federation: self.8,
            date: self.9,
            meet_country: self.10,
            meet_state: self.11,
            path: self.12,
            sex: self.13,
            equipment: self.14,
            age: self.15,
            division: self.16,
            bodyweight: self.17,
            weightclass: self.18,
            squat: self.19,
            bench: self.20,
            deadlift: self.21,
            total: self.22,
            points: self.23,
        }
    }
}

impl<'de> Deserialize<'de> for Powerlifter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data: PowerlifterInline = <PowerlifterInline>::deserialize(deserializer)?;
        Ok(data.into())
    }
}
