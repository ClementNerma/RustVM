use std::fmt;

/// Strongly-typed hardware exception
#[derive(Debug)]
pub enum AuxHwException {
    /// Unknown error
    UnknownError,

    /// Unspecified synchronization error
    UnspecifiedSyncError,

    /// Time synchronization error
    TimeSynchronizationError,

    /// An unknown operation was requested.
    /// This can be for instance an invalid code sent to the last addressed word of a buffered display.
    UnknownOperation(u8),

    /// An unsupported operation was requested.
    UnsupportedOperation,

    /// A physical read error occurred.
    /// If none other exception code matches the type of error you want to raise, use this one as a fallback.
    GenericPhysicalReadError,

    /// Tried to read a non-readable address of the component.
    MemoryNotReadable,

    /// A physical write error occurred.
    /// If none other exception code matches the type of error you want to raise, use this one as a fallback.
    GenericPhysicalWriteError,

    /// Tried to write a non-writable address of the component.
    MemoryNotWritable,
}

impl AuxHwException {
    /// Decode an auxiliary component's exception
    pub fn decode(ex: u16) -> Result<Self, ()> {
        let code = (ex >> 8) as u8;
        let data = ex as u8;

        Self::decode_parts(code, Some(data))
    }

    /// Decode an auxiliary component's exception from its code and data
    pub fn decode_parts(code: u8, data: Option<u8>) -> Result<Self, ()> {
        let data_or_err = data.ok_or(());

        match code {
            0x00 => Ok(Self::UnknownError),
            0x01 => Ok(Self::UnspecifiedSyncError),
            0x02 => Ok(Self::TimeSynchronizationError),

            0x10 => Ok(Self::UnknownOperation(data_or_err?)),
            0x11 => Ok(Self::UnsupportedOperation),

            0x20 => Ok(Self::GenericPhysicalReadError),
            0x21 => Ok(Self::MemoryNotReadable),

            0x30 => Ok(Self::GenericPhysicalWriteError),
            0x31 => Ok(Self::MemoryNotWritable),

            _ => Err(()),
        }
    }

    /// Get the exception's code
    pub fn code(&self) -> u8 {
        match self {
            Self::UnknownError => 0x00,
            Self::UnspecifiedSyncError => 0x01,
            Self::TimeSynchronizationError => 0x02,

            Self::UnknownOperation(_) => 0x10,
            Self::UnsupportedOperation => 0x11,

            Self::GenericPhysicalReadError => 0x20,
            Self::MemoryNotReadable => 0x21,

            Self::GenericPhysicalWriteError => 0x30,
            Self::MemoryNotWritable => 0x31,
        }
    }

    /// Get the exception's eventual associated data
    pub fn associated_data(&self) -> Option<u8> {
        match self {
            Self::UnknownError => None,
            Self::UnspecifiedSyncError => None,
            Self::TimeSynchronizationError => None,

            Self::UnknownOperation(op) => Some(*op),
            Self::UnsupportedOperation => None,

            Self::GenericPhysicalReadError => None,
            Self::MemoryNotReadable => None,

            Self::GenericPhysicalWriteError => None,
            Self::MemoryNotWritable => None,
        }
    }

    // Encode the exception with its eventual associated data
    pub fn encode(&self) -> u16 {
        ((self.code() as u16) << 8) + self.associated_data().unwrap_or(0) as u16
    }
}

impl From<AuxHwException> for u16 {
    fn from(ex: AuxHwException) -> u16 {
        ex.encode()
    }
}

impl fmt::Display for AuxHwException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnknownError => "Unknown error".to_string(),
                Self::UnspecifiedSyncError => "Unspecified synchronization error".to_string(),
                Self::TimeSynchronizationError => "Time synchronization error".to_string(),

                Self::UnknownOperation(op) => format!("Unknown operation {:#004X}", op),
                Self::UnsupportedOperation => "Unsupported operation".to_string(),

                Self::GenericPhysicalReadError => "Generic physical read error".to_string(),
                Self::MemoryNotReadable => "This memory address is not readable".to_string(),

                Self::GenericPhysicalWriteError => "Generic physical write error".to_string(),
                Self::MemoryNotWritable => "This memory address is not writable".to_string(),
            }
        )
    }
}
