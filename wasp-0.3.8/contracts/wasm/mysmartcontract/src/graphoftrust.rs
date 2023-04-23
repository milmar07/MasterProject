// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::structs::*;
use crate::*;
use hex;

use core::num::*;
use p256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};
use std::convert::TryInto;
use std::{any::Any, env};

// ********************** DECLARATION *****************************
// ****************************************************************
pub fn func_create_declaration(_ctx: &ScFuncContext, f: &CreateDeclarationContext) {
    // 1. calculate the hash of the declaration
    let signor_hash = f.params.signor_hash().value();
    let object_hash = f.params.object_hash().value();
    let declaration_type = f.params.declaration_type().value();

    let all = [signor_hash.to_bytes(), object_hash.to_bytes(), declaration_type.as_bytes().to_vec()].join("".as_bytes()); //declaration_type.as_bytes() has missmatch in datatype, only working when adding to_vec();
    let mut hasher = Sha256::new();
    hasher.update(all);
    let hash = hasher.finalize();
    //let split_hash: &[u8] = hash.as_slice().try_into().expect("Wrong length");
    let transaction_hash = hash_from_bytes(hash.as_slice());

    //2. check if the signature is valid
    //2.1 load signor identity claim

    let identity_claim = f.state.identity_claims().get_identity_claim(&signor_hash);
    _ctx.require(identity_claim.exists(), "signor does not exist!");

    //validate signature
    let public_key_bytes = hex::decode(identity_claim.value().public_key).unwrap();
    let verify_key = VerifyingKey::from_sec1_bytes(&&public_key_bytes).unwrap();

    let signature_bytes = hex::decode(f.params.signature().value()).unwrap();
    let signature: Signature = Signature::from_der(&signature_bytes).unwrap();

    let bytes: &[u8] = &[
        signor_hash.to_bytes(),
        object_hash.to_bytes(),
        declaration_type.as_bytes().to_vec(),
    ]
    .join("".as_bytes()); //same story as in top
    _ctx.require(
        verify_key.verify(bytes, &signature).is_ok(),
        "signature is not valid!",
    );

    //3. store the declaration in the state
    let declaration = Declaration {
        timestamp: _ctx.timestamp() as i64,
        transaction_hash: transaction_hash, //but if we do it like that here we will get a problem
        signor_hash: signor_hash,
        object_hash: object_hash,
        declaration_type: declaration_type,
        signature: f.params.signature().value(),
    };

    f.state
        .declarations()
        .get_declaration(&declaration.transaction_hash)
        .set_value(&declaration);

    //4. emit declaration create event
    f.events.declaration_created(
        &declaration.declaration_type,
        &declaration.object_hash,
        &declaration.signature,
        &declaration.signor_hash,
        declaration.timestamp,
        &declaration.transaction_hash,
    );
}



// ******************* IDENTITY CLAIM ********************
// *******************************************************
pub fn func_create_identity_claim(_ctx: &ScFuncContext, f: &CreateIdentityClaimContext) {
    //1. Calculate hash of the identity claim
    let identifier = f.params.identifier().value();
    let validation_url = f.params.validation_url().value();
    let public_key_bytes = hex::decode(f.params.public_key().value()).unwrap();

    let all = [
        identifier.as_bytes(),
        validation_url.as_bytes(),
        &public_key_bytes,
    ]
    .join("".as_bytes());

    let mut hasher = Sha256::new();
    hasher.update(all);
    let hash = hasher.finalize();
    let transaction_hash = hash_from_bytes(hash.as_slice());

    //2. check if signature is a valid signature of the identity with signature hash
    //2.1 check if identity claim already exists
    let _identity_claim = f
        .state
        .identity_claims()
        .get_identity_claim(&transaction_hash);

    
    //2.2 validate signature
    // signature matches public_key and transaction_hash
    let public_key_bytes = hex::decode(f.params.public_key().value()).unwrap();
    let verify_key = VerifyingKey::from_sec1_bytes(&public_key_bytes).unwrap();
    let signature_bytes = hex::decode(f.params.signature().value()).unwrap();
    let signature: Signature = Signature::from_der(&signature_bytes).unwrap();

    let bytes: &[u8] = &[
        identifier.as_bytes(),
        validation_url.as_bytes(),
        &public_key_bytes,
    ]
    .join("".as_bytes());

    _ctx.require(
        verify_key.verify(bytes, &signature).is_ok(),
        "signature is not valid",
    ); 

    //3. store the identity claim in the state
    let identity_claim = IdentityClaim {
        timestamp: _ctx.timestamp() as i64,
        transaction_hash: transaction_hash,
        identifier: identifier,
        validation_url: validation_url,
        public_key: f.params.public_key().value(),
        signature: f.params.signature().value(),
    };

    f.state
        .identity_claims()
        .get_identity_claim(&identity_claim.transaction_hash)
        .set_value(&identity_claim);
    _ctx.log(&hex::encode(identity_claim.transaction_hash.to_bytes()));

    // 4. emit identity claim create event
    f.events.identity_claim_created(
        &identity_claim.identifier,
        &identity_claim.public_key,
        &identity_claim.signature,
        identity_claim.timestamp,
        &identity_claim.transaction_hash,
        &identity_claim.validation_url,
    );
}

// ********************** ORGANIZATION ****************************
// ****************************************************************
pub fn func_register_organization(_ctx: &ScFuncContext, f: &RegisterOrganizationContext) {
    let org_id = f.params.org_id().value();
    let org_name = f.params.org_name().value();

    let organization = Organization {
        org_id: org_id.clone(),
        org_name: org_name.clone(),
    };

    f.state
        .organizations()
        .get_organization(&org_id)
        .set_value(&organization);

    // Emit organization registered event
    f.events.organization_registered(&org_id, &org_name, _ctx.timestamp() as i64);
}

// ********************** SENSOR **********************************
// ****************************************************************
pub fn func_register_sensor(_ctx: &ScFuncContext, f: &RegisterSensorContext) {
    let sensor_id = f.params.sensor_id().value();
    let sensor_name = f.params.sensor_name().value();
    let org_id = f.params.org_id().value();

    let sensor = Sensor {
        sensor_id: sensor_id.clone(),
        sensor_name: sensor_name.clone(),
        org_id: org_id.clone(),
    };

    f.state
        .sensors()
        .get_sensor(&sensor_id)
        .set_value(&sensor);

    // Emit sensor registered event
    f.events.sensor_registered(&sensor_id, &sensor_name, &org_id, _ctx.timestamp() as i64);
}

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.caller());
}
// func for set * 
pub fn func_set_owner(_ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.owner().value());
}

pub fn view_get_declarations(_ctx: &ScViewContext, _f: &GetDeclarationsContext) {
    //f.results.owner().set_value(&f.state.owner().value());
}

pub fn view_get_graph_of_trust_info(_ctx: &ScViewContext, f: &GetGraphOfTrustInfoContext) {
    f.results.info().set_value("Hello World!");
}

pub fn view_get_identity_claims(_ctx: &ScViewContext, _f: &GetIdentityClaimsContext) {
}

pub fn view_get_owner(_ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}

pub fn func_set_identifier(_ctx: &ScFuncContext, f: &SetIdentifierContext) {
    f.state.identifier().set_value(&f.params.identifier().value()) ;
}

pub fn func_set_public_key(_ctx: &ScFuncContext, f: &SetPublicKeyContext) {
    f.state.public_key().set_value(&f.params.public_key().value()) ;
}

pub fn func_set_signature(_ctx: &ScFuncContext, f: &SetSignatureContext) {
    f.state.signature().set_value(&f.params.signature().value()) ;
}

pub fn func_set_validation_url(_ctx: &ScFuncContext, f: &SetValidationUrlContext) {
    f.state.validation_url().set_value(&f.params.validation_url().value()) ;
}

pub fn view_get_signature_info(_ctx: &ScViewContext, f: &GetSignatureInfoContext) {
    f.results.signature().set_value(&f.state.signature().value());
}
