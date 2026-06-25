
// --- ExperienceRange ---
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ExperienceRange {
    LessThan1,  // "Less than 1 year"
    R13,        // "1–3 years"
    R35,        // "3–5 years"
    R510,       // "5–10 years"
    R1015,      // "10–15 years"
    R15Plus,    // "15+ years"
}