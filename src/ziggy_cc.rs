use starkware::statement::ziggy::ziggy_statement::ZiggyStatement;
//#include <algorithm>
//#include <utility>
use starkware::algebra::field_operations;
use starkware::utils::json_builder;
mod starkware {
    
    pub struct ZiggyStatement {
        public_input: JsonValue,
        private_input: Option<JsonValue>,
    }

    impl ZiggyStatement {
        pub fn new(public_input: JsonValue, private_input: Option<JsonValue>) -> Self {
            let public_key = (
                public_input["public_key"][0].as_field_element::<BaseFieldElement>(),
                public_input["public_key"][1].as_field_element::<BaseFieldElement>(),
                public_input["public_key"][2].as_field_element::<BaseFieldElement>(),
                public_input["public_key"][3].as_field_element::<BaseFieldElement>(),
            );
            let message = public_input["message"].as_str();
            let mut private_key = None;
            let mut secret_preimage = None;
            if let Some(private_input) = private_input {
                let mut private_key_t = PrivateKeyT::default();
                private_input["private_key"].as_bytes_from_hex_string(&mut private_key_t);
                private_key = Some(private_key_t);
                let mut secret_preimage_prng = Prng::new(get_secret_preimage_seed());
                let secret_preimage_t = SecretPreimageT {
                    elements: [
                        BaseFieldElement::random_element(&mut secret_preimage_prng),
                        BaseFieldElement::random_element(&mut secret_preimage_prng),
                        BaseFieldElement::random_element(&mut secret_preimage_prng),
                        BaseFieldElement::random_element(&mut secret_preimage_prng),
                        BaseFieldElement::random_element(&mut secret_preimage_prng),
                        BaseFieldElement::random_element(&mut secret_preimage_prng),
                        BaseFieldElement::random_element(&mut secret_preimage_prng),
                        BaseFieldElement::random_element(&mut secret_preimage_prng),
                    ],
                };
                secret_preimage = Some(secret_preimage_t);
            }
            ZiggyStatement { public_input, private_input, public_key, message, private_key, secret_preimage }
        }
    }

    pub fn get_air(&mut self, is_zero_knowledge: bool, n_queries: usize) -> &Air {
        assert!(is_zero_knowledge, "Ziggy proof must be zero knowledge.");
        self.air = Some(ZiggyAir::new(self.public_key.clone(), is_zero_knowledge, n_queries));
        //self.air.as_ref().unwrap()
    }

    pub fn get_initial_hash_chain_seed(&self) -> Vec<u8> {
        let ziggy_string = b"Ziggy\x00";
        let public_key_element_bytes = BaseFieldElement::size_in_bytes();
        let mut randomness_seed = vec![0; ziggy_string.len() + public_key_element_bytes * ZiggyAir::k_word_size() + self.message.len()];
        
        for (i, c) in ziggy_string.iter().enumerate() {
            randomness_seed[i] = *c;
        }
        
        for (i, val) in self.public_key.iter().enumerate() {
            let bytes = val.to_bytes();
            let start = ziggy_string.len() + i * public_key_element_bytes;
            let end = start + public_key_element_bytes;
            randomness_seed[start..end].copy_from_slice(&bytes);
        }
        
        for (i, c) in self.message.bytes().enumerate() {
            let index = ziggy_string.len() + public_key_element_bytes * ZiggyAir::k_word_size() + i;
            randomness_seed[index] = c;
        }
        
        randomness_seed
    }


    pub fn get_zero_knowledge_hash_chain_seed(&self) -> Vec<u8> {
        let private_string = b"Ziggy private seed\x00";
        let private_key_bytes = self.private_key.as_ref().unwrap().to_bytes();
        let mut randomness_seed = vec![0; private_string.len() + private_key_bytes.len() + self.message.len()];
        
        for (i, &c) in private_string.iter().enumerate() {
            randomness_seed[i] = c;
        }
        
        for (i, &val) in private_key_bytes.iter().enumerate() {
            let index = private_string.len() + i;
            randomness_seed[index] = val;
        }
        
        for (i, c) in self.message.bytes().enumerate() {
            let index = private_string.len() + private_key_bytes.len() + i;
            randomness_seed[index] = c;
        }
        
        randomness_seed
    }

    

    
        
    
    pub fn get_trace(&self, prng: &mut Prng) -> Trace {
        assert!(self.air.is_some(), "Cannot construct trace without a fully initialized AIR instance. Please call get_air() prior to get_trace().");
        assert!(self.secret_preimage.is_some(), "secret_preimage_ must have a value.");
        assert!(prng.is_some(), "prng should not be null when using zero knowledge.");
        self.air.unwrap().get_trace(self.secret_preimage.unwrap(), prng)
    }

    pub fn fix_public_input(&mut self) -> JsonValue {
        let mut root = JsonBuilder::new();
        self.public_key = ZiggyAir::public_input_from_private_input(self.secret_preimage.as_ref().unwrap().clone());
        for element in &self.public_key {
            root["public_key"].append(element.to_string());
        }
        root["message"] = self.message.clone();
        root.build()
    }
    
    
    
    
    
}

        

