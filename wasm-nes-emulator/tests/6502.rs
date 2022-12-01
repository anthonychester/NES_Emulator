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
mod bpl {
    use super::*;
    
    #[test]
    fn bpl_true() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x10, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x05);
    }
    
     #[test]
    fn bpl_false() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x85, 0x10, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
    }
}
mod bmi {
    use super::*;
    
    #[test]
    fn bmi_true() {
        let mut cpu = CPU::new();
        
        cpu.load_and_run(vec![0xa9, 0x85, 0x30, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x85);
    }
    
     #[test]
    fn bmi_false() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x30, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
    }
}
mod bvc {
    use super::*;
    
    #[test]
    fn bvc_true() {
        let mut cpu = CPU::new();
        
        cpu.load_and_run(vec![0xa9, 0x05, 0x69, 0x01, 0x50, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x06); //1 added in program
    }
    
     #[test]
    fn bvc_false() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x7f, 0x69, 0x01, 0x50, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
    }
}
mod bvs {
    use super::*;
    
    #[test]
    fn bvs_true() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x7f, 0x69, 0x01, 0x70, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x80); //1 added in program
        
    }
    
     #[test]
    fn bvs_false() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x69, 0x01, 0x70, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
        
    }
}
mod bcc {
    use super::*;
    
    #[test]
    fn bcc_true() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0x01, 0x90, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
        
    }
    
     #[test]
    fn bcc_false() {
        let mut cpu = CPU::new();
        
        cpu.load_and_run(vec![0xa9, 0x0f, 0x69, 0x01, 0x90, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x10); //1 added in program
    }
}
mod bcs {
    use super::*;
    
    #[test]
    fn bcs_true() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0x01, 0xB0, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
        
    }
    
     #[test]
    fn bcs_false() {
        let mut cpu = CPU::new();
        
        cpu.load_and_run(vec![0xa9, 0x0f, 0x69, 0x01, 0xB0, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00); //1 added in program
    }
}
mod bne {
    use super::*;
    
    #[test]
    fn bne_true() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0xD0, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x05);
        
    }
    
     #[test]
    fn bne_false() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xD0, 0x02, 0xa9, 0x01, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
    }
}
mod beq {
    use super::*;
    
    #[test]
    fn beq_true() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xF0, 0x02, 0xa9, 0x01, 0x00]);
        
        assert_eq!(cpu.register_a, 0x01);
        
    }
    
     #[test]
    fn beq_false() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0xF0, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
    }
}
mod inc {
    use super::*;
    
    #[test]
    fn inc_9() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x09, 0x85, 0x02, 0xe6, 0x02, 0xa5, 0x02, 0x00]);
        
        assert_eq!(cpu.register_a, 0x0a);
        
    }
    #[test]
    fn inc_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x85, 0x02, 0xe6, 0x02, 0xa5, 0x02, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
        
    }
}
mod flag_inst {
    use super::*;
    
    #[test]
    fn clc() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0001;
        cpu.load_and_run(vec![0x18, 0x00]);
        
        assert_eq!(cpu.status, 0b0000_0000);
        
    }
    
    #[test]
    fn sec() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0000;
        cpu.load_and_run(vec![0x38, 0x00]);
        
        assert_eq!(cpu.status, 0b0000_0001);
        
    }
    
    #[test]
    fn cli() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0100;
        cpu.load_and_run(vec![0x58, 0x00]);
        
        assert_eq!(cpu.status, 0b0000_0000);
        
    }
    
    #[test]
    fn sei() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0000;
        cpu.load_and_run(vec![0x78, 0x00]);
        
        assert_eq!(cpu.status, 0b0000_0100);
        
    }
    
    #[test]
    fn clv() {
        let mut cpu = CPU::new();
        cpu.status = 0b0100_0000;
        cpu.load_and_run(vec![0xB8, 0x00]);
        
        assert_eq!(cpu.status, 0b0000_0000);
        
    }
    
    #[test]
    fn cld() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_1000;
        cpu.load_and_run(vec![0xD8, 0x00]);
        
        assert_eq!(cpu.status, 0b0000_0000);
        
    }
    
    #[test]
    fn sed() {
        let mut cpu = CPU::new();
        cpu.status = 0b0000_0000;
        cpu.load_and_run(vec![0xF8, 0x00]);
        
        assert_eq!(cpu.status, 0b0000_1000);
        
    }
}
mod bit {
    use super::*;
    #[test]
    fn bit_C2_FF() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xC2 , 0x85, 0xC2 , 0xa9, 0xff, 0x24, 0xC2, 0x00]);
        
        assert_eq!(cpu.status, 0b1100_0000);
        
    }
    
    #[test]
    fn bit_FF_C2() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xF0 , 0x85, 0xC2 , 0xa9, 0x0F, 0x24, 0xC2, 0x00]);
        
        assert_eq!(cpu.status, 0b1100_0010);
        
    }
}
mod cmp {
    use super::*;
    #[test]
    fn cmp_eq() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05 , 0xc9, 0x05 , 0xd0, 0x02, 0xa9, 0x01, 0x00]);
        
        assert_eq!(cpu.register_a, 0x01);
        
    }
    #[test]
    fn cmp_gt_p() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0xc9, 0x03, 0xf0, 0x04, 0xb0, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x05);
        
    }
    
    #[test]
    fn cmp_gt_f() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0xc9, 0x09, 0xf0, 0x04, 0xb0, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
        
    }
    
    #[test]
    fn cmp_lt_p() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x03, 0xc9, 0x05, 0x90, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x03);
        
    }
    
    #[test]
    fn cmp_lt_f() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x09, 0xc9, 0x05, 0x90, 0x02, 0xa9, 0x00, 0x00]);
        
        assert_eq!(cpu.register_a, 0x00);
        
    }
}
mod cpx {
    use super::*;
    #[test]
    fn cpx_eq() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05 , 0xe0, 0x05 , 0xd0, 0x02, 0xa2, 0x01, 0x00]);
        
        assert_eq!(cpu.register_x, 0x01);
        
    }
    #[test]
    fn cpx_gt_p() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0xe0, 0x03, 0xf0, 0x04, 0xb0, 0x02, 0xa2, 0x00, 0x00]);
        
        assert_eq!(cpu.register_x, 0x05);
        
    }
    
    #[test]
    fn cpx_gt_f() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0xe0, 0x09, 0xf0, 0x04, 0xb0, 0x02, 0xa2, 0x00, 0x00]);
        
        assert_eq!(cpu.register_x, 0x00);
        
    }
    
    #[test]
    fn cpx_lt_p() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x03, 0xe0, 0x05, 0x90, 0x02, 0xa2, 0x00, 0x00]);
        
        assert_eq!(cpu.register_x, 0x03);
        
    }
    
    #[test]
    fn cpx_lt_f() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x09, 0xe0, 0x05, 0x90, 0x02, 0xa2, 0x00, 0x00]);
        
        assert_eq!(cpu.register_x, 0x00);
        
    }
}
mod cpy {
    use super::*;
    #[test]
    fn cpy_eq() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05 , 0xc0, 0x05 , 0xd0, 0x02, 0xa0, 0x01, 0x00]);
        
        assert_eq!(cpu.register_y, 0x01);
        
    }
    #[test]
    fn cpy_gt_p() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0xc0, 0x03, 0xf0, 0x04, 0xb0, 0x02, 0xa0, 0x00, 0x00]);
        
        assert_eq!(cpu.register_y, 0x05);
        
    }
    
    #[test]
    fn cpy_gt_f() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0xc0, 0x09, 0xf0, 0x04, 0xb0, 0x02, 0xa0, 0x00, 0x00]);
        
        assert_eq!(cpu.register_y, 0x00);
        
    }
    
    #[test]
    fn cpy_lt_p() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x03, 0xc0, 0x05, 0x90, 0x02, 0xa0, 0x00, 0x00]);
        
        assert_eq!(cpu.register_y, 0x03);
        
    }
    
    #[test]
    fn cpy_lt_f() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x09, 0xc0, 0x05, 0x90, 0x02, 0xa0, 0x00, 0x00]);
        
        assert_eq!(cpu.register_y, 0x00);
        
    }
    
    #[test]
    fn cpy_y_reg_full() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0xff, 0xc0, 0xff, 0xd0, 0x02, 0xa0, 0x00, 0x00]);
        
        assert_eq!(cpu.register_y, 0x00);
        
    }
}
mod dec {
    use super::*;
    
    #[test]
    fn dec_9() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x09, 0x85, 0x02, 0xc6, 0x02, 0xa5, 0x02, 0x00]);
        
        assert_eq!(cpu.register_a, 0x08);
        
    }
    #[test]
    fn dec_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x85, 0x02, 0xc6, 0x02, 0xa5, 0x02, 0x00]);
        
        assert_eq!(cpu.register_a, 0xff);
        
    }
}
mod asl {
    use super::*;

    #[test]
    fn asl_4() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x04, 0x0a, 0x00]);

        assert_eq!(cpu.register_a, 0x08)
    }

    #[test]
    fn asl_flags() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0xf0, 0x0a, 0x00]);

        assert_eq!(cpu.status, 0b10000001)
    }
    
    #[test]
    fn asl_zero() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x00, 0x0a, 0x00]);

        assert_eq!(cpu.status, 0b00000010)
    }

    #[test]
    fn asl_full() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x08, 0x85, 0x02, 0x06, 0x02, 0xa5, 0x02, 0x00]);

        assert_eq!(cpu.register_a, 0x10)
    }
}
mod jmp {
    use super::*;

    #[test]
    fn jmp_skip_some() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0x4c, 0x05, 0x80, 0xa9, 0x01, 0xa9, 0x02, 0x4c, 0x0c, 0x80, 0xa9, 0x00, 0x00]);

        assert_eq!(cpu.register_a, 0x02);
    }
}
mod txs {
    use super::*;
    #[test]
    fn txs() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa2, 0x05, 0x9a, 0x00]);

        assert_eq!(cpu.stack_ptr, 0x05);
    }
}
mod tsx {
    use super::*;

    #[test]
    fn txs() {
        let mut cpu = CPU::new();

        cpu.load(vec![0xba, 0x00]);
        cpu.reset();
        cpu.stack_ptr = 0x05;
        cpu.run();

        assert_eq!(cpu.register_x, 0x05);
    }
}
mod pha {
    use super::*;

    #[test]
    fn pha() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x05, 0x48, 0x48, 0x00]);


        assert_eq!(cpu.mem_read(0x01ff), 0x05);
        assert_eq!(cpu.mem_read(0x01fe), 0x05);
    }
}
mod pla {
    use super::*;

    #[test]
    fn pla() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x05, 0x48, 0xa9, 0x00, 0x68, 0x00]);

        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.stack_ptr, 0x00);
    }
}
mod php {
    use super::*;
    #[test]
    fn php() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x7f, 0x69, 0x01, 0x08, 0x00]);

        assert_eq!(cpu.mem_read(0x01ff), 0xc0);
    }
}
mod plp {
    use super::*;
    #[test]
    fn plp() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x7f, 0x69, 0x01, 0x08, 0xB8, 0x28, 0x00]);

        assert_eq!(cpu.status, 0xc0);
    }
}
mod jsr {
    use super::*;
    #[test]
    fn jsr() {
        let mut cpu = CPU::new();

       cpu.load(vec![0x20, 0x03, 0x80, 0x00]);
       cpu.reset();
       cpu.next(); //jsr
        assert_eq!(cpu.mem_read(0x1ff), 0x80);
        assert_eq!(cpu.mem_read(0x1fe), 0x02);
        assert_eq!(cpu.program_counter, 0x8003);
    }
}
mod rts {
    use super::*;
    #[test]
    fn rts() {
        let mut cpu = CPU::new();

       cpu.load(vec![0x20, 0x05, 0x80, 0xa2, 0x05, 0xa9, 0x05, 0x60, 0x00]);
       cpu.reset();
       cpu.next(); //JSR
       cpu.next(); //LDA
       cpu.next(); //RTS
       println!("pc: {}", cpu.mem_read(cpu.program_counter));
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.register_x == 0x05, false);
        assert_eq!(cpu.program_counter, 0x8003);
    }
}
