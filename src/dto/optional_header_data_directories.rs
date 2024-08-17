use crate::enums::data_directory_base::DataDirectoryBase;

pub struct OptionalHeaderDataDirectories {
    export_table: Option<DataDirectoryBase>,
    import_table: Option<DataDirectoryBase>,
    exception_table: Option<DataDirectoryBase>,
    certificate_table: Option<DataDirectoryBase>,
    base_relocation_table: Option<DataDirectoryBase>,
    debug: Option<DataDirectoryBase>,
    architecture: Option<DataDirectoryBase>,
    global_ptr: Option<DataDirectoryBase>,
    tls_table: Option<DataDirectoryBase>,
    load_config_table: Option<DataDirectoryBase>,
    bound_import: Option<DataDirectoryBase>,
    iat: Option<DataDirectoryBase>,
    delay_import_descriptor: Option<DataDirectoryBase>,
    clr_runtime_header: Option<DataDirectoryBase>,
    reserved: Option<DataDirectoryBase>,
}

impl OptionalHeaderDataDirectories {
    pub fn new(export_table: Option<DataDirectoryBase>, import_table: Option<DataDirectoryBase>, exception_table: Option<DataDirectoryBase>, certificate_table: Option<DataDirectoryBase>, base_relocation_table: Option<DataDirectoryBase>, debug: Option<DataDirectoryBase>, architecture: Option<DataDirectoryBase>, global_ptr: Option<DataDirectoryBase>, tls_table: Option<DataDirectoryBase>, load_config_table: Option<DataDirectoryBase>, bound_import: Option<DataDirectoryBase>, iat: Option<DataDirectoryBase>, delay_import_descriptor: Option<DataDirectoryBase>, clr_runtime_header: Option<DataDirectoryBase>, reserved: Option<DataDirectoryBase>) -> Self {
        Self { export_table, import_table, exception_table, certificate_table, base_relocation_table, debug, architecture, global_ptr, tls_table, load_config_table, bound_import, iat, delay_import_descriptor, clr_runtime_header, reserved }
    }
}
