use crate::simulator::instruction::{InstructionType, InstructionMeta};

pub struct Parser;

impl Parser {
    pub fn parse_line(line: &str) -> Option<InstructionMeta> {
        let line = line.trim();

        if line.is_empty() { return None; }

        let parts: Vec<&str> = line
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|s| !s.is_empty())
            .collect();

        let inst_type = parts.get(0)?.to_ascii_uppercase();

        match inst_type.as_str() {
            "L.D" | "S.D" if parts.len() == 3 => {
                let fp   = Self::parse_fp_register(parts[1])?;
                let (offset, base) = Self::parse_memory_address(parts[2])?;

                Some(InstructionMeta {
                    inst_type: if inst_type == "L.D" { InstructionType::LD }
                               else { InstructionType::SD },

                    rd: if inst_type == "L.D" { Some(fp) } else { None },
                    rs: if inst_type == "S.D" { Some(fp) } else { None },
                    rt: None,
                    
                    base: Some(base),
                    offset: Some(offset),
                })
            }

            "ADD.D" | "SUB.D" | "MUL.D" | "DIV.D" if parts.len() == 4 => {
                let rd  = Self::parse_fp_register(parts[1])?;
                let rs  = Self::parse_fp_register(parts[2])?;
                let rt  = Self::parse_fp_register(parts[3])?;

                Some(InstructionMeta {
                    inst_type: match inst_type.as_str() {
                        "ADD.D" => InstructionType::ADDD,
                        "SUB.D" => InstructionType::SUBD,
                        "MUL.D" => InstructionType::MULD,
                        _       => InstructionType::DIVD,
                    },

                    rd: Some(rd),
                    rs: Some(rs),
                    rt: Some(rt),

                    base: None,
                    offset: None,
                })
            }

            _ => None,
        }
    }

    pub fn parse_file(content: &str) -> Vec<InstructionMeta> {
        content.lines().filter_map(Self::parse_line).collect()
    }

    pub fn parse_fp_register(token: &str) -> Option<u8> {
        token.strip_prefix('F')?
             .parse::<u8>().ok()
             .filter(|n| n % 2 == 0 && *n <= 30)
             .map(|n| n / 2)
    }

    pub fn parse_int_register(token: &str) -> Option<u8> {
        token.strip_prefix('R')?.parse::<u8>().ok()
    }

    fn parse_memory_address(expr: &str) -> Option<(i32, u8)> {
        let (offset_str, rest)  = expr.split_once('(')?;
        let (base_str, _) = rest.split_once(')')?;

        let offset = offset_str.parse::<i32>().ok()?;
        let base   = Self::parse_int_register(base_str)?;

        Some((offset, base))
    }
}

pub(crate) fn fp_index_to_name(idx: u8) -> String {
    format!("F{}", idx * 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::fp_index_to_name;
    use std::fs;

    fn fmt_fp(opt: Option<u8>) -> Option<String> {
        opt.map(fp_index_to_name)
    }

    #[test]
    fn parse_test() {
        let content = fs::read_to_string("data/simple2.txt")
            .expect("Failed to read txt file.");

        let metas = Parser::parse_file(&content);

        let dump = metas
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let rd = fmt_fp(m.rd).unwrap_or("-".to_string());
                let rs = fmt_fp(m.rs).unwrap_or("-".to_string());
                let rt = fmt_fp(m.rt).unwrap_or("-".to_string());
                let base = m.base.map(|r| format!("R{}", r)).unwrap_or("-".into());
                let offset = m.offset.map_or("-".to_string(), |v| v.to_string());
                
            format!(
                "{:<03}:  {:<6} rd: {:<6} rs: {:<6} rt: {:<6} offset: {:<6} base: {}",
                i, format!("{:?}", m.inst_type),
                rd, rs, rt, offset, base
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

        println!("\n=================================================\n{}", dump);

        fs::write("output.txt", dump).expect("Write failed");

        assert!(!metas.is_empty(), "Parser returned 0 instruction");
    }
}
