/// This module defines request types as well as their serialization and
/// validation functions.
use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, Debug)]
#[repr(u16)]
/// Represents a request type that the API can make.
pub enum ApiIpcRequestType {
    /// Represents an LED change request.
    Led = 0,
    /// Represents a velocity change request.
    Vel = 1,
    /// Represents a position read request.
    Pos = 2,
}

#[derive(Deserialize, Debug)]
/// Represents a request that the API can make. This structure wraps around the
/// request, presenting metadata and a body as a string. Further structures
/// deserialize the body as required.
pub struct ApiIpcRequest {
    pub request_type: ApiIpcRequestType,
    pub body: String,
}

/// This trait ensures that ApiIpc*RequestBody functions can be validated.
pub trait ValidatesApiIpcBody {
    /// Validates an IpcBody to ensure that all fields are valid.
    fn validate(&self) -> bool;
}

#[derive(Deserialize, Debug)]
/// Represents a body for the [ApiIpcRequestType.Led] request type. The fields
/// here contain the RGB values for the LED.
pub struct ApiIpcLedRequestBody {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ValidatesApiIpcBody for ApiIpcLedRequestBody {
    fn validate(&self) -> bool {
        let valid_range = 0..=255;
        return valid_range.contains(&self.r)
            && valid_range.contains(&self.g)
            && valid_range.contains(&self.b);
    }
}

#[derive(Deserialize, Debug)]
/// Represents a velocity request body for [ApiIpcRequestType::Vel]. The
/// fields represent the left and right motor powers.
pub struct ApiIpcVelRequestBody {
    pub l: i8,
    pub r: i8,
}

impl ValidatesApiIpcBody for ApiIpcVelRequestBody {
    fn validate(&self) -> bool {
        let valid_range = -100..=100;
        return valid_range.contains(&self.l) && valid_range.contains(&self.r);
    }
}

#[derive(Deserialize, Debug)]
/// Represents a request body for [ApiIpcPosRequestType::Pos].
pub struct ApiIpcPosRequestBody {}

impl ValidatesApiIpcBody for ApiIpcPosRequestBody {
    fn validate(&self) -> bool {
        return true;
    }
}
