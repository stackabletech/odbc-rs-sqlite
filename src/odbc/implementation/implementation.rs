use odbc_sys::{InfoType, InfoTypeType};

pub(crate) fn get_info(info_type: InfoType) -> Option<InfoTypeType> {
    match info_type {
        InfoType::ActiveEnvironments => Some(InfoTypeType::SqlUSmallInt(0)),
        InfoType::UserName => Some(InfoTypeType::String("foo".to_string())),
        InfoType::MaxConcurrentActivities => Some(InfoTypeType::SqlUSmallInt(1)),
        InfoType::ScrollOptions => Some(InfoTypeType::SqlUInteger(1)), // Proper u32 for test
        _ => None,
    }
}

/*
pub(crate) fn get_supported_functions() -> Vec<FunctionId> {
    vec![FunctionId::SqlAllocConnect, FunctionId::SqlAllocHandle]
}


           */
