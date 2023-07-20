pub fn assemble(program: &str) -> Vec<u8> {
    let lines = program.lines().map(|x| x.trim()).collect::<Vec<&str>>();
    let mut bytes = vec![];

    for line in lines {
        let line = line.trim();
        if !line.is_empty() {
            let (operation, mut args) = parse(line);
            bytes.push(operation);
            bytes.append(&mut args);
        }
    }

    bytes
}

    fn parse(line: &str) -> (u8, Vec<u8>) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        let opcode = match parts[0] {
            "LDA" => 0xA9,
            "TAX" => 0xAA,
            "INX" => 0xE8,
            _ => 0x00
        };
        
        let mut args = vec![];
        
        if parts.len() > 1 {
            if let Ok(value) = u8::from_str_radix(&parts[1][2..], 16) {
                args.push(value); 
            }
        }
        
        (opcode, args)
    }
