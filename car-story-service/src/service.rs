use consts::{CARS_SERVICE_ID, CARS_SERVICE_NAME, TX_PRODUCE_CAR_ID, TX_CHANGE_CAR_OWNER_ID, TX_REGISTER_OWNER_ID};
use iron::Handler;
use router::Router;
use exonum::messages::FromRaw;
use exonum::api::Api;
use exonum::blockchain::{Service, ApiContext};
use exonum::messages::RawTransaction;
use exonum::blockchain::Transaction;
use exonum::encoding;
use transactions::{TxProduceCar, TxChangeCarOwner, TxRegisterOwner};
use api::{CarsApi, OwnersApi};

pub struct CarsService;

impl Service for CarsService {
    fn service_name(&self) -> &'static str { CARS_SERVICE_NAME }

    fn service_id(&self) -> u16 { CARS_SERVICE_ID }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let trans: Box<Transaction> = match raw.message_type() {
            TX_PRODUCE_CAR_ID => Box::new(TxProduceCar::from_raw(raw)?),
            TX_CHANGE_CAR_OWNER_ID => Box::new(TxChangeCarOwner::from_raw(raw)?),
            TX_REGISTER_OWNER_ID => Box::new(TxRegisterOwner::from_raw(raw)?),
            _ => {
                return Err(encoding::Error::IncorrectMessageType {
                    message_type: raw.message_type()
                });
            }
        };
        Ok(trans)
    }

    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let cars_api = CarsApi {
            channel: ctx.node_channel().clone(),
            blockchain: ctx.blockchain().clone()
        };
        cars_api.wire(&mut router);

        let owners_api = OwnersApi {
            channel: ctx.node_channel().clone(),
            blockchain: ctx.blockchain().clone()
        };
        owners_api.wire(&mut router);

        Some(Box::new(router))
    }
}