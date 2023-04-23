// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;

pub fn func_approve(_ctx: &ScFuncContext, _f: &ApproveContext) {
}

pub fn func_burn(_ctx: &ScFuncContext, _f: &BurnContext) {
}

pub fn func_init(ctx: &ScFuncContext, f: &InitContext) {
    if f.params.name().exists() {
        f.state.name().set_value(&f.params.name().value());
        return;
    }
    // Convert the ScAgentID to a string
    let sender_str = ctx.request_sender().to_string();
    f.state.name().set_value(&sender_str);
}


pub fn func_mint(_ctx: &ScFuncContext, _f: &MintContext) {
}

pub fn func_safe_transfer_from(_ctx: &ScFuncContext, _f: &SafeTransferFromContext) {
}

pub fn func_set_approval_for_all(_ctx: &ScFuncContext, _f: &SetApprovalForAllContext) {
}

pub fn func_transfer_from(_ctx: &ScFuncContext, _f: &TransferFromContext) {
}

pub fn view_balance_of(_ctx: &ScViewContext, _f: &BalanceOfContext) {
}

pub fn view_get_approved(_ctx: &ScViewContext, _f: &GetApprovedContext) {
}

pub fn view_is_approved_for_all(_ctx: &ScViewContext, _f: &IsApprovedForAllContext) {
}

pub fn view_name(_ctx: &ScViewContext, _f: &NameContext) {
}

pub fn view_owner_of(_ctx: &ScViewContext, _f: &OwnerOfContext) {
}

pub fn view_symbol(_ctx: &ScViewContext, _f: &SymbolContext) {
}

pub fn view_token_uri(_ctx: &ScViewContext, _f: &TokenURIContext) {
}
