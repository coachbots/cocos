pub trait IsApiInstructionArg {
    fn validate(&self) -> bool;
}

pub enum ApiInstruction {
    SetLed,
    SetSpeed
}

pub struct ApiSetSpeedArgs {
    left: u8,
    right: u8
}

impl IsApiInstructionArg for ApiSetSpeedArgs {
    fn validate(&self) -> bool {
        self.left <= 100u8 && self.right <= 100u8
    }
}

pub struct ApiSetLedArgs {
    red: u8,
    green: u8,
    blue: u8
}

impl IsApiInstructionArg for ApiSetLedArgs {
    fn validate(&self) -> bool {
        self.red <= 100u8 && self.green <= 100u8 && self.blue <= 100u8
    }
}

pub struct ApiCommand {
    instruction: ApiInstruction,
    args: ApiSetLedArgs
}
