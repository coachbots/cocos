use log::debug;
use serde::Deserialize;
/// This module exposes the [ApiMessager] class which is responsible for
/// communication with the API.

use zmq;
use crate::controllers::api::ipc_responses::ApiIpcErrorResponseBody;

use super::errors::ApiError;
use super::ipc_requests::{
    ApiIpcRequestType,
    ApiIpcRequest,
    ApiIpcLedRequestBody,
    ApiIpcVelRequestBody,
    ValidatesApiIpcBody
};
use super::ipc_responses::{
    ApiStatus,
    ApiResponse,
    ApiIpcLedResponseBody,
    ApiIpcVelResponseBody
};

/// Represents the main class used for communication with the API.
///
/// This object is responsible for the underlying ZMQ transactions as well as
/// parsing requests and giving responses.
pub struct ApiMessager {
    /// The UNIX file used for IPC communication.
    pub comm_file: &'static str,

    /// The zmq context used for communication.
    context: zmq::Context,

    /// The zmq socket used for communication. A [None] value represents either
    /// an unusable socket and/or an unconstructed socket.
    socket: Option<zmq::Socket>
}

impl ApiMessager {
    /// Constructs a new ApiMessager given the communication file uri.
    ///
    /// * `comm_file` - A static string of the form `ipc:///path/to/sock`
    ///                 through which communication is done.
    pub fn new(comm_file: &'static str) -> ApiMessager {
        ApiMessager {
            comm_file,
            context: zmq::Context::new(),
            socket: Option::None
        }
    }

    /// Starts the messager, initializing the socket, returning an error if an
    /// error ocurred. An error implies that the ApiMessager is unusable.
    pub fn start(&mut self) -> Result<(), ApiError> {
        match self.context.socket(zmq::REP) {
            Ok(sock) => {
                if let Err(zmq_err) = sock.bind(self.comm_file) {
                    return Err(ApiError::ZMQError(zmq_err));
                };

                self.socket = Some(sock);
                Ok(())
            }
            Err(err) => {
                Err(ApiError::ZMQError(err))
            }
        }
    }

    /// Immediately stops the messager, closing any open sockets.
    pub fn stop(&mut self) {
        match &mut self.socket {
            None => {}
            Some(sock) => {
                self.socket = Option::None;
            }
        }
    }

    /// A function that must be hooked into a looped task, reading a message,
    /// parsing it and emitting the requested instruction into the appropriate
    /// channel.
    ///
    /// This function returns an [ApiError::SockNotReady] if the socket is not
    /// well prepared. An [ApiError::ZMQError] is thrown for all zmq-related
    /// errors. These errors are unrecoverable the API will not be notified.
    ///
    /// [ApiError::DecodeError] is raised when the bytes could not be decoded,
    /// however, the API is notified via an [ApiStatus::InvalidEncoding].
    pub fn run_tick(&mut self) -> Result<(), ApiError> {
        match &self.socket {
            None => { Err(ApiError::SockNotReady) }
            Some(sock) => {
                // Receive the message, raising an error if an error ocurred.
                let message_res = sock.recv_msg(0);
                if let Err(err) = message_res {
                    return Err(ApiError::ZMQError(err));
                }
                let message = message_res.unwrap();
                log::debug!(target: "system.api.messager",
                            "Received raw message: {:?}", message);

                // Ensure the data received is valid UTF-8.
                if message.as_str().is_none() {
                    let response = serde_json::to_string(&ApiResponse {
                        status: ApiStatus::InvalidEncoding,
                        body: serde_json::to_string(&ApiIpcErrorResponseBody {
                            message: "The character encoding is not UTF-8. 
                                Are you trying something funny?".to_string()
                        }).unwrap()
                    }).unwrap();
                    match self.send_response(response) {
                        Ok(()) => { return Err(ApiError::DecodeError); }
                        Err(err) => { return Err(err); }
                    }
                }

                // Parse the request down to a concrete request.
                let msg_str = message.as_str().unwrap();

                // Ensure the message header can be decoded correctly.
                let req_opt: Result<ApiIpcRequest, serde_json::Error> =
                    serde_json::from_str(msg_str);
                if req_opt.is_err() {
                    let response = serde_json::to_string(&ApiResponse {
                        status: ApiStatus::InvalidRequestHead,
                        body: serde_json::to_string(&ApiIpcErrorResponseBody {
                            message: "The header of the message is invalid. 
                                Are you trying something funny?".to_string()
                        }).unwrap()
                    }).unwrap();
                    match self.send_response(response) {
                        Ok(()) => { return Err(ApiError::InvalidRequestHead); }
                        Err(err) => { return Err(err); }
                    }
                }
                let request = req_opt.unwrap();
                self.handle_request(&request)
            }
        }
    }

    fn handle_led_request(&mut self,
                          request: ApiIpcLedRequestBody) -> ApiResponse {
        // TODO: Emit to data channel
        debug!(target: "system.api.request",
               "Received LED request {:?}", request);
        ApiResponse {
            status: ApiStatus::Success,
            body: serde_json::to_string(
                &ApiIpcLedResponseBody {}).unwrap()
        }
    }

    fn handle_vel_request(&mut self,
                          request: ApiIpcVelRequestBody) -> ApiResponse {
        debug!(target: "system.api.request",
               "Received velocity request {:?}", request);
        ApiResponse {
            status: ApiStatus::Success,
            body: serde_json::to_string(
                &ApiIpcVelResponseBody {}).unwrap()
        }
    }

    /// This function is responsible for validating IPCRequestBody's.
    fn validate_body<'a, RequestBodyType>(
        &mut self,
        body_str: &'a str)
    -> Result<RequestBodyType, ApiError>
        where RequestBodyType: Deserialize<'a> + ValidatesApiIpcBody {

        // Parse the body, failing on error.
        let body_opt: Result<RequestBodyType, serde_json::Error> =
            serde_json::from_str(body_str);

        match body_opt {
            Ok(valid_body) => {
                // If the body is successfully parsed, we must
                // valiate it's arguments.
                if !valid_body.validate() {
                    let response = serde_json::to_string(&ApiResponse {
                        status: ApiStatus::InvalidRequestArgs,
                        body: serde_json::to_string(
                            &ApiIpcErrorResponseBody {
                                message: "The body params are invalid.
                                    Are you trying something funny?"
                                    .to_string()
                            }
                        ).unwrap()
                    }).unwrap();
                    match self.send_response(response) {
                        Ok(()) => {
                            return Err(ApiError::InvalidRequestBody);
                        }
                        Err(err) => { return Err(err); }
                    }
                }

                // If all is well, let us emit the body.
                Ok(valid_body)
            }
            Err(error) => {
                // If we did not successfully parse the body, let us notify the
                // API.
                let response = serde_json::to_string(&ApiResponse {
                    status: ApiStatus::InvalidRequestBody,
                    body: serde_json::to_string(
                        &ApiIpcErrorResponseBody {
                            message: "The body is invalid. Are you
                                trying something funny?".to_string()
                        }
                    ).unwrap()
                }).unwrap();
                match self.send_response(response) {
                    Ok(()) => {
                        return Err(ApiError::InvalidRequestBody);
                    }
                    Err(err) => { return Err(err); }
                }
            }
        }
    }

    fn handle_request(&mut self,
                      request: &ApiIpcRequest) -> Result<(), ApiError> {
        match request.request_type {
            ApiIpcRequestType::Led => {
                let body: Result<ApiIpcLedRequestBody, ApiError> =
                    self.validate_body(request.body.as_str());
                match body {
                    Ok(valid_body) => {
                        let response = self.handle_led_request(valid_body);
                        self.send_response(
                            serde_json::to_string(&response).unwrap())
                    }
                    Err(error) => { return Err(error); }
                }
            }
            ApiIpcRequestType::Vel => {
                let body: Result<ApiIpcVelRequestBody, ApiError> =
                    self.validate_body(request.body.as_str());
                match body {
                    Ok(valid_body) => {
                        let response = self.handle_vel_request(valid_body);
                        self.send_response(
                            serde_json::to_string(&response).unwrap())
                    }
                    Err(error) => { return Err(error); }
                }
            }
        }
    }

    fn send_response(&mut self, response: String) -> Result<(), ApiError> {
        match &self.socket {
            None => { Err(ApiError::SockNotReady) }
            Some(sock) => {
                match sock.send_str(response.as_str(), 0) {
                    Ok(()) => { Ok(()) }
                    Err(err) => { Err(ApiError::ZMQError(err)) }
                }
            }
        }
    }
}
