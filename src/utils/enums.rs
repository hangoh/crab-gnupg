use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Operation {
    NotSet,
    Verify, // this means verify if gpg was installed and is working, gpg operation verify file was under another naming
    GenerateKey,
    ListKey,
    SearchKey,
    ImportKey,
    TrustKey,
    ExportPublicKey,
    ExportSecretKey,
    Encrypt,
    Decrypt,
    Sign,
    VerifyFile,
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::NotSet => write!(f, "NotSet"),
            Operation::Verify => write!(f, "Verify"),
            Operation::GenerateKey => write!(f, "GenerateKey"),
            Operation::ListKey => write!(f, "ListKey"),
            Operation::SearchKey => write!(f, "SearchKey"),
            Operation::ImportKey => write!(f, "ImportKey"),
            Operation::TrustKey => write!(f, "TrustKey"),
            Operation::ExportPublicKey => write!(f, "ExportPublicKey"),
            Operation::ExportSecretKey => write!(f, "ExportSecretKey"),
            Operation::Encrypt => write!(f, "Encrypt"),
            Operation::Decrypt => write!(f, "Decrypt"),
            Operation::Sign => write!(f, "Sign"),
            Operation::VerifyFile => write!(f, "VerifyFile"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TrustLevel {
    Expired,
    Undefined,
    Never,
    Marginal,
    Fully,
    Ultimate,
}

impl TrustLevel {
    pub fn value(&self) -> u8 {
        match &self {
            TrustLevel::Expired => 1,
            TrustLevel::Undefined => 2,
            TrustLevel::Never => 3,
            TrustLevel::Marginal => 4,
            TrustLevel::Fully => 5,
            TrustLevel::Ultimate => 6,
        }
    }
}
