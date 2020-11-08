//! The buffered display component offers a simple UTF-8 display system.
//! See [`BufferedDisplay`] for more details.

use mrvm::board::Bus;
use mrvm_tools::exceptions::AuxHwException;
use mrvm_tools::metadata::{DeviceMetadata, DisplayType};

// The character display works by sending to a display callback the character written in the only word of the display.
// It may be an invalid UTF-8 character, in which case the invalid word will be sent to the header instead of the decoded string.
pub struct CharDisplay {
    handler: Box<dyn FnMut(Result<char, u32>)>,
    hw_id: u64,
}

impl CharDisplay {
    /// Create a character display component.
    pub fn new(
        handler: Box<dyn FnMut(Result<char, u32>)>,
        hw_id: u64,
    ) -> Result<Self, &'static str> {
        Ok(Self { handler, hw_id })
    }
}

impl Bus for CharDisplay {
    fn name(&self) -> &'static str {
        "Character Display"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(self.hw_id, 4, DisplayType::Buffered.into(), None, None).encode()
    }

    fn read(&mut self, _addr: u32, ex: &mut u16) -> u32 {
        *ex = AuxHwException::MemoryNotReadable.into();
        0
    }

    fn write(&mut self, _addr: u32, word: u32, _ex: &mut u16) {
        (self.handler)(std::char::from_u32(word).ok_or(word))
    }

    fn reset(&mut self) {}
}
