use std::fmt::Formatter;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("Unknown distro/variant {expected} got {got}")]
    UnknownVariant { expected: Distro, got: String },
    #[error("Invalid value")]
    Invalid { value: String },
    #[error("Failed to read file: {reason}")]
    ReadFailure {
        reason: String,
        #[source]
        err: std::io::Error,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Distro {
    Debian,
    Arch,
    Redhat,
    Suse,
    /// Other will include unknown or independent Linux/Unix variants
    /// Such as NixOS or Void Linux.
    ///
    /// Running ```is_supported()``` on the ```Other``` variant
    /// will return true, however; this will require custom instructions
    /// or more manual intervention than usually necessary
    Other(String),
}

impl Distro {
    // pub fn is_debian(&self) -> bool {}
    pub fn is_supported(&self) -> bool {
        match self {
            Distro::Debian => true,
            Distro::Arch => true,
            Distro::Redhat => false,
            Distro::Suse => false,
            Distro::Other(_) => true,
        }
    }
}

impl std::fmt::Display for Distro {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Distro::Debian => "Debian",
            Distro::Arch => "Arch",
            Distro::Redhat => "Redhat",
            Distro::Suse => "Suse",
            Distro::Other(s) => s,
        };
        write!(f, "{}", string)
    }
}

pub fn current_distro() -> Result<Distro, IdentityError> {
    let identity_path = Path::new("/etc/os-release");
    if !identity_path.exists() {
        return Err(IdentityError::UnknownVariant {
            expected: Distro::Other("Any".to_string()),
            got: "Unknown".to_string(),
        });
    };
    let identity_str = std::fs::read_to_string(identity_path);
    match identity_str {
        Ok(identity_str) => {
            if !identity_str.contains("ID=") {
                return Err(IdentityError::UnknownVariant {
                    expected: Distro::Other("Any".to_string()),
                    got: "Unknown".to_string(),
                });
            };
            let ident = identity_str
                .lines()
                .find(|line| line.starts_with("ID="))
                .and_then(|line| line.split('=').nth(1))
                .unwrap_or("Unknown");
            match ident {
                "debian" => Ok(Distro::Debian),
                "ubuntu" => Ok(Distro::Debian),
                "arch" => Ok(Distro::Arch),
                "centos" => Ok(Distro::Redhat),
                "rhel" => Ok(Distro::Redhat),
                "fedora" => Ok(Distro::Redhat),
                "rocky" => Ok(Distro::Redhat),
                "opensuse-leap" => Ok(Distro::Suse),
                "opensuse" => Ok(Distro::Suse),
                "opensuse-tumbleweed" => Ok(Distro::Suse),

                _ => Ok(Distro::Other(ident.to_string())),
            }
        }
        Err(e) => Err(IdentityError::ReadFailure {
            reason: "Failed to read /etc/os-release".to_string(),
            err: e,
        }),
    }
}
// pub fn current_distro_id() -> Result<String, IdentityError> {}
