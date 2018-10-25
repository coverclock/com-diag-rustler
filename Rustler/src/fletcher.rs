/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

/// Implements the computationally simple Fletcher checksum algorithm.
///
/// REFERENCES
///
/// J. Zweig, C. Partridge, "TCP Alternate Checksum Options", RFC 1146,
/// https://tools.ietf.org/html/rfc1146, IETF, February 1990
///
/// "Fletcher's checksum", Wikipedia,
/// https://en.wikipedia.org/wiki/Fletcher's_checksum, 2016-12-21
///
/// J. Fletcher, "An Arithmetic Checksum for Serial Transmissions",
/// IEEE Transactions on Communication, COM-30, No. 1, pp. 247-252,
/// January 1982
///
pub mod fletcher {

    use std::string;
    use std::mem;
  
    pub struct Fletcher {
        a:          u8,
        b:          u8,
    }

    pub static FLETCHER: usize = mem::size_of::<Fletcher>();
    
    impl string::ToString for Fletcher {
        
        fn to_string(& self) -> String {
            let mut c: u16;

            c = self.b as u16;
            c <<= 8;
            c |= self.a as u16;

            format!("Fletcher@{:p}[{}]:{{a:0x{:02x},b:0x{:02x},c:0x{:04x}}}",
                self, FLETCHER,
                self.a, self.b, c)
        }

    }
    
    impl Fletcher {
        
        /// new returns a freshly minted zeroed-out Fletcher object.
        pub fn new() -> Fletcher {
            
            Fletcher {
                a:  0,
                b:  0,
            }
            
        }
        
        /// init initializes a Fletcher object with new eight-bit A and B values.
        pub fn init(& mut self, a: u8, b: u8) {
            self.a = a;
            self.b = b;
        }
        
        /// reset resets a Fletcher object back to its initial state.
        pub fn reset(& mut self) {
            self.a = 0;
            self.b = 0;
        }
        
        /// checksum16 computes a running sixteen-bit Fletcher checksum based on
        /// a slice of a byte buffer and the two eight-bit running checksum variables.
        /// The current sixteen-bit checksum is returned by concatenating the two
        /// eight-bit running values.
        pub fn checksum(& mut self, buffer: & [u8]) -> u16 {
            let mut c: u16;
            let mut a: u16 = self.a as u16;
            let mut b: u16 = self.b as u16;
            
            for d in buffer {
                a = (a + (*d as u16)) % 255;
                b = (b + a) % 255;
            }
            
            self.a = a as u8;
            self.b = b as u8;

            c = b;
            c <<= 8;
            c |= a;
            
            return c;
        }
        
    }
    
}
