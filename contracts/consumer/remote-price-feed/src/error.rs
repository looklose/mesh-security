use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use mesh_apis::ibc::VersionError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    IbcVersion(#[from] VersionError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid authorized endpoint: {0}")]
    InvalidEndpoint(String),

    #[error("Contract already has an open IBC channel")]
    IbcChannelAlreadyOpen,

    #[error("You must start the channel handshake on the other side, it doesn't support OpenInit")]
    IbcOpenInitDisallowed,

    #[error("Contract does not accept ack packets")]
    IbcAckNotAccepted,

    #[error("The oracle hasn't received any price data")]
    NoPriceData,
}
