use crate::enums::data_directory_base::ImageDataDirectoryBase;

pub struct OptionalHeaderDataDirectories{
    export_table: Option<ImageDataDirectoryBase>,
    import_table: Option<ImageDataDirectoryBase>,
    exception_table: Option<ImageDataDirectoryBase>,
    certificate_table: Option<ImageDataDirectoryBase>,
    base_relocation_table: Option<ImageDataDirectoryBase>,
    debug: Option<ImageDataDirectoryBase>,
    architecture: Option<ImageDataDirectoryBase>,
    global_ptr: Option<ImageDataDirectoryBase>,
    tls_table: Option<ImageDataDirectoryBase>,
    load_config_table: Option<ImageDataDirectoryBase>,
    bound_import: Option<ImageDataDirectoryBase>,
    iat: Option<ImageDataDirectoryBase>,
    delay_import_descriptor: Option<ImageDataDirectoryBase>,
    clr_runtime_header: Option<ImageDataDirectoryBase>,
    reserved: Option<ImageDataDirectoryBase>,
}