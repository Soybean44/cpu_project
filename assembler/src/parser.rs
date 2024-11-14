use phf::phf_map;

pub enum OpCodes {
    NOP = 0x0000,
    HLT = 0x1000,
    ADD = 0x2000,
    SUB = 0x3000,
    OR = 0x4000,
    AND = 0x5000,
    XOR = 0x6000,
    NOT = 0x7000,
    RSH = 0x8000,
    CMP = 0x9000,
    LDI = 0xA000,
    JE = 0xB000,
    LDR = 0xC000,
    STR = 0xD000,
    DIS = 0xF000,
}

const REGISTERS: phf::Map<&'static str, u16> = phf_map! {
    "r0" => 0x0,
    "r1" => 0x1,
    "r2" => 0x2,
    "r3" => 0x3,
    "r4" => 0x4,
    "r5" => 0x5,
    "r6" => 0x6,
    "r7" => 0x7,
    "r8" => 0x8,
    "r9" => 0x9,
    "r10" => 0xa,
    "r11" => 0xb,
    "r12" => 0xc,
    "r13" => 0xd,
    "r14" => 0xe,
    "r15" => 0xf,
};

fn parse_arg(arg: &str) -> u16 {
    let test = arg.parse::<u8>();
    let chars: Vec<_> = arg.chars().collect();
    if arg[..2] == *"0x" {
        return u16::from_str_radix(&arg[2..], 16).unwrap_or(0x00);
    } else if test.is_ok() {
        return u16::from_str_radix(&arg[2..], 10).unwrap_or(0x00);
    } else if chars[0] == '"' && chars.len() == 3 {
        return chars[1] as u16;
    } else {
        eprintln!("Error in argument parsing");
        std::process::exit(1);
    }
}

pub fn parse_assembly(src: &str) -> Vec<u16> {
    let cleaned_src = src.replace("\" \"", "0x20");
    let src_arr: Vec<Vec<&str>> = cleaned_src
        .lines()
        .map(|x| x.split(" ").collect())
        .collect();
    let mut machine_code = vec![];
    for item in src_arr {
        let instruction: u16;
        if item.len() < 1 {
            continue;
        }
        match item[0] {
            "NOP" => instruction = OpCodes::NOP as u16,
            "HLT" => instruction = OpCodes::HLT as u16,
            "ADD" => {
                if item.len() < 4 {
                    eprintln!("Error ADD has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::ADD as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | REGISTERS.get(item[2]).cloned().unwrap_or(0x00) << 4
                    | REGISTERS.get(item[3]).cloned().unwrap_or(0x00);
            }
            "SUB" => {
                if item.len() < 4 {
                    eprintln!("Error SUB has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::SUB as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | REGISTERS.get(item[2]).cloned().unwrap_or(0x00) << 4
                    | REGISTERS.get(item[3]).cloned().unwrap_or(0x00);
            }
            "OR" => {
                if item.len() < 4 {
                    eprintln!("Error OR has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::OR as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | REGISTERS.get(item[2]).cloned().unwrap_or(0x00) << 4
                    | REGISTERS.get(item[3]).cloned().unwrap_or(0x00);
            }
            "AND" => {
                if item.len() < 4 {
                    eprintln!("Error AND has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::AND as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | REGISTERS.get(item[2]).cloned().unwrap_or(0x00) << 4
                    | REGISTERS.get(item[3]).cloned().unwrap_or(0x00);
            }
            "XOR" => {
                if item.len() < 4 {
                    eprintln!("Error XOR has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::XOR as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | REGISTERS.get(item[2]).cloned().unwrap_or(0x00) << 4
                    | REGISTERS.get(item[3]).cloned().unwrap_or(0x00);
            }
            "NOT" => {
                if item.len() < 3 {
                    eprintln!("Error NOT has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::NOT as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | REGISTERS.get(item[2]).cloned().unwrap_or(0x00);
            }
            "RSH" => {
                if item.len() < 3 {
                    eprintln!("Error RSH has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::RSH as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | REGISTERS.get(item[2]).cloned().unwrap_or(0x00);
            }
            "CMP" => {
                if item.len() < 4 {
                    eprintln!("Error CMP has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::CMP as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | REGISTERS.get(item[2]).cloned().unwrap_or(0x00) << 4
                    | REGISTERS.get(item[3]).cloned().unwrap_or(0x00);
            }
            "LDI" => {
                if item.len() < 3 {
                    eprintln!("Error LDI has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::LDI as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | parse_arg(&item[2]);
            }
            "JE" => {
                if item.len() < 3 {
                    eprintln!("Error JE has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::JE as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | parse_arg(&item[2]);
            }
            "LDR" => {
                if item.len() < 3 {
                    eprintln!("Error LDR has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::LDR as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | parse_arg(&item[2]);
            }
            "STR" => {
                if item.len() < 3 {
                    eprintln!("Error STR has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::STR as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | parse_arg(&item[2]);
            }
            "DIS" => {
                panic!("DIS is depecated")
            }
            _ => continue,
        }
        machine_code.push(instruction);
    }
    return machine_code;
}
