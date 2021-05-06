use std::{env, fs::File, process};
use std::io::prelude::*;

/// Check command line arguments
fn get_filename() -> String {
    let argv = env::args().collect::<Vec<String>>();
    match argv.len() {
        0 => {
            println!("Unknown Error.");
            process::exit(-1)
        }
        1 => {
            println!("Usage:");
            println!("    {:#?} <evm-asm file>", argv[0]);
            process::exit(0)
        }
        _ => argv[1].to_owned(),
    }
}

#[derive(Debug,Clone)]
struct Instruction {
    address: usize,
    label: Option<String>,
    opcode: u8,
    oprand: Option<Vec<u8>>,
    raw_param: Option<String>,
    ins_len: usize,
}

impl Instruction {
    fn new_ins_from_raw_str(raw: Vec<&str>) -> Self {
        if raw.len() == 1 && raw[0].ends_with(":") {   // is label
            Self {
                address: 0,
                label: Some(raw[0][..raw[0].len()-1].to_owned()),
                opcode: 0,
                oprand: None,
                raw_param: None,
                ins_len: 0,
            }
        } else {
            Self {
                address: 0,
                label: None,
                opcode: Self::mnemonic_to_opcode(raw[0]),
                oprand: None,
                raw_param: if raw.len() == 1 {
                    None
                } else {
                    Some(raw[1].to_owned())
                },
                ins_len: 1,
            }
        }
    }

    fn mnemonic_to_opcode(mnemonic: &str) -> u8 {
        match mnemonic.to_uppercase().as_str() {
            "STOP" => 0x00,
            "ADD" => 0x01,
            "MUL" => 0x02,
            "SUB" => 0x03,
            "DIV" => 0x04,
            "SDIV" => 0x05,
            "MOD" => 0x06,
            "SMOD" => 0x07,
            "ADDMOD" => 0x08,
            "MULMOD" => 0x09,
            "EXP" => 0x0a,
            "SIGNEXTEND" => 0x0b,
            "LT" => 0x10,
            "GT" => 0x11,
            "SLT" => 0x12,
            "SGT" => 0x13,
            "EQ" => 0x14,
            "ISZERO" => 0x15,
            "AND" => 0x16,
            "OR" => 0x17,
            "XOR" => 0x18,
            "NOT" => 0x19,
            "BYTE" => 0x1a,
            "SHL" => 0x1b,
            "SHR" => 0x1c,
            "SAR" => 0x1d,
            "SHA3" => 0x20,
            "ADDRESS" => 0x30,
            "BALANCE" => 0x31,
            "ORIGIN" => 0x32,
            "CALLER" => 0x33,
            "CALLVALUE" => 0x34,
            "CALLDATALOAD" => 0x35,
            "CALLDATASIZE" => 0x36,
            "CALLDATACOPY" => 0x37,
            "CODESIZE" => 0x38,
            "CODECOPY" => 0x39,
            "GASPRICE" => 0x3a,
            "EXTCODESIZE" => 0x3b,
            "EXTCODECOPY" => 0x3c,
            "RETURNDATASIZE" => 0x3d,
            "RETURNDATACOPY" => 0x3e,
            "EXTCODEHASH" => 0x3f,
            "BLOCKHASH" => 0x40,
            "COINBASE" => 0x41,
            "TIMESTAMP" => 0x42,
            "NUMBER" => 0x43,
            "DIFFICULTY" => 0x44,
            "GASLIMIT" => 0x45,
            "CHAINID" => 0x46,
            "SELFBALANCE" => 0x47,
            "POP" => 0x50,
            "MLOAD" => 0x51,
            "MSTORE" => 0x52,
            "MSTORE8" => 0x53,
            "SLOAD" => 0x54,
            "SSTORE" => 0x55,
            "JUMP" => 0x56,
            "JUMPI" => 0x57,
            "PC" => 0x58,
            "MSIZE" => 0x59,
            "GAS" => 0x5a,
            "JUMPDEST" => 0x5b,
            "PUSH1" => 0x60,
            "PUSH2" => 0x61,
            "PUSH3" => 0x62,
            "PUSH4" => 0x63,
            "PUSH5" => 0x64,
            "PUSH6" => 0x65,
            "PUSH7" => 0x66,
            "PUSH8" => 0x67,
            "PUSH9" => 0x68,
            "PUSH10" => 0x69,
            "PUSH11" => 0x6a,
            "PUSH12" => 0x6b,
            "PUSH13" => 0x6c,
            "PUSH14" => 0x6d,
            "PUSH15" => 0x6e,
            "PUSH16" => 0x6f,
            "PUSH17" => 0x70,
            "PUSH18" => 0x71,
            "PUSH19" => 0x72,
            "PUSH20" => 0x73,
            "PUSH21" => 0x74,
            "PUSH22" => 0x75,
            "PUSH23" => 0x76,
            "PUSH24" => 0x77,
            "PUSH25" => 0x78,
            "PUSH26" => 0x79,
            "PUSH27" => 0x7a,
            "PUSH28" => 0x7b,
            "PUSH29" => 0x7c,
            "PUSH30" => 0x7d,
            "PUSH31" => 0x7e,
            "PUSH32" => 0x7f,
            "DUP1" => 0x80,
            "DUP2" => 0x81,
            "DUP3" => 0x82,
            "DUP4" => 0x83,
            "DUP5" => 0x84,
            "DUP6" => 0x85,
            "DUP7" => 0x86,
            "DUP8" => 0x87,
            "DUP9" => 0x88,
            "DUP10" => 0x89,
            "DUP11" => 0x8a,
            "DUP12" => 0x8b,
            "DUP13" => 0x8c,
            "DUP14" => 0x8d,
            "DUP15" => 0x8e,
            "DUP16" => 0x8f,
            "SWAP1" => 0x90,
            "SWAP2" => 0x91,
            "SWAP3" => 0x92,
            "SWAP4" => 0x93,
            "SWAP5" => 0x94,
            "SWAP6" => 0x95,
            "SWAP7" => 0x96,
            "SWAP8" => 0x97,
            "SWAP9" => 0x98,
            "SWAP10" => 0x99,
            "SWAP11" => 0x9a,
            "SWAP12" => 0x9b,
            "SWAP13" => 0x9c,
            "SWAP14" => 0x9d,
            "SWAP15" => 0x9e,
            "SWAP16" => 0x9f,
            "LOG0" => 0xa0,
            "LOG1" => 0xa1,
            "LOG2" => 0xa2,
            "LOG3" => 0xa3,
            "LOG4" => 0xa4,
            "PUSH" => 0xb0, // used for parsing jumpi relocations
            "CREATE" => 0xf0,
            "CALL" => 0xf1,
            "CALLCODE" => 0xf2,
            "RETURN" => 0xf3,
            "DELEGATECALL" => 0xf4,
            "CREATE2" => 0xf5,
            "STATICCALL" => 0xfa,
            "REVERT " => 0xfd,
            "SELFDESTRUCT" => 0xff,
            _ => {
                println!("Unknown mnemonic : {:?}", mnemonic);
                process::exit(-1)
            }
        }
    }

    fn from_file(filename: String) -> (Vec<Self>, Vec<(String,usize)>) {
        let mut file = File::open(filename).expect("open()");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("read_to_string()");
        let lines: Vec<&str> = contents.split("\n").collect();
        let mut raw_contents = Vec::<Vec<&str>>::new();

        // iterate over each line
        for i in 0..lines.len() {
            // first strip comments: comments start with `;` or `/`(`//`) character
            let payload = lines[i]
                .split(|c| c == ';' || c == '/')
                .collect::<Vec<&str>>();

            if payload.len() > 0 {
                // if contains comment
                raw_contents.push(payload[0].split_ascii_whitespace().collect::<Vec<&str>>());
            } else {
                // no comments in this line
                raw_contents.push(payload);
            }
        }

        // filter empty lines
        raw_contents = raw_contents.into_iter().filter(|x| x.len() > 0).collect();

        println!("raw_contents = {:#?}", raw_contents);

        // build return Vec<Instruction>
        let mut res: Vec<Instruction> = vec![];
        let mut labels = vec![];
        for i in raw_contents {
            // label treat as an instruction
            if i.len() == 1 && i[0].ends_with(":") {
                labels.push((i[0][..i[0].len()-1].to_owned(),0))
            } else {
                res.push(Self::new_ins_from_raw_str(i))
            }
        }
        (res, labels)
    }
}

const PUSH_START: u8 = 0x60;
const PUSH_END: u8 = 0x7f;
const PUSH: u8 = 0xb0;

fn main() {
    let filename = get_filename();
    let (mut ins_arr, mut labels) = Instruction::from_file(filename);
    
    // if the jump instruction is not of fixed size, label relocation is more complex
    // so i just try the smallest first, if not enough, then increase later.
    let mut current_addr = 0;
    for mut ins in &mut ins_arr {
        if ins.opcode >= PUSH_START && ins.opcode <= PUSH_END {
            // adjust the correct ins len
            ins.ins_len = (2 + ins.opcode - PUSH_START) as usize
        } else if ins.opcode == PUSH {
            ins.ins_len = 2
        }

        // TODO: adjust the label offset , maybe 1+ bytes
        ins.address = current_addr;
        current_addr += ins.ins_len;

        // keep down label's address
        if let Some(s) = ins.label.as_deref() {
            for i in 0..labels.len() {
                if labels[i].0 == s {
                    labels[i].1 = ins.address;
                }
            }
        }
    }

    // parse target address of push into binary
    for mut ins in &mut ins_arr {
        if ins.opcode == PUSH {
            for (k, v) in &labels {
                if k.eq(ins.raw_param.as_deref().unwrap()) {
                    ins.oprand = Some(vec![*v as u8]);
                }
            }
        }
    }
    
    // parse args of pushx into binary
    for mut ins in &mut ins_arr {
        if ins.opcode >= PUSH_START && ins.opcode <= PUSH_END {
            let param = ins.raw_param.as_deref().unwrap();
            println!("param = {:?}", param);

            // TODO: now the code assumes well-formatted number
            
            let mut target = vec![];
            for i in 0..ins.opcode-PUSH_START+1 {
                let sum;
                if param.starts_with("0x") {
                    sum = usize::from_str_radix(&param[2+(i as usize*2)..4+(i as usize*2)],16)
                    .expect("err oprand");
                } else {
                    sum = usize::from_str_radix(param,10).expect("err oprand");
                }
                target.insert(0, sum as u8);
            }
            ins.oprand = Some(target);
        }
    }


    let mut final_arr = vec![];
    for ins in &ins_arr {
        final_arr.push(ins.opcode);
        if let Some(v) = ins.oprand.as_ref() {
            for e in v {
                final_arr.push(*e)
            }
        }
    }

    println!("final_arr = {:?}", final_arr);
    
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn test_is_valid_ip() {
        
        assert_eq!(usize::from_str_radix("123",16).unwrap(), 0x123);
    }
}
