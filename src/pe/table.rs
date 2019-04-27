use super::tables::*;

pub const NUM_TABLES: usize = 45;

#[derive(Debug, Clone, PartialEq)]
pub enum Table {
    Assembly(AssemblyTable),
    AssemblyRef(AssemblyRefTable),
    CustomAttribute(CustomAttributeTable),
    MemberRef(MemberRefTable),
    MethodDef(MemberDefTable),
    Module(ModuleTable),
    ModuleRef, // table?
    TypeDef(TypeDefTable),
    TypeRef(TypeRefTable),
}

impl TableKind {
    pub fn from_table(bit_vec: u64) -> Vec<Self> {
        let mut tables = vec![];
        for i in 0..64 {
            if bit_vec & (1 << i) > 0 {
                tables.push(TableKind::from(i as u8))
            }
        }
        tables
    }
}

macro_rules! make_table_conv {
    ($($ident:ident = $expr:expr);* $(;)?) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum TableKind {
            $($ident,)*
        }

        impl From<TableKind> for usize {
            fn from(kind: TableKind) -> Self {
                use TableKind::*;
                match kind {
                    $($ident => $expr,)*
                }
            }
        }

        impl From<u8> for TableKind {
            fn from(val: u8) -> Self {
                use TableKind::*;
                match val {
                    $($expr => $ident,)*
                    _=> unreachable!(),
                }
            }
        }
    };
}

make_table_conv! {
    Module                 = 0x00;
    TypeRef                = 0x01;
    TypeDef                = 0x02;
    Field                  = 0x04;
    MethodDef              = 0x06;
    Param                  = 0x08;
    InterfaceImpl          = 0x09;
    MemberRef              = 0x0A;
    Constant               = 0x0B;
    CustomAttribute        = 0x0C;
    FieldMarshal           = 0x0D;
    DeclSecurity           = 0x0E;
    ClassLayout            = 0x0F;
    FieldLayout            = 0x10;
    StandAloneSig          = 0x11;
    EventMap               = 0x12;
    Event                  = 0x14;
    PropertyMap            = 0x15;
    Property               = 0x17;
    MethodSemantics        = 0x18;
    MethodImpl             = 0x19;
    ModuleRef              = 0x1A;
    TypeSpec               = 0x1B;
    ImplMap                = 0x1C;
    FieldRVA               = 0x1D;
    Assembly               = 0x20;
    AssemblyProcessor      = 0x21;
    AssemblyOs             = 0x22;
    AssemblyRef            = 0x23;
    AssemblyRefProcessor   = 0x24;
    AssemblyRefOs          = 0x25;
    File                   = 0x26;
    ExportedType           = 0x27;
    ManifestResource       = 0x28;
    NestedClass            = 0x29;
    GenericParam           = 0x2A;
    MethodSpec             = 0x2B;
    GenericParamConstraint = 0x2C;
}
