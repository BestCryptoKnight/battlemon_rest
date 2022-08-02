// use actix_web::body::MessageBody;
// use actix_web::dev::{ServiceRequest, ServiceResponse};
// use actix_web_lab::middleware::Next;
//
// async fn my_middleware(
//     req: ServiceRequest,
//     next: Next<impl MessageBody>,
// ) -> Result<ServiceResponse<impl MessageBody>, Error> {
//     let response = next.call(req).await;
// }
//
