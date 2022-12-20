pub struct ApiErrorCode(pub &'static str, pub &'static str);

// Generic errors.
pub const ERR_DB_CONNECTION_ERROR: ApiErrorCode = ApiErrorCode("G9000", "Error when connecting to database.");
pub const ERR_INVALID_REQUEST: ApiErrorCode = ApiErrorCode("G0001", "Invalid request");

// Pagination errors.
pub const PG_ERR_PAGE_REQUIRED: ApiErrorCode = ApiErrorCode("PG0001", "Param page is required.");
pub const PG_ERR_PAGE_SIZE_REQUIRED: ApiErrorCode = ApiErrorCode("PG0002", "Param pageSize is required.");

// Field errors.
pub const ERR_REQUIRED_FIELD: ApiErrorCode = ApiErrorCode("F0001", "This field is required.");
pub const ERR_MIN_SIZE: ApiErrorCode = ApiErrorCode("F0002", "This field must has a minimum amount of characters.");

// Application errors.
pub const APP_ERR_INSERTING: ApiErrorCode = ApiErrorCode("APP0001", "Error when insert a new application.");
pub const APP_ERR_FINDING_PAGINATED: ApiErrorCode = ApiErrorCode("APP0002", "Error when search applications with pagination.");
pub const APP_ERR_FIND_BY_ID: ApiErrorCode = ApiErrorCode("APP0003", "Error when search an application by id.");
pub const APP_ERR_NOT_FOUND: ApiErrorCode = ApiErrorCode("APP0004", "Application wasn't find.");
pub const APP_ERR_UPDATING: ApiErrorCode = ApiErrorCode("APP0005", "Error when update an application.");
pub const APP_ERR_DELETE: ApiErrorCode = ApiErrorCode("APP0006", "Error when delete an application.");
pub const APP_ERR_ID_IS_REQUIRED: ApiErrorCode = ApiErrorCode("APP0007", "The Id of Application is required.");