use crate::odbc::implementation::alloc_handles::EnvironmentHandle;
use odbc_sys::AttrOdbcVersion;

// TODO: Make this return something.... a Result?
pub(crate) fn set_odbc_version(env: &mut EnvironmentHandle, odbc_version: AttrOdbcVersion) {
    env.odbc_version = odbc_version;
}

pub(crate) fn get_odbc_version(env: &EnvironmentHandle) -> AttrOdbcVersion {
    env.odbc_version.clone()
}
