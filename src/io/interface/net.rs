/// Implementing this enables data to be broadcast over the wire between
/// coachbots.
trait BroadcastsData {
    /// Attempts to send bytes over the wire to all other devices via a
    /// broadcast transport (likely UDP to a broadcast address).
    fn send_bytes(data: Vec<u8>);
}

trait ListensForData {
}
