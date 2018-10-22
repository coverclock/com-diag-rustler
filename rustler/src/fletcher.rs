/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

pub mod fletcher {

    use std::string;
  
    pub struct Fletcher {
        a:          u8,
        b:          u8,
    }
    
    impl string::ToString for Fletcher {
        
        fn to_string(& self) -> String {
            let mut c: u16;

            c = self.b as u16;
            c <<= 8;
            c |= self.a as u16;

            format!("Fletcher@{:p}(a=0x{:02x},b=0x{:02x},c=0x{:04x})",
                self, self.a, self.b, c)
        }

    }
    
    impl Fletcher {
        
        pub fn new() -> Fletcher {
            
            Fletcher {
                a:  0,
                b:  0,
            }
            
        }
        
        pub fn init(& mut self) {
            self.a = 0;
            self.b = 0;
        }
        
        pub fn reset(& mut self) {
            self.init();
        }
        
        pub fn checksum(& mut self, buffer: & [u8]) -> u16 {
            let mut c: u16;
            let mut a: u16 = self.a as u16;
            let mut b: u16 = self.b as u16;
            
            for d in buffer {
                a = (a + (d as u16)) % 255;
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
