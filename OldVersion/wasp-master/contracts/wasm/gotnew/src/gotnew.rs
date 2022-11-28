// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;
use crate::structs::*;
use hex;

use std::env;
use sha2::{Sha256,Digest};
use p256::{ecdsa::{SigningKey,Signature, signature::Signer, VerifyingKey, signature::Verifier}};
use std::convert::TryInto;



// CREATE DECLARATION
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
    let transaction_hash = ScHash::new(&hash);
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
    let transaction_hash = ScHash::new(&hash);
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

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.contract_creator());
}

pub fn func_set_owner(_ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.owner().value());
}

pub fn view_get_declarations(_ctx: &ScViewContext, _f: &GetDeclarationsContext) {
}

pub fn view_get_graph_of_trust_info(_ctx: &ScViewContext, _f: &GetGraphOfTrustInfoContext) {
}

pub fn view_get_identity_claims(_ctx: &ScViewContext, _f: &GetIdentityClaimsContext) {
}

pub fn view_get_owner(_ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}
