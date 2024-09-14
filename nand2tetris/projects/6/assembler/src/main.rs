use std::fmt::Debug;

fn main() {
    // 1. gets input file, stores it in a String
    // 2. splits the String by lines into a vector of Strings
    // 3. let symbol parser to handle
    // 4. let code generater to handle
    // 5. output the result to `out.hack`

    // 从控制台第一个参数读取 input 文件名
    let args: Vec<String> = std::env::args().collect();
    let input_file = &args[1];
    
    // Read the input file, stores it in a String
    let input_file = std::fs::read_to_string(input_file).expect("Failed to read input file");
    let lines: Vec<&str> = input_file.lines().collect();
    // 丢弃空行与 "//" 开头的注释行
    let lines: Vec<&str> = lines.into_iter().filter(|line| !line.is_empty() && !line.starts_with("//")).collect();
    let mut lines: Vec<&str>;
}


/// Parse a line of instruction and return a binary code.
fn code_generate(inst: &str) -> String {
    if inst.starts_with("@") { // Handle A instrustion
        return a_parser(inst);
    } else { // Handle C instrustion
        return c_parser(inst);
    }
}

/// Parse an A instruction and return its binary code.
/// 
/// "@xxx" -> "0"+"xxx in binary"
/// 
/// ```
/// assert_eq!(a_parser("@17"), "0000000000010001");
/// assert_eq!(a_parser("@14"), "0000000000001110");
/// ```
fn a_parser(inst: &str) -> String {
    let num = &inst[1..];
    let decimal_num = num.parse::<u16>().unwrap();
    format!("0{:0>15b}", decimal_num)
}

/// Parse a C instruction and return its binary code.
/// "dest=comp;jump"
/// 
/// ```
/// assert_eq!(c_parser("A=-1"), "1110111010100000");
/// assert_eq!(c_parser("D=D+1 ; JLE"), "1110011111010110");
/// assert_eq!(c_parser("0;JMP"), "1110101010000111");
/// ```
fn c_parser(inst: &str) -> String {
    // parse `dest`, `comp` and `jump`
    let (mut dest, mut comp, mut jump) = ("", "", "");

    let split: Vec<&str> = inst.split(";").collect();
    jump = if split.len() > 1 { split[1].trim() } else { "" };

    let split: Vec<&str> = split[0].split("=").collect();
    comp = if split.len() > 1 { split[1].trim() } else { split[0].trim() };
    dest = if split.len() > 1 { split[0].trim() } else { "" };

    println!("DEBUG: inst: {}, dest: {}, comp: {}, jump: {}", inst, dest, comp, jump);

    let jjj = match jump {
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => "000" // no jump
    };

    let acccccc = match comp {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        _ => "0000000"
    };

    let ddd = match dest {
        "M" => "001",
        "D" => "010",
        "DM" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "ADM" => "111",
        _ => "000" // the value is not stored
    };
    
    format!("111{}{}{}", acccccc, ddd, jjj)
}

fn symbol_parser(inst: &str) -> String {
    // if not an A instruction, return what is inputed
    if !inst.starts_with("@") {
       return inst.to_string();
    } 
    !unimplemented!("");
}



mod tests {
    use super::*;

    #[test]
    fn test_a_parser() {
        assert_eq!(a_parser("@17"), "0000000000010001");
        assert_eq!(a_parser("@14"), "0000000000001110");
    }

    #[test]
    fn test_c_parser() {
        assert_eq!(c_parser("A=-1"), "1110111010100000");
        assert_eq!(c_parser("D=D+1 ; JLE"), "1110011111010110");
        assert_eq!(c_parser("0;JMP"), "1110101010000111");
    }
}