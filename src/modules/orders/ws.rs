// use actix::{Actor, ActorContext, AsyncContext, StreamHandler};

// use actix_web::{Error, HttpRequest, HttpResponse, web};

// use actix_web_actors::ws;

// use tokio::sync::broadcast;
// use tokio_stream::wrappers::{BroadcastStream, errors::BroadcastStreamRecvError};

// use super::service::OrderEvent;

// pub struct OrderWs {
//   receiver: Option<broadcast::Receiver<OrderEvent>>,
// }

// impl OrderWs {
//   pub fn new(receiver: broadcast::Receiver<OrderEvent>) -> Self {
//     Self {
//       receiver: Some(receiver),
//     }
//   }
// }

// impl Actor for OrderWs {
//   type Context = ws::WebsocketContext<Self>;

//   fn started(&mut self, ctx: &mut Self::Context) {
//     if let Some(receiver) = self.receiver.take() {
//       ctx.add_stream(BroadcastStream::new(receiver));
//     }
//   }
// }

// // Handle messages received from the broadcast channel.
// impl StreamHandler<Result<OrderEvent, BroadcastStreamRecvError>> for OrderWs {
//   fn handle(
//     &mut self,
//     event: Result<OrderEvent, BroadcastStreamRecvError>,
//     ctx: &mut Self::Context,
//   ) {
//     match event {
//       Ok(event) => {
//         let message = serde_json::json!({
//             "event": event.event,
//             "data": event.data,
//         });

//         ctx.text(message.to_string());
//       }

//       Err(BroadcastStreamRecvError::Lagged(_)) => {
//         // The client fell behind and missed messages.
//         // It should refresh its order data through REST.
//       }
//     }
//   }
// }

// // Handle messages received directly from the WebSocket client.
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for OrderWs {
//   fn handle(&mut self, message: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//     match message {
//       Ok(ws::Message::Ping(message)) => {
//         ctx.pong(&message);
//       }

//       Ok(ws::Message::Close(reason)) => {
//         ctx.close(reason);
//         ctx.stop();
//       }

//       Ok(ws::Message::Text(_)) => {
//         // This connection currently receives order events.
//         // Client commands can be implemented separately.
//       }

//       Ok(ws::Message::Pong(_))
//       | Ok(ws::Message::Binary(_))
//       | Ok(ws::Message::Continuation(_))
//       | Ok(ws::Message::Nop) => {}

//       Err(_) => {
//         ctx.stop();
//       }
//     }
//   }
// }

// // WebSocket endpoint.
// pub async fn order_ws(
//   req: HttpRequest,
//   stream: web::Payload,
//   state: web::Data<crate::state::AppState>,
// ) -> Result<HttpResponse, Error> {
//   let receiver = state.orders.subscribe();

//   ws::start(OrderWs::new(receiver), &req, stream)
// }
