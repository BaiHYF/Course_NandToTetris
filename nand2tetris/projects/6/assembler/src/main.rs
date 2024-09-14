use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref SYMBOL_TABLE: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        m.insert(String::from("R0"), String::from("0"));
        m.insert(String::from("R1"), String::from("1"));
        m.insert(String::from("R2"), String::from("2"));
        m.insert(String::from("R3"), String::from("3"));
        m.insert(String::from("R4"), String::from("4"));
        m.insert(String::from("R5"), String::from("5"));
        m.insert(String::from("R6"), String::from("6"));
        m.insert(String::from("R7"), String::from("7"));
        m.insert(String::from("R8"), String::from("8"));
        m.insert(String::from("R9"), String::from("9"));
        m.insert(String::from("R10"), String::from("10"));
        m.insert(String::from("R11"), String::from("11"));
        m.insert(String::from("R12"), String::from("12"));
        m.insert(String::from("R13"), String::from("13"));
        m.insert(String::from("R14"), String::from("14"));
        m.insert(String::from("R15"), String::from("15"));
        m.insert(String::from("SCREEN"), String::from("16384"));
        m.insert(String::from("KBD"), String::from("24576"));
        m.insert(String::from("SP"), String::from("0"));
        m.insert(String::from("LCL"), String::from("1"));
        m.insert(String::from("ARG"), String::from("2"));
        m.insert(String::from("THIS"), String::from("3"));
        m.insert(String::from("THAT"), String::from("4"));
        Mutex::new(m)
    };
}

static mut VAR_ADDR: i32 = 16;

// static mut SYMBOL_TABLE: HashMap<String, String> = HashMap::new();

fn main() {
    // 从控制台第一个参数读取 input 文件名
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    let name: Vec<&str> = input_file.split(".").collect();
    println!("DEBUG: {}.{}", name[0], name[1]);
    let outname = name[0];

    // Read the input file, stores it in a String
    let input_file = std::fs::read_to_string(input_file).expect("Failed to read input file");
    let mut lines: Vec<&str> = input_file.lines().collect();
    // 丢弃空行与 "//" 开头的注释行
    lines.retain(|line| !line.is_empty() && !line.starts_with("//"));
    
    // create a new file named "new.asm"
    let output_file = format!("{}{}", outname.to_string(), ".hack");
    println!("DEBUG: {}", output_file);
    let mut file = File::create(output_file).expect("Failed to create output file");

    let mut line_num = 0;
    for line in lines {
        let line = line.trim();
        if line.starts_with("//") {
            // Handle label
            continue;
        }
        println!("parsing: {}", line);
        let line = symbol_parser(line, line_num);
        line_num += 1;
        if line == "" {
            continue;
        }
        writeln!(file, "{}", code_generate(&line)).expect("Failed to write to output file");
    }
}

/// Parse a line of instruction and return a binary code.
fn code_generate(inst: &str) -> String {
    if inst.starts_with("@") {
        // Handle A instrustion
        return a_parser(inst);
    } else {
        // Handle C instrustion
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
    let split: Vec<&str> = inst.split(";").collect();
    let jump = if split.len() > 1 { split[1].trim() } else { "" };

    let split: Vec<&str> = split[0].split("=").collect();
    let comp = if split.len() > 1 {
        split[1].trim()
    } else {
        split[0].trim()
    };
    let dest = if split.len() > 1 { split[0].trim() } else { "" };

    // println!("DEBUG: inst: {}, dest: {}, comp: {}, jump: {}", inst, dest, comp, jump);

    let jjj = match jump {
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => "000", // no jump
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
        _ => "0000000",
    };

    let ddd = match dest {
        "M" => "001",
        "D" => "010",
        "DM" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "ADM" => "111",
        _ => "000", // the value is not stored
    };

    format!("111{}{}{}", acccccc, ddd, jjj)
}

fn symbol_parser(inst: &str, line_num: i32) -> String {
    if inst.starts_with("(") {
        let label = inst.trim_matches(|c| c == '(' || c == ')').to_string();
        SYMBOL_TABLE.lock().unwrap().insert(label, line_num.to_string());
        return "".to_string();  // return empty string
    }

    // if not an A instruction, return what is inputed
    if !inst.starts_with("@") {
        return inst.to_string();
    } else {
        let num = &inst[1..];
        // if it is a number, return it
        if num.parse::<i32>().is_ok() {
            return inst.to_string();
        } else {
            // if it is a symbol, return the value of the symbol
            let sym = num.trim();
            
            let val = if SYMBOL_TABLE.lock().unwrap().contains_key(sym) {
                SYMBOL_TABLE.lock().unwrap().get(sym).unwrap().to_string()
            } else {
                unsafe {
                    println!("DEBUG: {}:{}", sym, VAR_ADDR);
                    SYMBOL_TABLE.lock().unwrap().insert(sym.to_string(), VAR_ADDR.to_string());
                    VAR_ADDR += 1;
                    (VAR_ADDR - 1).to_string()
                }
            };
            return format!("@{}", val);
        }
    }
}

#[cfg(test)]
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

    #[test]
    fn test_symbol_parser() {
        assert_eq!(symbol_parser("@R2", 0), "@2");
        assert_eq!(symbol_parser("@KBD", 0), "@24576");
        assert_eq!(symbol_parser("@3", 3), "@3");
        assert_eq!(symbol_parser("D=1", 4), "D=1");
        assert_eq!(symbol_parser("(LOOP)", 4), "");
    }
}
