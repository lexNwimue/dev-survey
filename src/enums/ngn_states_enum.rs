// enums/profile.rs

// --- NigerianState ---
// Used for both state_of_origin and state_of_residence on 
// the user profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NigerianState {
    Ab, Ad, Ak, An, Ba, By, Be, Bo, Cr, De,
    Eb, Ed, Ek, En, Go, Im, Ji, Kd, Kn, Kb,
    Ko, Kw, La, Na, Ni, Og, On, Os, Oy, Pl,
    Ri, So, Ta, Yo, Za, Fc,
}

impl NigerianState {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Ab => "AB", Self::Ad => "AD", Self::Ak => "AK",
            Self::An => "AN", Self::Ba => "BA", Self::By => "BY",
            Self::Be => "BE", Self::Bo => "BO", Self::Cr => "CR",
            Self::De => "DE", Self::Eb => "EB", Self::Ed => "ED",
            Self::Ek => "EK", Self::En => "EN", Self::Go => "GO",
            Self::Im => "IM", Self::Ji => "JI", Self::Kd => "KD",
            Self::Kn => "KN", Self::Kb => "KB", Self::Ko => "KO",
            Self::Kw => "KW", Self::La => "LA", Self::Na => "NA",
            Self::Ni => "NI", Self::Og => "OG", Self::On => "ON",
            Self::Os => "OS", Self::Oy => "OY", Self::Pl => "PL",
            Self::Ri => "RI", Self::So => "SO", Self::Ta => "TA",
            Self::Yo => "YO", Self::Za => "ZA", Self::Fc => "FCT",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Ab => "Abia",          Self::Ad => "Adamawa",
            Self::Ak => "Akwa Ibom",     Self::An => "Anambra",
            Self::Ba => "Bauchi",        Self::By => "Bayelsa",
            Self::Be => "Benue",         Self::Bo => "Borno",
            Self::Cr => "Cross River",   Self::De => "Delta",
            Self::Eb => "Ebonyi",        Self::Ed => "Edo",
            Self::Ek => "Ekiti",         Self::En => "Enugu",
            Self::Go => "Gombe",         Self::Im => "Imo",
            Self::Ji => "Jigawa",        Self::Kd => "Kaduna",
            Self::Kn => "Kano",          Self::Kb => "Kebbi",
            Self::Ko => "Kogi",          Self::Kw => "Kwara",
            Self::La => "Lagos",         Self::Na => "Nasarawa",
            Self::Ni => "Niger",         Self::Og => "Ogun",
            Self::On => "Ondo",          Self::Os => "Osun",
            Self::Oy => "Oyo",           Self::Pl => "Plateau",
            Self::Ri => "Rivers",        Self::So => "Sokoto",
            Self::Ta => "Taraba",        Self::Yo => "Yobe",
            Self::Za => "Zamfara",       Self::Fc => "FCT (Abuja)",
        }
    }
}
