use crate::uefi::SystemTable;

struct SystemTableRefined {
    table: *const SystemTable,
}

impl SystemTableRefined {
    pub fn print(&self, str: &str) {
        let mut buffer:[u16;1] = [0];
        str.charts().for_each(|c| 
            unsafe {
            let utf16 = c.encode_utf16(&buffer);
            ((*(*self.table.output).output_string)((*self.table).output, &utf16[0]));
        });
    }
}
