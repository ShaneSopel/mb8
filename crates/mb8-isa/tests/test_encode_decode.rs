use mb8_isa::{decode::decode, encode::encode, opcodes::Opcode, registers::Register};

#[test]
fn test_round_trip() {
    {
        let opcode = Opcode::Nop {};
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Halt {};
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Mov {
            dst: Register::R0,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Add {
            dst: Register::R0,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Sub {
            dst: Register::R0,
            src: Register::R1,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Ldi {
            dst: Register::R0,
            value: 0x12,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Jmp { addr: 0x123 };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Jz { addr: 0x123 };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Jnz { addr: 0x123 };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Call { addr: 0x123 };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Ret {};
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::Ld {
            dst: Register::R0,
            addr: 0x12,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
    {
        let opcode = Opcode::St {
            src: Register::R0,
            addr: 0x12,
        };
        let bin = encode(&opcode);
        assert_eq!(decode(bin), Some(opcode));
    }
}
