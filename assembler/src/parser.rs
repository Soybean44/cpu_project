use phf::phf_map;

pub enum OpCodes {
    NOP = 0x0000,
    ADD = 0x2000,
    LDI = 0x8000,
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

pub fn parse_assembly(src: &str) -> Vec<u16> {
    let src_arr: Vec<Vec<&str>> = src.lines().map(|x| x.split(" ").collect()).collect();
    let mut machine_code = vec![];
    for item in src_arr {
        let instruction: u16;
        if item.len() < 1 {
            continue;
        }
        match item[0] {
            "NOP" => instruction = OpCodes::NOP as u16,
            "ADD" => {
                if item.len() < 4 {
                    eprintln!("Error add has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::ADD as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | REGISTERS.get(item[2]).cloned().unwrap_or(0x00) << 4
                    | REGISTERS.get(item[3]).cloned().unwrap_or(0x00);
            }
            "LDI" => {
                if item.len() < 3 {
                    eprintln!("Error add has too few arguments");
                    std::process::exit(1);
                }
                instruction = OpCodes::LDI as u16
                    | REGISTERS.get(item[1]).cloned().unwrap_or(0x00) << 8
                    | u16::from_str_radix(&item[2][2..], 16).unwrap_or(0x00);
            }
            _ => continue,
        }
        machine_code.push(instruction);
    }
    return machine_code;
}
