use std::sync::Arc;

use axum::async_trait;
use hyper::StatusCode;
use sqlx::PgPool;

use crate::{
    exception::{ApiError, APP_ERR_NOT_FOUND},
    model::{Application, ApplicationReq, Pagination, PaginationResponse},
    repository::{ApplicationRepository, ApplicationRepositoryTrait},
};

#[async_trait]
pub trait ApplicationServiceTrait {
    async fn find_all(
        &self,
        pagination: Pagination,
    ) -> Result<PaginationResponse<Application>, ApiError>;

    async fn find_by_id(&self, id: i64) -> Result<Application, ApiError>;

    async fn save(&self, entity: ApplicationReq) -> Result<Application, ApiError>;
}

pub struct ApplicationService {
    application_repository: Arc<dyn ApplicationRepositoryTrait + Send + Sync>,
}

#[async_trait]
impl ApplicationServiceTrait for ApplicationService {
    async fn find_all(
        &self,
        pagination: Pagination,
    ) -> Result<PaginationResponse<Application>, ApiError> {
        pagination.validate()?;

        let response = self.application_repository.find_all(pagination).await?;
        Ok(response)
    }

    async fn find_by_id(&self, id: i64) -> Result<Application, ApiError> {
        let response = self.application_repository.find_by_id(id).await?;

        if let None = response {
            return Err(ApiError::new_with_status(
                StatusCode::NOT_FOUND,
                APP_ERR_NOT_FOUND,
            ));
        }
        Ok(response.unwrap())
    }

    async fn save(&self, entity: ApplicationReq) -> Result<Application, ApiError> {
        entity.validate()?;

        let application = self.application_repository.save(entity).await?;
        Ok(application)
    }
}

impl ApplicationService {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        ApplicationService {
            application_repository: Arc::new(ApplicationRepository { pg_pool }),
        }
    }

    pub fn new_with_repo(repository: Arc<dyn ApplicationRepositoryTrait + Send + Sync>) -> Self {
        ApplicationService {
            application_repository: repository,
        }
    }
}

#[cfg(test)]
mod application_service_test {
    use std::str::FromStr;

    use chrono::Utc;

    use crate::{
        exception::{PG_ERR_PAGE_REQUIRED, PG_ERR_PAGE_SIZE_REQUIRED, APP_ERR_INSERTING},
        repository::MockApplicationRepositoryTrait, model::StringMinSize3,
    };

    use super::*;

    #[tokio::test]
    async fn find_all() {
        let mut mock_repo = MockApplicationRepositoryTrait::new();
        mock_repo.expect_find_all().returning(|pagination| {
            Ok(PaginationResponse {
                page: pagination.page.unwrap(),
                page_size: pagination.page_size.unwrap(),
                total: 2,
                elements: Vec::<Application>::new(),
            })
        });

        let service = ApplicationService::new_with_repo(Arc::new(mock_repo));

        let response = service
            .find_all(Pagination {
                page: Some(0),
                page_size: Some(10),
            })
            .await;
        assert_eq!(true, response.is_ok());
        assert_eq!(2, response.unwrap().total);
    }

    #[tokio::test]
    async fn find_all_without_page() {
        let service =
            ApplicationService::new_with_repo(Arc::new(MockApplicationRepositoryTrait::new()));

        let response = service
            .find_all(Pagination {
                page: None,
                page_size: None,
            })
            .await;
        assert_eq!(true, response.is_err());
        assert_eq!(PG_ERR_PAGE_REQUIRED.0, response.unwrap_err().code);
    }

    #[tokio::test]
    async fn find_all_without_page_size() {
        let service =
            ApplicationService::new_with_repo(Arc::new(MockApplicationRepositoryTrait::new()));

        let response = service
            .find_all(Pagination {
                page: Some(1),
                page_size: None,
            })
            .await;
        assert_eq!(true, response.is_err());
        assert_eq!(PG_ERR_PAGE_SIZE_REQUIRED.0, response.unwrap_err().code);
    }

    #[tokio::test]
    async fn find_by_id() {
        let mut mock_repo = MockApplicationRepositoryTrait::new();
        mock_repo.expect_find_by_id().returning(|_| {
            Ok(Some(Application {
                id: 1,
                name: String::from("Teste"),
                path: String::from("/teste"),
                url_destination: String::from("http://anothergtw.com"),
                created_dttm: Utc::now(),
                update_dttm: Utc::now(),
            }))
        });

        let service = ApplicationService::new_with_repo(Arc::new(mock_repo));

        let response = service.find_by_id(1).await;
        assert_eq!(true, response.is_ok());
        assert_eq!(1, response.ok().unwrap().id);
    }

    #[tokio::test]
    async fn find_by_id_not_found() {
        let mut mock_repo = MockApplicationRepositoryTrait::new();
        mock_repo.expect_find_by_id().returning(|_| {
            Ok(None)
        });

        let service = ApplicationService::new_with_repo(Arc::new(mock_repo));

        let response = service.find_by_id(1).await;
        assert_eq!(true, response.is_err());
        assert_eq!(APP_ERR_NOT_FOUND.0, response.unwrap_err().code);
    }

    #[tokio::test]
    async fn save() {
        let mut mock_repo = MockApplicationRepositoryTrait::new();
        mock_repo.expect_save().returning(|_| {
            Ok(Application {
                id: 1,
                name: String::from("Teste"),
                path: String::from("/teste"),
                url_destination: String::from("http://anothergtw.com"),
                created_dttm: Utc::now(),
                update_dttm: Utc::now(),
            })
        });

        let request = ApplicationReq {
            name: Some(StringMinSize3::from_str("teste").unwrap()),
            path: Some(StringMinSize3::from_str("/teste").unwrap()),
            url_destination: Some(StringMinSize3::from_str("http://anothergw.com").unwrap())
        };

        let service = ApplicationService::new_with_repo(Arc::new(mock_repo));

        let response = service.save(request).await;
        assert_eq!(true, response.is_ok());
        assert_eq!(1, response.unwrap().id);
    }

    #[tokio::test]
    async fn save_without_fields() {
        let mock_repo = MockApplicationRepositoryTrait::new();

        let request = ApplicationReq {
            name: None,
            path: None,
            url_destination: None
        };

        let service = ApplicationService::new_with_repo(Arc::new(mock_repo));

        let response = service.save(request).await;
        assert_eq!(true, response.is_err());

        let api_error = response.unwrap_err();
        assert_eq!(false, api_error.field_errors.is_none());
    }

    #[tokio::test]
    async fn save_without_min_size() {
        let mock_repo = MockApplicationRepositoryTrait::new();

        let request = ApplicationReq {
            name: Some(StringMinSize3::from_str("te").unwrap()),
            path: Some(StringMinSize3::from_str("/t").unwrap()),
            url_destination: Some(StringMinSize3::from_str("ht").unwrap())
        };

        let service = ApplicationService::new_with_repo(Arc::new(mock_repo));

        let response = service.save(request).await;
        assert_eq!(true, response.is_err());

        let api_error = response.unwrap_err();
        assert_eq!(true, api_error.field_errors.is_some());
        assert_eq!(3, api_error.field_errors.unwrap().len());
    }

    #[tokio::test]
    async fn save_with_repository_error() {
        let mut mock_repo = MockApplicationRepositoryTrait::new();
        mock_repo.expect_save().returning(|_| {
            Err(ApiError::new(APP_ERR_INSERTING))
        });

        let request = ApplicationReq {
            name: Some(StringMinSize3::from_str("teste").unwrap()),
            path: Some(StringMinSize3::from_str("/teste").unwrap()),
            url_destination: Some(StringMinSize3::from_str("http://anothergw.com").unwrap())
        };

        let service = ApplicationService::new_with_repo(Arc::new(mock_repo));

        let response = service.save(request).await;
        assert_eq!(true, response.is_err());
        assert_eq!(APP_ERR_INSERTING.0, response.unwrap_err().code);
    }
}
