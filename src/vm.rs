// ©2025 - BestMat - All rights reserved.
#[allow(non_snake_case, dead_code)]

use std::collections::HashMap;

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
}

impl <'a> RustVM <'a> {
    pub fn new() -> RustVM<'a> {
        let ( memory, flagger ) = initVM();

        return Self {
            memory,
            flagger,
        };
    }

    pub fn alloc(&mut self, value: &'a str) -> String {
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

                if self.isFree(id.clone()) {
                    self.memory[col as usize][row as usize].value = value;
                    self.flagger.insert(id.clone(), true);
                    return id;
                }
            }
        }

        panic!(
            "RustVM: The memory of the machine is full. To allocate more memory please free unused memory."
        );
    }

    pub fn free(&mut self, id: String) {
        let col = format!("{}{}", id.chars().nth(0).unwrap(), id.chars().nth(1).unwrap());
        let row = format!("{}{}", id.chars().nth(2).unwrap(), id.chars().nth(3).unwrap());
        
        self.memory[col.parse::<usize>().unwrap()][row.parse::<usize>().unwrap()].value = "00000000";
        self.flagger.insert(id.clone(), false);
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
