use exonum::crypto::{PublicKey, Hash};
use exonum::blockchain::{Blockchain, Transaction};
use exonum::node::{TransactionSend, ApiSender};
use exonum::api::{Api, ApiError};
use transactions::{TxProduceCar, TxChangeCarOwner, TxRegisterOwner};
use iron::prelude::*;
use router::Router;
use bodyparser;
use serde_json;
use structs::{Car, Owner};
use schema::{CarsSchema, OwnersSchema};


/*------------------------------------------------ Cars section -----------------------------------------------------------*/
#[serde(untagged)]
#[derive(Clone, Serialize, Deserialize)]
enum CarTransactionRequest {
    ProduceCar(TxProduceCar),
    ChangeCarOwner(TxChangeCarOwner),
}

#[derive(Serialize, Deserialize)]
struct TransactionResponse {
    tx_hash: Hash,
}

#[derive(Clone)]
pub struct CarsApi {
    pub channel: ApiSender,
    pub blockchain: Blockchain,
}

impl CarsApi {
    fn get_car(&self, vin: &str) -> Option<Car> {
        let mut view = self.blockchain.fork();
        let mut schema = CarsSchema { view: &mut view };
        schema.car(vin)
    }

    fn get_cars(&self) -> Option<Vec<Car>> {
        let mut view = self.blockchain.fork();
        let mut schema = CarsSchema { view: &mut view };
        let idx = schema.cars();
        let cars: Vec<Car> = idx.values().collect();
        if cars.is_empty() {
            None
        } else {
            Some(cars)
        }
    }
}

impl Into<Box<Transaction>> for CarTransactionRequest {
    fn into(self) -> Box<Transaction> {
        match self {
            CarTransactionRequest::ProduceCar(trans) => Box::new(trans),
            CarTransactionRequest::ChangeCarOwner(trans) => Box::new(trans),
        }
    }
}

impl Api for CarsApi {
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();

        let tx_handler = move |req: &mut Request| -> IronResult<Response> {
            match req.get::<bodyparser::Struct<CarTransactionRequest>>() {
                Ok(Some(tx)) => {
                    let tx: Box<Transaction> = tx.into();
                    let tx_hash = tx.hash();
                    self_.channel.send(tx).map_err(|e| ApiError::IncorrectRequest(Box::new(e)))?;
                    let json = TransactionResponse { tx_hash };
                    self_.ok_response(&serde_json::to_value(&json).unwrap())
                }
                Ok(None) => Err(ApiError::IncorrectRequest("Empty request body".into()))?,
                Err(e) => Err(ApiError::IncorrectRequest(Box::new(e)))?,
            }
        };

        // Gets status of all cars in the database.
        let self_ = self.clone();
        let cars_info = move |_: &mut Request| -> IronResult<Response> {
            if let Some(cars) = self_.get_cars() {
                self_.ok_response(&serde_json::to_value(cars).unwrap())
            } else {
                self_.not_found_response(&serde_json::to_value("Cars database is empty").unwrap())
            }
        };

        // Gets status of the car with the provided vin.
        let self_ = self.clone();
        let car_info = move |req: &mut Request| -> IronResult<Response> {
            let path = req.url.path();
            let vin = path.last().unwrap();

            if let Some(car) = self_.get_car(&vin) {
                self_.ok_response(&serde_json::to_value(car).unwrap())
            } else {
                self_.not_found_response(&serde_json::to_value("Car not found").unwrap())
            }
        };

        router.post("/v1/cars/transaction", tx_handler, "cars_transaction");
        router.get("/v1/cars", cars_info, "cars_info");
        router.get("/v1/car/:vin", car_info, "car_info");
    }
}

/*------------------------------------------------ Owners section -----------------------------------------------------------*/
#[serde(untagged)]
#[derive(Clone, Serialize, Deserialize)]
enum OwnerTransactionRequest {
    RegisterOwner(TxRegisterOwner)
}

#[derive(Clone)]
pub struct OwnersApi {
    pub channel: ApiSender,
    pub blockchain: Blockchain,
}

impl OwnersApi {
    fn get_owner(&self, public_key: PublicKey) -> Option<Owner> {
        let mut view = self.blockchain.fork();
        let mut schema = OwnersSchema { view: &mut view };
        schema.owner(&public_key)
    }
}

impl Into<Box<Transaction>> for OwnerTransactionRequest {
    fn into(self) -> Box<Transaction> {
        match self {
            OwnerTransactionRequest::RegisterOwner(trans) => Box::new(trans)
        }
    }
}

impl Api for OwnersApi {
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();

        let tx_handler = move |req: &mut Request| -> IronResult<Response> {
            match req.get::<bodyparser::Struct<OwnerTransactionRequest>>() {
                Ok(Some(tx)) => {
                    let tx: Box<Transaction> = tx.into();
                    let tx_hash = tx.hash();
                    self_.channel.send(tx).map_err(|e| ApiError::IncorrectRequest(Box::new(e)))?;
                    let json = TransactionResponse { tx_hash };
                    self_.ok_response(&serde_json::to_value(&json).unwrap())
                }
                Ok(None) => Err(ApiError::IncorrectRequest("Empty request body".into()))?,
                Err(e) => Err(ApiError::IncorrectRequest(Box::new(e)))?,
            }
        };

        router.post("/v1/owners/transaction", tx_handler, "owners_transaction");
    }
}