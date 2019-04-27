use std::io::{Seek, Read};
use crate::pe::*;

impl<R: Read + Seek> ReadPe<R> for MetaDataStream {
    fn read(rd: &mut Reader<R>) -> Result<Self> {
        cmp!(rd.read_u32()? == 0); // reserved

        let major_version = rd.read_u8()?;
        let minor_version = rd.read_u8()?;
        let heap_sizes = rd.read_u8()?;

        cmp!(rd.read_u8()? == 1); // reserved

        let valid = rd.read_u64()?;
        let sorted = rd.read_u64()?;

        let max = valid.count_ones();
        let mut rows = Vec::with_capacity(max as usize);
        for _ in 0..max {
            rows.push(rd.read_u32()?);
        }

        let kinds = TableKind::from_table(valid);
        let mut tables = vec![vec![]; crate::pe::NUM_TABLES];
        for (i, kind) in kinds.iter().enumerate() {
            for _ in 0..rows[i] {
                let n: usize = (*kind).into();
                tables[n].push(match kind {
                    TableKind::Assembly => Table::Assembly(rd.read_struct()?),
                    TableKind::AssemblyRef => Table::AssemblyRef(rd.read_struct()?),
                    TableKind::CustomAttribute => Table::CustomAttribute(rd.read_struct()?),
                    TableKind::MethodDef => Table::MethodDef(rd.read_struct()?),
                    TableKind::MemberRef => Table::MemberRef(rd.read_struct()?),
                    TableKind::TypeDef => Table::TypeDef(rd.read_struct()?),
                    TableKind::TypeRef => Table::TypeRef(rd.read_struct()?),
                    TableKind::Module => Table::Module(rd.read_struct()?),
                    _ => Table::ModuleRef,
                })
            }
        }

        Ok(MetaDataStream {
            major_version,
            minor_version,
            heap_sizes,
            valid,
            sorted,
            rows,
            tables,
        })
    }
}
