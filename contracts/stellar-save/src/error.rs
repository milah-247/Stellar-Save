use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Error {
    GroupNotFound = 1,
    GroupFull = 2,
    AlreadyMember = 3,
    NotMember = 4,
    AlreadyContributed = 5,
    CyclePending = 6,
    GroupComplete = 7,
    TransferFailed = 8,
    InvalidConfig = 9,
}
