
pub enum ResultCode {
    Success,
    DataNotFound,
    OtherDbErr,
    OtherErr,
    Undefined,
}

impl ResultCode {
    pub fn code(&self) -> &str {
        match self {
            Self::Success => "00",
            Self::DataNotFound => "2001",
            Self::OtherDbErr => "2002",
            Self::OtherErr => "2003",
            _ => "undefined",
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::Success => "Success",
            Self::DataNotFound => "Data is not Found",
            Self::OtherDbErr => "Other Db err",
            Self::OtherErr => "Other err",
            _ => "undefined",
        }
    }
}

impl From<&str> for ResultCode {
    fn from(value: &str) -> Self {
        match value {
            "00" => Self::Success,
            "2001" => Self::DataNotFound,
            "2002" => Self::OtherDbErr,
            _ => Self::Undefined,
        }
    }
}