
/// Strongly-typed hardware exception
pub enum AuxHwException {
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
    pub fn decode(code: u16) -> Result<Self, ()> {
        let code = (code >> 8) as u8;
        let data = (code & 0xFF) as u8;

        match code {
            0x10 => Ok(Self::UnknownOperation(data)),
            0x11 => Ok(Self::UnsupportedOperation),
            
            0x20 => Ok(Self::GenericPhysicalReadError),
            0x21 => Ok(Self::MemoryNotReadable),

            0x30 => Ok(Self::GenericPhysicalWriteError),
            0x31 => Ok(Self::MemoryNotWritable),

            _ => Err(())
        }
    }

    /// Get the exception's code
    pub fn code(&self) -> u8 {
        match self {
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
        (self.code() as u16) << 8 + self.associated_data().unwrap_or(0) as u16
    }
}

impl Into<u16> for AuxHwException {
    fn into(self) -> u16 {
        self.encode()
    }
}