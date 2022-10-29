/// This module defines request types as well as their serialization and
/// validation functions.

use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, Debug)]
#[repr(u16)]
/// Represents a request type that the API can make.
pub enum ApiIpcRequestType {
    Led = 0,
    Vel = 1
}

#[derive(Deserialize, Debug)]
/// Represents a request that the API can make. This structure wraps around the
/// request, presenting metadata and a body as a string. Further structures
/// deserialize the body as required.
pub struct ApiIpcRequest {
    pub request_type: ApiIpcRequestType,
    pub body: String
}

/// This trait ensures that ApiIpc*RequestBody functions can be validated.
pub trait ValidatesApiIpcBody {
    /// Validates an IpcBody to ensure that all fields are valid.
    fn validate(&self) -> bool;
}

#[derive(Deserialize, Debug)]
/// Represents a body for the [ApiIpcRequestType.Led] request type. The fields
/// here contain the RGB values for the LED.
pub struct ApiIpcLedRequestBody { r: u8, g: u8, b: u8 }

impl ValidatesApiIpcBody for ApiIpcLedRequestBody {
    fn validate(&self) -> bool {
        let valid_range = 0..255;
        return valid_range.contains(&self.r)
            && valid_range.contains(&self.g)
            && valid_range.contains(&self.b);
    }
}

#[derive(Deserialize, Debug)]
/// Represents a velocity request body for [ApiIpcRequestType.Vel] request. The
/// fields represent the left and right motor powers.
pub struct ApiIpcVelRequestBody { l: i8, r: i8 }

impl ValidatesApiIpcBody for ApiIpcVelRequestBody {
    fn validate(&self) -> bool {
        let valid_range = -100..100;
        return valid_range.contains(&self.l)
            && valid_range.contains(&self.r);
    }
}
