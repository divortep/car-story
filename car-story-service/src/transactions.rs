use consts::{CARS_SERVICE_ID, TX_PRODUCE_CAR_ID, TX_CHANGE_CAR_OWNER_ID, TX_REGISTER_OWNER_ID};
use exonum::messages::Message;
use exonum::blockchain::Transaction;
use exonum::storage::Fork;
use exonum::crypto::{PublicKey, verify};
use structs::{Car, Owner};
use schema::{CarsSchema, OwnersSchema};

/*------------------------------------------------ Cars section -----------------------------------------------------------*/
message! {
    struct TxProduceCar {
        const TYPE = CARS_SERVICE_ID;
        const ID = TX_PRODUCE_CAR_ID;
        const SIZE = 104;

        field vin:              &str        [00 => 08]
        field brand:            &str        [08 => 16]
        field model:            &str        [16 => 24]
        field color:            &str        [24 => 32]
        field year:             u8          [32 => 40]
        field mileage:          u16         [40 => 56]
        field factory_pub_key:  &PublicKey  [56 => 88]
    }
}

impl Transaction for TxProduceCar {
    fn verify(&self) -> bool {
        self.verify_signature(self.factory_pub_key())
    }

    fn execute(&self, view: &mut Fork) {
        let mut schema = CarsSchema { view };
        let vin = self.vin();
        if schema.car(vin).is_none() {
            let car = Car::new(
                self.vin(),
                self.brand(),
                self.model(),
                self.color(),
                self.year(),
                self.mileage(),
                self.factory_pub_key());

            println!("Created an car: {:?}", &car);
            schema.cars().put(&String::from(vin), car);

        } else {
            println!("Car with a vin {} already exists", &vin);
        }
    }
}

message! {
    struct TxChangeCarOwner {
        const TYPE = CARS_SERVICE_ID;
        const ID = TX_CHANGE_CAR_OWNER_ID;
        const SIZE = 80;

        field owner_pub_key:        &PublicKey  [00 => 32]
        field new_owner_pub_key:    &PublicKey  [32 => 64]
        field vin:                  &str        [64 => 72]
        field seed:                 u64         [72 => 80]
    }
}

impl Transaction for TxChangeCarOwner {
    fn verify(&self) -> bool {
        self.verify_signature(self.owner_pub_key())
    }

    fn execute(&self, view: &mut Fork) {
        let mut schema = CarsSchema { view };
        let vin = self.vin();
        let car = schema.car(vin);
        if car.is_some() {
            schema.cars().put(&String::from(vin), car.unwrap().change_owner(self.new_owner_pub_key()));
            println!("Car with vin {} changed its owner", &vin);
        } else {
            println!("Car with vin {} doesn't exist", &vin);
        }
    }
}

/*------------------------------------------------ Owners section -----------------------------------------------------------*/

message! {
    struct TxRegisterOwner {
        const TYPE = CARS_SERVICE_ID;
        const ID = TX_REGISTER_OWNER_ID;
        const SIZE = 40;

        field name:             &str        [00 => 08]
        field pub_key:          &PublicKey  [08 => 40]
    }
}

impl Transaction for TxRegisterOwner {
    fn verify(&self) -> bool {
        self.verify_signature(self.pub_key())
    }

    fn execute(&self, view: &mut Fork) {
        let mut schema = OwnersSchema { view };
        let pub_key = self.pub_key();
        if schema.owner(pub_key).is_none() {
            let owner = Owner::new(
                self.name(),
                self.pub_key());

            println!("Created an owner: {:?}", &owner);
            schema.owners().put(&pub_key.to_string(), owner);

        } else {
            println!("Owner with a public key {:?} already exists", &pub_key);
        }
    }
}
