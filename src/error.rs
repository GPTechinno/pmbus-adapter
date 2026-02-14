/// Errors that can occur during PMBus operations.
#[derive(Debug)]
pub enum PmbusError<E> {
    /// Underlying bus (I2C/SMBus) error.
    Bus(E),
    /// A value could not be encoded into the PMBus format.
    EncodingError,
    /// The device response had an unexpected length.
    InvalidResponseLength,
}

impl<E> From<E> for PmbusError<E> {
    fn from(e: E) -> Self {
        PmbusError::Bus(e)
    }
}
