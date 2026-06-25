// --- AgeRange ---
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AgeRange {
    Under18,   // "Under 18"
    R1824,     // "18–24"
    R2534,     // "25–34"
    R3544,     // "35–44"
    R4554,     // "45–54"
    R55Plus,   // "55 and above"
}