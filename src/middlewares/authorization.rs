use std::cell::RefCell;
use std::future::{Future, ready, Ready};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use actix_web::{Error};
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::HeaderValue;
use log::{error, info};
use crate::errors::errors::ErrorResponse;
use crate::repos::Repository;

pub(crate) struct Authorization<'a> {
    pub repo: Arc<Repository<'a>>,
}

impl<'a> Authorization<'a> {
    pub fn new(repo: Arc<Repository<'a>>) -> Self {
        Authorization {
            repo
        }
    }
}

impl<'a, S, B> Transform<S, ServiceRequest> for Authorization<'a>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthorizationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware {
            service: Arc::new(RefCell::new(service)),
        }))
    }
}

pub struct AuthorizationMiddleware<S> {
    service: Arc<RefCell<S>>,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
    where
        S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        Box::pin(async move {
            let value = HeaderValue::from_str("")?;
            let value = req.headers().get("Authorization").unwrap_or(&value);
            info!("Authorization: {} - {:?}", req.path(), value);

            let token = value.to_str().unwrap();

            if token.starts_with("Bearer") {
                Ok(svc.call(req).await?)
            } else {
                error!("Authorization: check error");
                Err(Error::from(ErrorResponse::AUTHORIZATION_ERROR))
            }
        })
    }
}