#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Status {
    Initialised,
    InProgress,
    Finished,
    Unknown,
}

impl Status {
    pub fn from_str(status: &str) -> Status {
        match status.to_lowercase().as_str() {
            "initialised" => Status::Initialised,
            "inprogress" => Status::InProgress,
            "finished" => Status::Finished,
            _ => Status::Unknown,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match *self {
            Status::Initialised => "Initialised",
            Status::InProgress => "InProgress",
            Status::Finished => "Finished",
            Status::Unknown => "Unknown",
        }
    }
}
