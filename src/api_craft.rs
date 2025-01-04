pub mod respbody {
    pub use api_response::{
        ApiError, ApiResponse, ApiResult, ApiSuccessResponse, Cost, DefaultMeta, ErrWrapper, ErrorResponse, IntoError,
        MaybeString, Pagination, RateLimit, SuccessResponse, UserMeta,
    };
}

pub mod errtype {
    pub use api_errtype::*;
    pub use api_response::{
        api_err,
        error_code::{
            tally::{tally_err_decl, ErrDeclTally, ErrDeclTree, ErrDeclTreeText, ErrDeclVecText},
            ErrBrief, ErrDecl, ErrPath, ErrPathParent, ErrPathRoot, ErrType, InvalidErrPathFlag, InvalidErrTypeFlag,
        },
    };
}
