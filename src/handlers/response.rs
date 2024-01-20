use actix_web::body::{BoxBody};
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Response<T>
    where
        T: Serialize,
{
    pub code: u32,
    pub data: T,
}


impl<T> Response<T>
    where T: Serialize
{
    pub fn ok(t: T) -> Self {
        Response {
            code: 0,
            data: t,
        }
    }
}

impl<T: Serialize> Responder for Response<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .status(StatusCode::OK)
            .body(body)
    }
}

//
// impl<T> Future for Response<T>
//     where
//         T: Serialize,
// {
//     type Output = actix_web::Result<ServiceResponse<Pin<Box<Response<T>>>>, Error>;
//
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let this = self.project();
//
//         let res = match ready!(this.fut.poll(cx)) {
//             Ok(res) => res,
//             Err(err) => return Poll::Ready(Err(err)),
//         };
//
//         if let Some(error) = res.response().error() {
//             debug!("Error in response: {:?}", error);
//         }
//
//         let res = if let Some(ref mut format) = this.format {
//             // to avoid polluting all the Logger types with the body parameter we swap the body
//             // out temporarily since it's not usable in custom response functions anyway
//
//             let (req, res) = res.into_parts();
//             let (res, body) = res.into_parts();
//
//             let temp_res = ServiceResponse::new(req, res.map_into_boxed_body());
//
//             for unit in &mut format.0 {
//                 unit.render_response(&temp_res);
//             }
//
//             // re-construct original service response
//             let (req, res) = temp_res.into_parts();
//             ServiceResponse::new(req, res.set_body(body))
//         } else {
//             res
//         };
//
//         let resp = Response::ok(ErrorResponse::INTERNAL_ERROR);
//
//         Poll::Ready(Ok(res.map_body(move |_, body| Box::pin(resp))))
//     }
// }


