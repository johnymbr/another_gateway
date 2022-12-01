use hyper::StatusCode;

use crate::{
    exception::{ApiError, ApiFieldError},
    model::{Application, ApplicationReq},
};

struct ApplicationService {}

impl ApplicationService {
    fn save(entity: ApplicationReq) -> Result<Application, ApiError> {
        let mut field_errors = Vec::<ApiFieldError>::new();

        let mut name = String::new();
        match entity.name {
            Some(name) => {
                if name.len() < 3 {
                    field_errors.push(ApiFieldError {
                        code: "F0001".to_owned(),
                        message: "Field need to have at least 3 characters.".to_owned(),
                        field: "application.name".to_owned(),
                    });
                }
            }
            None => {
                field_errors.push(ApiFieldError {
                    code: "F0001".to_owned(),
                    message: "Field is required".to_owned(),
                    field: "application.name".to_owned(),
                });
            }
        }

        Ok(())
    }
}
