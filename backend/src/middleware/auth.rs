use crate::messages::error::ErrorResponse;
use actix_service::{Service, Transform};
use actix_session::UserSession;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error::ResponseError,
    Error,
};
use futures::{
    future::{self, Either, FutureResult},
    Future, Poll,
};

pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<
        Box<Future<Item = Self::Response, Error = Self::Error>>,
        FutureResult<Self::Response, Self::Error>,
    >;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, mut request: ServiceRequest) -> Self::Future {
        println!("Hello from auth layer! Request: {}", request.path());

        let username = request
            .get_session()
            .get::<String>("username")
            .unwrap_or(None);

        println!("ALERT, USER IS {:?}", username);

        if let Some(username) = username {
            if username != "Anonymous" {
                return Either::A(Box::new(self.service.call(request).and_then(|response| {
                    log::info!("RESPONSE");

                    Ok(response)
                })));
            }
        }

        Either::B(future::ok(request.into_response(
            ErrorResponse::Unauthorised.error_response().into_body(),
        )))
    }
}