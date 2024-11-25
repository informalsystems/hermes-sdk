#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VerificationStatus {
    Unverified,
    Verified,
    Trusted,
}
