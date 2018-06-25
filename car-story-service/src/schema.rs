use exonum::crypto::PublicKey;
use exonum::storage::{Fork, MapIndex};
use structs::{Car, Owner};

/*------------------------------------------------ Cars section -----------------------------------------------------------*/
pub struct CarsSchema<'a> {
    pub view: &'a mut Fork,
}

impl<'a> CarsSchema<'a> {
    pub fn cars(&mut self) -> MapIndex<&mut Fork, String, Car> {
        MapIndex::new("cars", self.view)
    }

    pub fn car(&mut self, vin: &str) -> Option<Car> {
        self.cars().get(&vin.to_string())
    }
}

/*------------------------------------------------ Owners section -----------------------------------------------------------*/
pub struct OwnersSchema<'a> {
    pub view: &'a mut Fork,
}

impl<'a> OwnersSchema<'a> {
    pub fn owners(&mut self) -> MapIndex<&mut Fork, String, Owner> {
        MapIndex::new("owners", self.view)
    }

    pub fn owner(&mut self, public_key: &PublicKey) -> Option<Owner> {
        self.owners().get(&public_key.to_string())
    }
}



