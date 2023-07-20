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

fn parse(_line: &str) -> (u8, Vec<u8>) {
    (0, vec![0])
}
