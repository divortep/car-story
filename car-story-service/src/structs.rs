use exonum::crypto::PublicKey;

/*------------------------------------------------ Car section -----------------------------------------------------------*/
encoding_struct! {
    struct Car {
        const SIZE = 72;

        field vin:         &str        [00 => 08]
        field brand:       &str        [08 => 16]
        field model:       &str        [16 => 24]
        field color:       &str        [24 => 32]
        field year:        u8          [32 => 40]
        field mileage:     u16         [40 => 56]
        field owner:       &PublicKey  [56 => 72]
    }
}

impl Car {
    pub fn change_owner(self, new_owner: &PublicKey) -> Self {
        Self::new(
            self.vin(),
            self.brand(),
            self.model(),
            self.color(),
            self.year(),
            self.mileage(),
            new_owner)
    }
}

/*------------------------------------------------ Owner section -----------------------------------------------------------*/
encoding_struct! {
    struct Owner {
        const SIZE = 40;

        field name:        &str        [00 => 08]
        field public_key:  &PublicKey  [08 => 40]
    }
}