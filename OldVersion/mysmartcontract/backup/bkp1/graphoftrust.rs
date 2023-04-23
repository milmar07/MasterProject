// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;

pub fn func_create_declaration(ctx: &ScFuncContext, f: &CreateDeclarationContext) {
}

pub fn func_create_identity_claim(ctx: &ScFuncContext, f: &CreateIdentityClaimContext) {
}

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.owner().exists() {
        f.state.owner().set_value(&f.params.owner().value());
        return;
    }
    f.state.owner().set_value(&ctx.contract_creator());
}

pub fn func_set_owner(ctx: &ScFuncContext, f: &SetOwnerContext) {
    f.state.owner().set_value(&f.params.owner().value());
}

pub fn view_get_declarations(ctx: &ScViewContext, f: &GetDeclarationsContext) {
}

pub fn view_get_graph_of_trust_info(ctx: &ScViewContext, f: &GetGraphOfTrustInfoContext) {
}

pub fn view_get_identity_claims(ctx: &ScViewContext, f: &GetIdentityClaimsContext) {
}

pub fn view_get_owner(ctx: &ScViewContext, f: &GetOwnerContext) {
    f.results.owner().set_value(&f.state.owner().value());
}
