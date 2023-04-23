// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryInto;

use wasmlib::*;

use crate::*;
use ed25519_dalek::*;
use base64;
use sha2::{Sha256, Digest};

pub fn func_create_declaration(ctx: &ScFuncContext, f: &CreateDeclarationContext) {
    // 1. Calculate the hash of the declaration
    let signor_hash = f.params.signor_hash().value();
    let object_hash = f.params.object_hash().value();
    let declaration_type = f.params.declaration_type().value();

    let all = [
        signor_hash.to_bytes(),
        object_hash.to_bytes(),
        declaration_type.as_bytes().to_vec(),
    ]
    .concat();

    let mut hasher = Sha256::new();
    hasher.update(all);
    let hash = hasher.finalize();
    let transaction_hash = hash_from_bytes(&hash);


    // 2. Check if the signature is valid
    // 2.1 Load signor identity claim
    let identity_claim = f.state.identity_claims().get_identity_claim(&signor_hash);
    ctx.require(identity_claim.exists(), "signor does not exist!");

    // Validate signature
    let public_key_bytes = hex::decode(identity_claim.value().public_key).unwrap();
    let verify_key = PublicKey::from_bytes(&public_key_bytes).unwrap();

    let signature_bytes = hex::decode(f.params.signature().value()).unwrap();
    let signature: Signature = Signature::from_bytes(&signature_bytes).unwrap();


    let bytes = [signor_hash.to_bytes(), object_hash.to_bytes(), declaration_type.as_bytes().to_vec()].concat();
    ctx.require(
        verify_key.verify(&bytes, &signature).is_ok(),
        "signature is not valid!",
    );

    // 3. Store the declaration in the state
    let declaration = Declaration {
        timestamp: ctx.timestamp() as i64,
        transaction_hash: transaction_hash,
        signor_hash: signor_hash,
        object_hash: object_hash,
        declaration_type: declaration_type,
        signature: f.params.signature().value(),
    };

    f.state
        .declarations()
        .get_declaration(&declaration.transaction_hash)
        .set_value(&declaration);

    // 4. Emit declaration create event
    f.events.dcreated(
        &declaration.declaration_type,
        &declaration.object_hash,
        &declaration.signature,
        &declaration.signor_hash,
        declaration.timestamp,
        &declaration.transaction_hash,
    );
}



pub fn func_create_identity_claim(ctx: &ScFuncContext, f: &CreateIdentityClaimContext) {
    //Calculate the hash of the identity claim
    let identifier = f.params.identifier().value();
    let validation_url = f.params.validation_url().value();
    let public_key_bytes = hex::decode(f.params.public_key().value()).unwrap();

    let all = [
        identifier.as_bytes(),
        validation_url.as_bytes(),
        &public_key_bytes,
    ]
    .concat();

    let mut hasher = Sha256::new();
    hasher.update(all);
    let hash = hasher.finalize();
    let transaction_hash = hash_from_bytes(&hash);

    // Create a new identity claim
    let identity_claim = IdentityClaim {
        timestamp: ctx.timestamp() as i64,
        transaction_hash: transaction_hash,
        identifier: f.params.identifier().value(),
        validation_url: f.params.validation_url().value(),
        public_key: f.params.public_key().value(),
        signature: f.params.signature().value(),
    };

    // Calculate the hash of the identity claim
    let identity_claim_hash = ctx.utility().hash_blake2b(identity_claim.to_bytes().as_slice());

    // Verify the signature
    if !ctx.utility().ed25519_valid_signature(&identity_claim.to_bytes(), identity_claim.public_key.as_bytes(), identity_claim.signature.as_bytes()) {
        ctx.log("Invalid signature");
        return;
    }

    // Store the identity claim in the state
    f.state.identity_claims().get_identity_claim(&identity_claim_hash).set_value(&identity_claim);


    // Emit an identity_claim_created event
    let gpevent = GraphOfTrustEvents {};
        gpevent.iccreated(
        &identity_claim.identifier,
        &identity_claim.public_key,
        &identity_claim.signature,
        identity_claim.timestamp,
        &identity_claim.transaction_hash,
        &identity_claim.validation_url,
    );
}

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.caller());

}


pub fn create_transaction_hash(all: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(all);
    let hash = hasher.finalize();
    hash.to_vec()
}

pub fn func_register_organization(_ctx: &ScFuncContext, _f: &RegisterOrganizationContext) {
    // To be implemented based on your specific requirements
}

pub fn func_register_sensor(_ctx: &ScFuncContext, _f: &RegisterSensorContext) {
    // To be implemented based on your specific requirements
}

pub fn func_set_identifier(_ctx: &ScFuncContext, _f: &SetIdentifierContext) {
    // To be implemented based on your specific requirements
}

pub fn func_set_owner(_ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.owner().value());
}

pub fn func_set_public_key(_ctx: &ScFuncContext, _f: &SetPublicKeyContext) {
    // To be implemented based on your specific requirements
}

pub fn func_set_signature(_ctx: &ScFuncContext, _f: &SetSignatureContext) {
    // To be implemented based on your specific requirements
}

pub fn func_set_validation_url(_ctx: &ScFuncContext, _f: &SetValidationUrlContext) {
    // To be implemented based on your specific requirements
}
/*
pub fn view_get_declarations(ctx: &ScViewContext, f: &GetDeclarationsContext) {
    let mut declarations: Vec<Declaration> = Vec::new();
    let iter = f.state.declarations().iter();
    for declaration_kv in iter {
        let declaration = declaration_kv.1;

        declarations.push(declaration.to_declaration());
    }
    f.results.declarations().set(&declarations);

}
*/


pub fn view_get_declarations(_ctx: &ScViewContext, f: &GetDeclarationsContext) {
    let mut declarations: Vec<Declaration> = Vec::new();
    
    let declarations_dict = f.state.declarations(); // Create a longer-lived binding
    let iter = declarations_dict.iter(); // Now it's borrowing from `declarations_dict`
    
    for declaration_kv in iter {
        let declaration = declaration_kv.1;
        declarations.push(declaration.to_declaration())
    }

    let mutable_declarations = f.results.declarations();
    for (i, declaration) in declarations.iter().enumerate() {
        let index = i as u32;
        if index < mutable_declarations.length() {
            mutable_declarations.proxy.index(index).set(&declaration.to_bytes());
        } else {
            // Error handling or logging can be added here
        }
    }
}





pub fn view_get_graph_of_trust_info(_ctx: &ScViewContext, f: &GetGraphOfTrustInfoContext) {
    let info = "The Graph of Trust is a distributed public key infrastructure";
    f.results.info().set_value(info);
}

pub fn view_get_identity_claims(_ctx: &ScViewContext, f: &GetIdentityClaimsContext) {
    let mut identity_claims: Vec<IdentityClaim> = Vec::new();
    let identity_claims_dict = f.state.identity_claims();
    let iter = identity_claims_dict.proxy.sc_dict().iter_as_vec();
    for identity_claim_kv in iter {
        let identity_claim_bytes = identity_claim_kv.1;
        let identity_claim = IdentityClaim::from_bytes(&identity_claim_bytes);
        identity_claims.push(identity_claim);
    }

    let mutable_identity_claims = f.results.identity_claims();
    for (i, identity_claim) in identity_claims.iter().enumerate() {
        let index = i as i32;
        if index < mutable_identity_claims.length() {
            let mutable_identity_claim = mutable_identity_claims.get_identity_claim(index);
            mutable_identity_claim.set_value(identity_claim);
        } else {
            // Error handling or logging can be added here
        }
    }
}






pub fn view_get_owner(_ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}

pub fn view_get_registered_organizations(_ctx: &ScViewContext, _f: &GetRegisteredOrganizationsContext) {
    // To be implemented based on your specific requirements
}

pub fn view_get_registered_sensors(_ctx: &ScViewContext, _f: &GetRegisteredSensorsContext) {
    // To be implemented based on your specific requirements
}

pub fn view_get_signature_info(_ctx: &ScViewContext, _f: &GetSignatureInfoContext) {
    // To be implemented based on your specific requirements
}


pub fn func_store_public_key(_ctx: &ScFuncContext, f: &StorePublicKeyContext) {
    let org_id = f.params.org_id().value();
    let public_key = f.params.public_key().value();
    f.state.public_keys().get_string(&org_id).set_value(&public_key);
}

pub fn func_verify_signature(_ctx: &ScFuncContext, f: &VerifySignatureContext) {
    let org_id = f.params.org_id().value();
    let _sensor_id = f.params.sensor_id().value();
    let data = f.params.data().value();
    let signature_bytes = f.params.signature().value();

    // Retrieve the public key for the organization
    let public_key_str = f.state.public_keys().get_string(&org_id).value();

    if public_key_str.is_empty() {
        f.results.is_valid().set_value(false);
        return;
    }

    // Convert the public key string to a PublicKey object
    let public_key_bytes = match base64::decode(&public_key_str) {
        Ok(bytes) => {
            bytes
        },
        Err(_) => {
            f.results.is_valid().set_value(false);
            return;
        }
    };
    let public_key = match PublicKey::from_bytes(&public_key_bytes) {
        Ok(key) => key,
        Err(_) => {
            f.results.is_valid().set_value(false);
            return;
        }
    };

    // Decode the signature_bytes String to a Vec<u8>
    let decoded_signature_bytes = match base64::decode(&signature_bytes) {
        Ok(bytes) => bytes,
        Err(_) => {
            f.results.is_valid().set_value(false);
            return;
        }
    };

    // Convert the decoded signature bytes to a Signature object
    let signature = match Signature::from_bytes(&decoded_signature_bytes) {
        Ok(sig) => sig,
        Err(_) => {
            f.results.is_valid().set_value(false);
            return;
        }
    };

    // Verify the signature
    let is_valid = public_key.verify(data.as_bytes(), &signature).is_ok();

    // Set the result
    f.results.is_valid().set_value(is_valid);
}
