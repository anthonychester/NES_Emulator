//! Test suite for the Web and headless browsers.
extern crate wasm_nes_emulator;
use wasm_nes_emulator::cpu::CPU;

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod multi {
   use super::*;
 
    #[test]
   fn _5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.register_x, 0xc1)
   }
   
    #[test]
   fn load_and_read() {
      let mut cpu = CPU::new();
      cpu.load_and_run(vec![0xa9, 0x05, 0x85, 0x05, 0xa9, 0x00, 0xa5, 0x05]);
      
      assert_eq!(cpu.register_a, 0x05);
   } 
}


mod sta {
    use super::*;
    
    #[test]
    fn load_a() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9,0x05,0x85,0x05]);
        assert_eq!(cpu.mem_read(0x0005), 0x05);
    }
}
mod stx {
    use super::*;
    
    #[test]
    fn load_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2,0x05,0x86,0x05]);
        assert_eq!(cpu.mem_read(0x0005), 0x05);
    }
}
mod sty {
    use super::*;
    
    #[test]
    fn load_y() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0,0x05,0x84,0x05]);
        assert_eq!(cpu.mem_read(0x0005), 0x05);
    }
}
mod lda {
    use super::*;
    
    #[test]
   fn lda_from_memory() {
       let mut cpu = CPU::new();
       cpu.mem_write(0x10, 0x55);

       cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

       assert_eq!(cpu.register_a, 0x55);
   }
   
   #[test]
   fn lda_immidiate_load_data() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
       assert_eq!(cpu.register_a, 0x05);
       assert!(cpu.status & 0b0000_0010 == 0b00);
       assert!(cpu.status & 0b1000_0000 == 0);
   }

    #[test]
    fn lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
}
mod ldx {
    use super::*;
    
    #[test]
   fn ldx_from_memory() {
       let mut cpu = CPU::new();
       cpu.mem_write(0x10, 0x55);

       cpu.load_and_run(vec![0xa6, 0x10, 0x00]);

       assert_eq!(cpu.register_x, 0x55);
   }
   
   #[test]
   fn ldx_immidiate_load_data() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa2, 0x05, 0x00]);
       assert_eq!(cpu.register_x, 0x05);
       assert!(cpu.status & 0b0000_0010 == 0b00);
       assert!(cpu.status & 0b1000_0000 == 0);
   }

    #[test]
    fn ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
}
mod ldy {
    use super::*;
    
    #[test]
   fn ldy_from_memory() {
       let mut cpu = CPU::new();
       cpu.mem_write(0x10, 0x55);

       cpu.load_and_run(vec![0xa4, 0x10, 0x00]);

       assert_eq!(cpu.register_y, 0x55);
   }
   
   #[test]
   fn ldy_immidiate_load_data() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa0, 0x05, 0x00]);
       assert_eq!(cpu.register_y, 0x05);
       assert!(cpu.status & 0b0000_0010 == 0b00);
       assert!(cpu.status & 0b1000_0000 == 0);
   }

    #[test]
    fn ldx_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
}
mod txa {
    use super::*;
    
    #[test]
   fn txa_set_and_move_x_to_a() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0x05, 0xa8, 0xc8, 0x98, 0x00]);
 
       assert_eq!(cpu.register_a, 0x06)
   }
}
mod tax {
    use super::*;
    
    #[test]
   fn tax_set_and_move_a_to_x() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0x02, 0xaa, 0x00]);
 
       assert_eq!(cpu.register_x, 0x02)
   }
}
mod inx {
    use super::*;
    
      #[test]
   fn _6_inx_of_1() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0x01, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.register_x, 0x02)
   }

    #[test]
    fn inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0x01)
    }
}
mod dex {
    use super::*;

    #[test]
    fn _6_dex_of_1() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0x05, 0xaa, 0xca, 0x00]);
 
       assert_eq!(cpu.register_x, 0x04)
   }
   
    #[test]
    fn dex_underflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0xaa, 0xca, 0x00]);

        assert_eq!(cpu.register_x, 0xff)
    }
}
mod tya {
    use super::*;
    
    #[test]
   fn set_and_move_y_to_a() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0x05, 0xaa, 0xe8, 0x8a, 0x00]);
 
       assert_eq!(cpu.register_a, 0x06)
   }
}
mod tay {
    use super::*;
    
    #[test]
   fn set_and_move_a_to_y() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0x02, 0xa8, 0x00]);
 
       assert_eq!(cpu.register_y, 0x02)
   }
}
mod iny {
    use super::*;
    
      #[test]
   fn _6_iny_of_1() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0x01, 0xa8, 0xc8, 0x00]);
 
       assert_eq!(cpu.register_y, 0x02)
   }

    #[test]
    fn iny_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xa8, 0xc8, 0xc8, 0x00]);

        assert_eq!(cpu.register_y, 0x01)
    }
}
mod dey {
    use super::*;

    #[test]
    fn _6_dey_of_1() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0x05, 0xa8, 0x88, 0x00]);
 
       assert_eq!(cpu.register_y, 0x04)
   }
   
    #[test]
    fn dey_underflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0xa8, 0x88, 0x00]);

        assert_eq!(cpu.register_y, 0xff)
    }
}
mod nop {
    use super::*;
    
    #[test]
    fn no_op() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xEA, 0x00]);
        
        assert_eq!(cpu.program_counter, 0x8002);
    }
}
mod and {
    use super::*;
    
    #[test]
    fn and() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x29, 0x03, 0x00]);
        
        assert_eq!(cpu.register_a, 0x01);
    }
    
    #[test]
    fn and_with_mem() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x85, 0x03, 0xa9, 0xff, 0x25, 0x03, 0x00]);
        
        assert_eq!(cpu.register_a, 0x05);
    }
}
mod ora {
    use super::*;
    
    #[test]
    fn ora() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x09, 0x03, 0x00]);
        
        assert_eq!(cpu.register_a, 0x07);
    }
    
    #[test]
    fn ora_with_mem() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x85, 0x03, 0xa9, 0xff, 0x05, 0x03, 0x00]);
        
        assert_eq!(cpu.register_a, 0xff);
    }
}
mod ero {
    use super::*;
    
    #[test]
    fn ero() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x49, 0x03, 0x00]);
        
        assert_eq!(cpu.register_a, 0x06);
    }
    
    #[test]
    fn ero_with_mem() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x85, 0x03, 0xa9, 0xff, 0x49, 0x03, 0x00]);
        
        assert_eq!(cpu.register_a, 0xfc);
    }
}
mod adc {
    use super::*;
    
    #[test]
    fn adc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x69, 0x05, 0x00]);
        
        assert_eq!(cpu.register_a, 0x0a);
    }
    #[test]
    fn adc_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x69, 0x05, 0x00]);
        
        assert_eq!(cpu.status, 0b00000000);
    }
    #[test]
    fn adc_over() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0xff, 0x00]);
        
        assert_eq!(cpu.register_a, 0xfe);
    }
    #[test]
    fn adc_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x7f, 0x69, 0x01, 0x00]);
        
        assert_eq!(cpu.register_a, 0x80);
    }
    #[test]
    fn adc_overflow_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x7f, 0x69, 0x01, 0x00]);
        
        assert_eq!(cpu.status, 0b11000000);
    }
    #[test]
    fn adc_zero() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x69, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
    }
    #[test]
    fn adc_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x69, 0x00, 0x00]);
        
        assert_eq!(cpu.status, 0b00000010);
    }
    
}
mod sbc {
    use super::*;
    
    #[test]
    fn sbc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0xE9, 0x05, 0x00]);
        
        assert_eq!(cpu.register_a, 0xff);
    }
    #[test]
    fn sbc_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0xE9, 0x05, 0x00]);
        
        assert_eq!(cpu.status, 0b10000000);
    }
    #[test]
    fn sbc_over() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x02, 0xE9, 0xf5, 0x00]);
        
        assert_eq!(cpu.register_a, 0x0c);
    }
    #[test]
    fn sbc_zero() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0xE9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0xff);
    }
    #[test]
    fn sbc_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0xE9, 0x00, 0x00]);
        
        assert_eq!(cpu.status, 0b10000000);
    }
    
}
