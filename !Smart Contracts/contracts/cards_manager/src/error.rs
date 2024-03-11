use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum ContractError {
    #[error("StdError: {0}")]
    StdError(#[from] cosmwasm_std::StdError),
    #[error("Unauthorized")]
    Unauthorized {},
}
