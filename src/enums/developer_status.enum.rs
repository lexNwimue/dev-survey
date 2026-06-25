
// --- DeveloperStatus ---
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DeveloperStatus {
    ProfessionalDev,        // "I am a developer by profession"
    StudentLearning,        // "I am a student learning to code"
    SelfTaughtUnemployed,   // "I am self-taught but not currently employed as a developer"
    HobbyDev,               // "I code primarily as a hobby"
    TechNonDev,             // "I work in tech but don't write code"
    FormerDev,              // "I used to be a developer professionally"
    NonDev,                 // "I am not a developer and don't write code"
}