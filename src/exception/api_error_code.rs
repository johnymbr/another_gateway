pub struct ApiErrorCode(pub &'static str, pub &'static str);

// Generic errors.
pub const ERR_DB_CONNECTION_ERROR: ApiErrorCode = ApiErrorCode("G9000", "Error when connecting to database.");
pub const ERR_INVALID_REQUEST: ApiErrorCode = ApiErrorCode("G0001", "Invalid request");

// Field errors.
pub const ERR_REQUIRED_FIELD: ApiErrorCode = ApiErrorCode("F0001", "This field is required.");
pub const ERR_MIN_SIZE: ApiErrorCode = ApiErrorCode("F0002", "This field must has a minimum amount of characters.");

// Application errors.
pub const APP_ERR_INSERTING_ERROR: ApiErrorCode = ApiErrorCode("APP0001", "Error when inserting a new application.");