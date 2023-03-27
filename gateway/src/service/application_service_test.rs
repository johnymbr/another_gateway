use std::str::FromStr;

use chrono::Utc;

use crate::{
    exception::{APP_ERR_INSERTING, PG_ERR_PAGE_REQUIRED, PG_ERR_PAGE_SIZE_REQUIRED, ERR_INVALID_REQUEST, APP_ERR_UPDATING, APP_ERR_DELETE},
    model::StringMinSize3,
    repository::MockApplicationRepositoryTrait,
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
    mock_repo.expect_find_by_id().returning(|_| Ok(None));

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
        url_destination: Some(StringMinSize3::from_str("http://anothergw.com").unwrap()),
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
        url_destination: None,
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
        url_destination: Some(StringMinSize3::from_str("ht").unwrap()),
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
    mock_repo
        .expect_save()
        .returning(|_| Err(ApiError::new(APP_ERR_INSERTING)));

    let request = ApplicationReq {
        name: Some(StringMinSize3::from_str("teste").unwrap()),
        path: Some(StringMinSize3::from_str("/teste").unwrap()),
        url_destination: Some(StringMinSize3::from_str("http://anothergw.com").unwrap()),
    };

    let service = ApplicationService::new_with_repo(Arc::new(mock_repo));

    let response = service.save(request).await;
    assert_eq!(true, response.is_err());
    assert_eq!(APP_ERR_INSERTING.0, response.unwrap_err().code);
}

#[tokio::test]
async fn update() {
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

    mock_repo.expect_update().returning(|_| {
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
        url_destination: Some(StringMinSize3::from_str("http://anothergw.com").unwrap()),
    };

    let service = ApplicationService::new_with_repo(Arc::new(mock_repo));

    let response = service.update(1, request).await;
    assert_eq!(true, response.is_ok());
    assert_eq!(1, response.unwrap().id);
}

#[tokio::test]
async fn update_application_not_found() {
    let mut mock_repo = MockApplicationRepositoryTrait::new();
    mock_repo.expect_find_by_id().returning(|_| {
        Ok(None)
    });

    let request = ApplicationReq {
        name: Some(StringMinSize3::from_str("teste").unwrap()),
        path: Some(StringMinSize3::from_str("/teste").unwrap()),
        url_destination: Some(StringMinSize3::from_str("http://anothergw.com").unwrap()),
    };

    let service = ApplicationService::new_with_repo(Arc::new(mock_repo));

    let response = service.update(1, request).await;
    assert_eq!(true, response.is_err());
    assert_eq!(APP_ERR_NOT_FOUND.0, response.unwrap_err().code);
}

#[tokio::test]
async fn update_with_field_errors() {
    let request = ApplicationReq {
        name: Some(StringMinSize3::from_str("te").unwrap()),
        path: Some(StringMinSize3::from_str("/teste").unwrap()),
        url_destination: Some(StringMinSize3::from_str("http://anothergw.com").unwrap()),
    };

    let service = ApplicationService::new_with_repo(Arc::new(MockApplicationRepositoryTrait::new()));

    let response = service.update(1, request).await;
    assert_eq!(true, response.is_err());

    let api_error = response.unwrap_err();
    assert_eq!(ERR_INVALID_REQUEST.0, api_error.code);
    assert_eq!(true, api_error.field_errors.is_some());
    assert_eq!(1, api_error.field_errors.unwrap().len());
}

#[tokio::test]
async fn update_repository_error() {
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

    mock_repo
        .expect_update()
        .returning(|_| Err(ApiError::new(APP_ERR_UPDATING)));

    let request = ApplicationReq {
        name: Some(StringMinSize3::from_str("teste").unwrap()),
        path: Some(StringMinSize3::from_str("/teste").unwrap()),
        url_destination: Some(StringMinSize3::from_str("http://anothergw.com").unwrap()),
    };

    let service = ApplicationService::new_with_repo(Arc::new(mock_repo));

    let response = service.update(1, request).await;
    assert_eq!(true, response.is_err());
    assert_eq!(APP_ERR_UPDATING.0, response.unwrap_err().code);
}

#[tokio::test]
async fn delete() {
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

    mock_repo.expect_delete().returning(|_| Ok(()));

    let service = ApplicationService::new_with_repo(Arc::new(mock_repo));
    let response = service.delete(1).await;
    assert_eq!(true, response.is_ok());
}

#[tokio::test]
async fn delete_application_not_found() {
    let mut mock_repo = MockApplicationRepositoryTrait::new();
    mock_repo.expect_find_by_id().returning(|_| {
        Ok(None)
    });

    let service = ApplicationService::new_with_repo(Arc::new(mock_repo));
    let response = service.delete(1).await;
    assert_eq!(true, response.is_err());
    assert_eq!(APP_ERR_NOT_FOUND.0, response.unwrap_err().code);
}

#[tokio::test]
async fn delete_with_repository_error() {
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

    mock_repo
        .expect_delete()
        .returning(|_| Err(ApiError::new(APP_ERR_DELETE)));

    let service = ApplicationService::new_with_repo(Arc::new(mock_repo));
    let response = service.delete(1).await;
    assert_eq!(true, response.is_err());
    assert_eq!(APP_ERR_DELETE.0, response.unwrap_err().code);
}