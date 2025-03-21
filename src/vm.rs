// ©2025 - BestMat - All rights reserved.
#[allow(non_snake_case, dead_code)]

use std::collections::HashMap;
use std::fmt::Binary;

const BITS: i8 = 9;

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryValue <'a> {
    id: String,
    value: &'a str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RustVM <'a> {
    pub memory: Vec<Vec<MemoryValue<'a>>>,      // Memory for the virtual machine
    pub flagger: HashMap<String, bool>,         // Flagger for the virtual machine
    pub registers: HashMap<&'a str, String>,    // Registers for the virtual machine
}

impl <'a> RustVM <'a> {
    pub fn new() -> RustVM<'a> {
        let ( memory, flagger ) = initVM();

        return Self {
            memory,
            flagger,
            registers: HashMap::new(),
        };
    }

    #[allow(non_snake_case)]
    pub fn memalloc(&mut self, value: Vec<&'a str>, bits: usize) -> String {
        for col in 0..BITS {
            for row in 0..BITS {
                let mut id: String = String::new();

                if col < 10 && row < 10 {
                    id = format!("0{}0{}", col, row);
                } else if col < 10 {
                    id = format!("0{}{}", col, row);
                } else if row < 10 {
                    id = format!("{}0{}", col, row);
                } else {
                    id = format!("{}{}", col, row);
                }

                if bits == 1 {
                    if self.isFree(id.clone()) {
                        self.memory[col as usize][row as usize].value = value[0];
                        self.flagger.insert(id.clone(), true);
                        return id;
                    }
                } else {
                    let mut emptyCells: Vec<String> = Vec::new();

                    for nrow in row..(bits as i8) {
                        let id = self.memory[col as usize][nrow as usize].id.clone();

                        if *self.flagger.get(&id).unwrap() == false {
                            match emptyCells.last() {
                                None => emptyCells.push(id.clone()),
                                Some(lastcell) => {
                                    if *lastcell != id {
                                        emptyCells.push(id.clone())
                                    }
                                }
                            }

                            let nid = self.memory[col as usize][(nrow + 1) as usize].id.clone(); 

                            if *self.flagger.get(&nid).unwrap() == false {
                                emptyCells.push(nid);
                            } 
                        }
                    }

                    if emptyCells.len() != bits {
                        emptyCells = Vec::new();

                        for nrow in row..(bits as i8) {
                            let id = self.memory[(col + 1) as usize][nrow as usize].id.clone();

                            if *self.flagger.get(&id).unwrap() == false {
                                match emptyCells.last() {
                                    None => emptyCells.push(id.clone()),
                                    Some(lastcell) => {
                                        if *lastcell != id {
                                            emptyCells.push(id.clone())
                                        }
                                    }
                                }

                                let nid = self.memory[(col + 1) as usize][(nrow + 1) as usize].id.clone(); 

                                if *self.flagger.get(&nid).unwrap() == false {
                                    emptyCells.push(nid);
                                } 
                            }
                        }
                    }

                    if value.len() != bits {
                        panic!(
                            "RustVM: Cannot allocate {} bit(s) in {} requested bit(s).",
                            value.len(),
                            bits
                        );
                    }
                    
                    let mut startPointerID: String = String::new();

                    for i in 0..(emptyCells.len() - 1 as usize) {
                        let emptyCellCol = format!(
                            "{}{}", emptyCells[i].chars().nth(0).unwrap(),
                            emptyCells[i].chars().nth(1).unwrap()
                        );

                        let emptyCellRow = format!(
                            "{}{}", emptyCells[i].chars().nth(2).unwrap(),
                            emptyCells[i].chars().nth(3).unwrap()
                        );
                        
                        println!("{}", emptyCells[i]);

                        self.memory[emptyCellCol.parse::<usize>().unwrap()][emptyCellRow.parse::<usize>().unwrap()].value = value[i];

                        startPointerID = emptyCells[i].clone();

                        self.flagger.insert(emptyCells[i].clone(), true);
                    }
                    
                    return startPointerID;
                }
            }
        }

        panic!(
            "RustVM: The memory of the machine is full. To allocate more memory please free unused memory."
        );
    }

    pub fn memfree(&mut self, id: String) {
        let col = format!("{}{}", id.chars().nth(0).unwrap(), id.chars().nth(1).unwrap());
        let row = format!("{}{}", id.chars().nth(2).unwrap(), id.chars().nth(3).unwrap());

        if *self.flagger.get(&id).unwrap() == false {
            panic!("RustVM: {} is already freed. Cannot free a null pointer.", id);
        }
        
        self.memory[col.parse::<usize>().unwrap()][row.parse::<usize>().unwrap()].value = "00000000";
        self.flagger.insert(id.clone(), false);
    }

    pub fn memget(&self, id: String) -> &'a str {
        let col = format!("{}{}", id.chars().nth(0).unwrap(), id.chars().nth(1).unwrap());
        let row = format!("{}{}", id.chars().nth(2).unwrap(), id.chars().nth(3).unwrap());

        if *self.flagger.get(&id).unwrap() == false {
            panic!("RustVM: {} is not allocated. Cannot access a memory location which is null.", id);
        }

        return self.memory[col.parse::<usize>().unwrap()][row.parse::<usize>().unwrap()].value;
    }

    #[allow(non_snake_case)]
    pub fn regmov(&mut self, reg: &'a str, val: &'a str) {
        let mut isRegAvail: bool = false;
        let registers = vec!["r1", "r2", "r3", "r4", "r5", "acc"];

        for register in registers {
            if register == reg {
                isRegAvail = true;
            }
        }

        if !isRegAvail {
            panic!("RustVM: Unknown register {}.", reg);
        }

        let cellid = self.memalloc(vec![val], 1);
        self.registers.insert(reg, cellid);
    }

    pub fn regget(&self, reg: &'a str) -> &'a str {
        return self.memget(self.registers.get(reg).unwrap().to_string());
    }

    pub fn intalloc<T: Binary>(&mut self, val: T, bits: usize) -> String {
        if bits == 8 {
            let binary = format!("{val:08b}").leak();
            return self.memalloc(vec![binary], 1);
        }
        
        panic!(
            "RustVM: Cannot allocate integer of bits {}", bits
        );
    }

    pub fn stralloc(&mut self, val: &'a str, len: &'a str) -> String {
        let characters = val.split_terminator("").skip(1).collect::<Vec<_>>();
        let mut args: Vec<&str> = Vec::new();

        args.push(len);

        for character in characters.clone() {
            args.push(character);
        }

        args.push("00000000");
        args.push("00000000");
        
        return self.memalloc(args, characters.len() + 3);
    }

    #[allow(non_snake_case)]
    fn isFree(&self, id: String) -> bool {
        return !(*self.flagger.get(&id).unwrap());
    }
}

#[allow(non_snake_case)]
fn initVM<'a>() -> (Vec<Vec<MemoryValue<'a>>>, HashMap<String, bool>) {
    let mut memory: Vec<Vec<MemoryValue<'a>>> = Vec::new();
    let mut flagger: HashMap<String, bool> = HashMap::new();

    for col in 0..BITS {
        memory.push(initMemoryValue(col as usize));
    }

    for col in 0..BITS {
        for row in 0..BITS {
            let mut id = String::new();

            if col < 10 && row < 10 {
                id = format!("0{}0{}", col, row);
            } else if col < 10 {
                id = format!("0{}{}", col, row);
            } else if row < 10 {
                id = format!("{}0{}", col, row);
            } else {
                id = format!("{}{}", col, row);
            }
            
            flagger.insert(id, false);
        }
    }

    return (memory, flagger);
}

#[allow(non_snake_case)]
fn initMemoryValue<'a>(col: usize) -> Vec<MemoryValue<'a>> {
    let mut vals: Vec<MemoryValue<'a>> = Vec::new();

    for row in 0..BITS {
        let mut id = String::new();

        if col < 10 && row < 10 {
            id = format!("0{}0{}", col, row);
        } else if col < 10 {
            id = format!("0{}{}", col, row);
        } else if row < 10 {
            id = format!("{}0{}", col, row);
        } else {
            id = format!("{}{}", col, row);
        }

        vals.push(MemoryValue {
            id,
            value: "00000000",
        });
    }

    return vals;
}
