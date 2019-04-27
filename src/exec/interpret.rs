use crate::pe::{Image, MethodBodyRef, Table};
use crate::exec::{Instruction};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Int32(i32),
    String(u32),
}

impl Value {
    fn as_string(self) -> Option<u32> {
        match self {
            Value::String(d) => Some(d),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Interpreter {
    stack: [Value; 1024],
    bp: usize,
    sp: usize,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            stack: [Value::Int32(0); 1024],
            bp: 0,
            sp: 0,
        }
    }
}

impl Interpreter {
    pub fn run(&mut self, image: &mut Image, method: MethodBodyRef) {
        for inst in &method.borrow().body {
            match inst {
                Instruction::Ldstr { us_offset } => self.load_str(*us_offset),
                Instruction::Call { table, entry } => self.call(&image, *table, *entry),
                _ => {}
            }
            log::trace!("stack: {:#?}", &self.stack[0..4])
        }
    }

    #[inline]
    pub fn push(&mut self, val: Value) {
        self.stack[self.bp + self.sp] = val;
        self.sp += 1;
    }

    #[inline]
    pub fn pop(&mut self) -> Value {
        self.sp -= 1;
        self.stack[self.bp + self.sp]
    }
}

impl Interpreter {
    #[inline]
    fn load_str(&mut self, val: u32) {
        self.push(Value::String(val))
    }

    #[inline]
    fn call(&mut self, image: &Image, table: usize, entry: usize) {
        if let Table::MemberRef(mem_table) =
            &image.metadata.metadata_stream.tables[table][entry - 1]
        {
            let (table, entry) = mem_table.class_table_and_entry();
            let class = &image.metadata.metadata_stream.tables[table][entry - 1];

            if let Table::TypeRef(ty_table) = class {
                let (table, entry) = ty_table.resolution_scope_table_and_entry();
                let aref = match image.metadata.metadata_stream.tables[table][entry - 1] {
                    Table::AssemblyRef(table) => table,
                    _ => unimplemented!(),
                };

                let ar_name = image.get_string(aref.name);
                let ty_namespace = image.get_string(ty_table.type_namespace);
                let ty_name = image.get_string(ty_table.type_name);
                let name = image.get_string(mem_table.name);

                log::info!("[{}]{}.{}::{}", ar_name, ty_namespace, ty_name, name);
                match (ar_name, ty_namespace, ty_name, name) {
                    ("mscorlib", "System", "Console", "WriteLine") => {
                        let val = self.pop().as_string().unwrap();
                        let out = String::from_utf16_lossy(&image.metadata.user_strings[&val]);
                        println!("{}", out)
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }
}
