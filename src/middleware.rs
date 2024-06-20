use actix_web::{Error, HttpRequest, HttpResponse};
use actix_service::{Service, Transform};
use futures::future::{ok, Ready};
use futures::Future;
use jsonwebtoken::{decode, Validation, DecodingKey};
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub struct JwtMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddleware { service })
    }
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization").and_then(|h| h.to_str().ok());

        if let Some(token) = token {
            let token_data = decode::<Claims>(&token, &DecodingKey::from_secret("your_jwt_secret".as_ref()), &Validation::default());

            if let Ok(data) = token_data {
                req.extensions_mut().insert(data.claims);
                return Box::pin(self.service.call(req));
            }
        }

        Box::pin(async { Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body())) })
    }
}
