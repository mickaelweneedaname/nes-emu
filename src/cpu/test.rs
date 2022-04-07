use super::*;
use crate::traits::Memory;

struct MemoryMock {
    pub memory: [u8; 0xFFFF],
}

impl MemoryMock {
    pub fn new(program: &[u8], origin: u16) -> Self {
        let mut mock = Self {
            memory: [0x00; 0xFFFF],
        };
        unsafe {
            mock.load(program, origin);
            mock.mem_write_u16(0xFFFC, origin);
        }
        mock
    }
}

impl Memory for MemoryMock {
    unsafe fn load(&mut self, data: &[u8], dest: u16) {
        self.memory[usize::from(dest)..usize::from(dest) + data.len()].copy_from_slice(data);
    }

    unsafe fn mem_read_u8(&mut self, addr: u16) -> u8 {
        self.memory[usize::from(addr)]
    }

    unsafe fn mem_write_u8(&mut self, addr: u16, byte: u8) {
        self.memory[usize::from(addr)] = byte
    }
}

fn create_mock_from_script(script: &str) -> MemoryMock {
    let program = asm_6502::compile(script.to_string(), 0x8000);
    println!("{:?}", program);
    MemoryMock::new(&program, 0x8000)
}

fn create_branch_forward_test_scipt(command: &str, value: u8) -> String {
    format!(
        r#"{command} forward
    LDA #{value}
    STA $55
    {command} end
forward:
    LDA #33
    STA $55
end:
    BRK"#
    )
}

fn create_branch_backward_test_scipt(command: &str) -> String {
    format!(
        r#"{command} forward
backward:
    LDA #42
    STA $42
    {command} end
forward:
    {command} backward
    LDA #99
    STA $42
end:
    BRK"#
    )
}

#[test]
fn lda_immediate_and_sta_zero_page() {
    let mut mock = create_mock_from_script(
        r#"LDA #10 ; load 10 into register A
           STA $5  ; store register A at address 0x00 + 0x05 = 0x05
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(10, mock.memory[0x0005])
}

#[test]
fn lda_absolute_and_sta_absolute() {
    let mut mock = create_mock_from_script(
        r#"LDA $1234 ; load A at $1234
           STA $4321 ; store register A at 4321
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x1234, 42);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x4321])
}

#[test]
fn ldx_immediate_and_stx_zero_page() {
    let mut mock = create_mock_from_script(
        r#"LDX #25 ; load 25 into register X
           STX $9  ; store register X at address 0x0 + 0x9 = 0x09
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(25, mock.memory[0x0009])
}

#[test]
fn ldx_absolute_and_stx_absolute() {
    let mut mock = create_mock_from_script(
        r#"LDX $2341 ; load X at $2341
           STX $3214 ; store register X at 3214
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x2341, 99);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(99, mock.memory[0x3214])
}

#[test]
fn ldy_immediate_and_sty_zero_page() {
    let mut mock = create_mock_from_script(
        r#"LDY #33 ; load 33 into register Y
           STY $2A  ; store register A at address 0x00 + 0X2A = 0x2A
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    };
    assert_eq!(33, mock.memory[0x002A])
}

#[test]
fn ldy_absolute_and_sty_absolute() {
    let mut mock = create_mock_from_script(
        r#"LDY $3412 ; load T at $3412
           STY $2143 ; store register X at $2143
           BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x3412, 89);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(89, mock.memory[0x2143])
}

#[test]
fn bcc_forward() {
    let script = create_branch_forward_test_scipt("BCC", 10);
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(33, mock.memory[0x55])
}

#[test]
fn bcc_backward() {
    let script = create_branch_backward_test_scipt("BCC");
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn bpl_forward() {
    let script = create_branch_forward_test_scipt("BPL", 10);
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(33, mock.memory[0x55])
}

#[test]
fn bpl_backward() {
    let script = create_branch_backward_test_scipt("BPL");
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn bvc_forward() {
    let script = create_branch_forward_test_scipt("BVC", 10);
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(33, mock.memory[0x55])
}

#[test]
fn bvc_backward() {
    let script = create_branch_backward_test_scipt("BVC");
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn bne_forward() {
    let script = create_branch_forward_test_scipt("BNE", 10);
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(33, mock.memory[0x55])
}

#[test]
fn bne_backward() {
    let script = create_branch_backward_test_scipt("BNE");
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn beq_forward() {
    let mut mock = create_mock_from_script(
        r#" LDA #0
    BEQ forward
    LDY #99
    STY $42
forward:
    LDY #42
    STY $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn beq_backward() {
    let mut mock = create_mock_from_script(
        r#"BCC forward
backward:
    LDX #42
    STX $42
    LDX #0
    BEQ end
forward:
    LDX #99
    STX $42
    LDA #0
    BEQ backward
end:
    BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn bmi_forward() {
    let script = create_branch_forward_test_scipt("BMI", 10u8.wrapping_neg());
    let mut mock = create_mock_from_script(&script);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(10u8.wrapping_neg(), mock.memory[0x55])
}

#[test]
fn adc_without_carry() {
    let mut mock = create_mock_from_script(
        r#"LDA #30
    ADC #12
    STA $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn adc_signed_without_carry() {
    let mut mock = create_mock_from_script(
        format!(
            r#"LDA #30
    ADC #{}
    STA $42"#,
            40u8.wrapping_neg()
        )
        .as_ref(),
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(10u8.wrapping_neg(), mock.memory[0x42])
}

#[test]
fn adc_with_carry() {
    let mut mock = create_mock_from_script(
        r#"LDA #255
    ADC #2
    BCS end
    STA $42
end:
    LDA #42
    STA $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn adc_with_overflow() {
    let mut mock = create_mock_from_script(
        r#"LDA #80
    ADC #80
    BVS end
    STA $42
end:
    LDA #42
    STA $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_and_1() {
    let mut mock = create_mock_from_script(
        r#"LDA #%11110000
    AND #%00001111
    STA $00"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b0000_0000, mock.memory[0x00])
}

#[test]
fn test_and_2() {
    let mut mock = create_mock_from_script(
        r#"LDA #%01010101
    AND #%10011001
    STA $00"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b0001_0001, mock.memory[0x00])
}

#[test]
fn test_sec() {
    let mut mock = create_mock_from_script(
        r#"SEC
    BCS good
    LDX #99
    STX $1ABC
    BEQ end
good:
    LDX #42
    STX $1ABC
end:
    BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x1ABC])
}

#[test]
fn test_clc() {
    let mut mock = create_mock_from_script(
        r#"SEC
    CLC
    BCC good
    LDX #99
    STX $1ABC
    BEQ end
good:
    LDX #42
    STX $1ABC
end:
    BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x1ABC])
}

#[test]
fn test_clv() {
    let mut mock = create_mock_from_script(
        r#"LDA #80
    ADC #80
    CLV
    BVC good
    LDX #99
    STX $1ABC
    BEQ end
good:
    LDX #42
    STX $1ABC
end:
    BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x1ABC])
}

#[test]
fn test_asl_a_and_bcs() {
    let mut mock = create_mock_from_script(
        r#"LDA $05
    ASL A
    STA $10
    BCS end
    LDX #99
    STX $0042
end:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x05, 0b1010_1010);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b0101_0100, mock.memory[0x10]);
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_asl_and_bcs() {
    let mut mock = create_mock_from_script(
        r#"ASL $05
    LDA $05
    STA $10
    BCS end
    LDX #99
    STX $0042
end:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x05, 0b1010_1010);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b0101_0100, mock.memory[0x10]);
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_cmp_bmi() {
    let mut mock = create_mock_from_script(
        r#"LDA #10
    CMP $00
    BMI inferior
    LDX #99
    STX $0042
inferior:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 15);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_cmp_bpl() {
    let mut mock = create_mock_from_script(
        r#"LDA #20
    CMP $00
    BPL superior
    LDX #99
    STX $0042
superior:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 15);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_cmp_beq() {
    let mut mock = create_mock_from_script(
        r#"LDA #15
    CMP $00
    BEQ equal
    LDX #99
    STX $0042
equal:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 15);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_cpx_bmi() {
    let mut mock = create_mock_from_script(
        r#"LDX #10
    CPX $00
    BMI inferior
    LDX #99
    STX $0042
inferior:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 15);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_cpx_bpl() {
    let mut mock = create_mock_from_script(
        r#"LDX #20
    CPX $00
    BPL superior
    LDX #99
    STX $0042
superior:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 15);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_cpx_beq() {
    let mut mock = create_mock_from_script(
        r#"LDX #15
    CPX $00
    BEQ equal
    LDX #99
    STX $0042
equal:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 15);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_cpy_bmi() {
    let mut mock = create_mock_from_script(
        r#"LDY #10
    CPY $00
    BMI inferior
    LDX #99
    STX $0042
inferior:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 15);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_cpy_bpl() {
    let mut mock = create_mock_from_script(
        r#"LDY #20
    CPY $00
    BPL superior
    LDX #99
    STX $0042
superior:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 15);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_cpy_beq() {
    let mut mock = create_mock_from_script(
        r#"LDY #15
    CPY $00
    BEQ equal
    LDX #99
    STX $0042
equal:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 15);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_dec_beq() {
    let mut mock = create_mock_from_script(
        r#"DEC $00
    LDA $00
    CMP #9
    BEQ equal
    LDX #99
    STX $0042
equal:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 10);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_dex_beq() {
    let mut mock = create_mock_from_script(
        r#"LDX $00
    DEX
    CPX #9
    BEQ equal
    LDX #99
    STX $0042
equal:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 10);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_dey_beq() {
    let mut mock = create_mock_from_script(
        r#"LDY $00
    DEY
    CPY #9
    BEQ equal
    LDX #99
    STX $0042
equal:
    LDX #42
    STX $0042"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x00, 10);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_eor() {
    let mut mock = create_mock_from_script(
        r#"LDA $12
    EOR #%10101010
    STA $12"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x12, 0b11001100);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b01100110, mock.memory[0x12])
}

#[test]
fn test_inc() {
    let mut mock = create_mock_from_script(r#"INC $12"#);
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x12, 9);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(10, mock.memory[0x12])
}

#[test]
fn test_inx() {
    let mut mock = create_mock_from_script(
        r#"LDX $12
    INX
    STX $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x12, 9);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(10, mock.memory[0x42])
}

#[test]
fn test_iny() {
    let mut mock = create_mock_from_script(
        r#"LDY $12
    INY
    STY $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0x12, 9);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(10, mock.memory[0x42])
}

#[test]
fn test_lsr_a_and_bcs() {
    let mut mock = create_mock_from_script(
        r#"LDA $AB
    LSR A
    STA $AB
    BCS carryset
    LDX #99
    STX $42
carryset:
    LDX #42
    STX $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0xAB, 0b1010_1011);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b0101_0101, mock.memory[0xAB]);
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_lsr_and_bcs() {
    let mut mock = create_mock_from_script(
        r#"LSR $AB
    BCS carryset
    LDX #99
    STX $42
carryset:
    LDX #42
    STX $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0xAB, 0b1010_1011);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b0101_0101, mock.memory[0xAB]);
    assert_eq!(42, mock.memory[0x42])
}

#[test]
fn test_ora() {
    let mut mock = create_mock_from_script(
        r#"LDA $AB
    ORA #%00001111
    STA $BA"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0xAB, 0b1111_0000);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b1111_1111, mock.memory[0xBA]);
}

#[test]
fn test_pha() {
    let mut mock = create_mock_from_script(
        r#"LDA #42
    PHA"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x01FF]);
}

#[test]
fn test_pla() {
    let mut mock = create_mock_from_script(
        r#"LDA #42
    PHA
    LDA #0
    PLA
    STA $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42]);
}

#[test]
fn test_rol_a() {
    let mut mock = create_mock_from_script(
        r#"SEC
    LDA $AB
    ROL A
    STA $42
    BCS end
    LDA #99
    STA $AB
end:
    LDA #42
    STA $AB"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0xAB, 0b1010_1010);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b0101_0101, mock.memory[0x42]);
    assert_eq!(42, mock.memory[0xAB]);
}

#[test]
fn test_rol() {
    let mut mock = create_mock_from_script(
        r#"SEC
    ROL $AB
    BCS end
    LDA #99
    STA $42
end:
    LDA #42
    STA $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0xAB, 0b1010_1010);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b0101_0101, mock.memory[0xAB]);
    assert_eq!(42, mock.memory[0x42]);
}

#[test]
fn test_ror_a() {
    let mut mock = create_mock_from_script(
        r#"SEC
    LDA $AB
    ROR A
    STA $42
    BCC end
    LDA #99
    STA $AB
    BRK
end:
    LDA #42
    STA $AB"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0xAB, 0b1010_1010);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b1101_0101, mock.memory[0x42]);
    assert_eq!(42, mock.memory[0xAB]);
}

#[test]
fn test_ror() {
    let mut mock = create_mock_from_script(
        r#"SEC
    ROR $AB
    BCC end
    LDA #99
    STA $42
    BRK
end:
    LDA #42
    STA $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        mock.mem_write_u8(0xAB, 0b1010_1010);
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0b1101_0101, mock.memory[0xAB]);
    assert_eq!(42, mock.memory[0x42]);
}

#[test]
fn test_tax() {
    let mut mock = create_mock_from_script(
        r#"LDA #42
    TAX
    STX $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42]);
}

#[test]
fn test_txa() {
    let mut mock = create_mock_from_script(
        r#"LDX #42
    TXA
    STA $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42]);
}

#[test]
fn test_tay() {
    let mut mock = create_mock_from_script(
        r#"LDA #42
    TAY
    STY $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42]);
}

#[test]
fn test_tya() {
    let mut mock = create_mock_from_script(
        r#"LDY #42
    TYA
    STA $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42]);
}

#[test]
fn test_txs() {
    let mut mock = create_mock_from_script(
        r#"LDA #1
    PHA
    LDA #2
    PHA
    LDX $FF
    TXS 
    LDA #10
    PHA
    LDA #30
    PHA
    PLA
    TAX
    STX $42
    PLA
    STA $43"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(30, mock.memory[0x42]);
    assert_eq!(10, mock.memory[0x43]);
}

#[test]
fn test_tsx() {
    let mut mock = create_mock_from_script(
        r#"PHA
    TSX
    STX $42"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(0xFE, mock.memory[0x42]);
}

#[test]
fn test_jsr_jmp_rts() {
    let mut mock = create_mock_from_script(
        r#"JSR func
    LDA #42
    STA $42
    jmp end
    LDA #99
    STA $42
func:
    LDX #42
    STX $4242
    RTS
end:
    BRK"#,
    );
    let mut cpu = Cpu::new(&mut mock);
    unsafe {
        cpu.reset();
        cpu.run_loop()
    }
    assert_eq!(42, mock.memory[0x42]);
    assert_eq!(42, mock.memory[0x4242]);
}
