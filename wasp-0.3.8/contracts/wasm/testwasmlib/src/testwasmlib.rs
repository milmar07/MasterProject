// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasmlib::*;

use crate::*;

pub fn func_address_map_of_address_array_append(ctx: &ScFuncContext, f: &AddressMapOfAddressArrayAppendContext) {
}

pub fn func_address_map_of_address_array_clear(ctx: &ScFuncContext, f: &AddressMapOfAddressArrayClearContext) {
}

pub fn func_address_map_of_address_array_set(ctx: &ScFuncContext, f: &AddressMapOfAddressArraySetContext) {
}

pub fn func_address_map_of_address_map_clear(ctx: &ScFuncContext, f: &AddressMapOfAddressMapClearContext) {
}

pub fn func_address_map_of_address_map_set(ctx: &ScFuncContext, f: &AddressMapOfAddressMapSetContext) {
}

pub fn func_array_of_address_array_append(ctx: &ScFuncContext, f: &ArrayOfAddressArrayAppendContext) {
}

pub fn func_array_of_address_array_clear(ctx: &ScFuncContext, f: &ArrayOfAddressArrayClearContext) {
}

pub fn func_array_of_address_array_set(ctx: &ScFuncContext, f: &ArrayOfAddressArraySetContext) {
}

pub fn func_array_of_address_map_clear(ctx: &ScFuncContext, f: &ArrayOfAddressMapClearContext) {
}

pub fn func_array_of_address_map_set(ctx: &ScFuncContext, f: &ArrayOfAddressMapSetContext) {
}

pub fn func_array_of_string_array_append(ctx: &ScFuncContext, f: &ArrayOfStringArrayAppendContext) {
}

pub fn func_array_of_string_array_clear(ctx: &ScFuncContext, f: &ArrayOfStringArrayClearContext) {
}

pub fn func_array_of_string_array_set(ctx: &ScFuncContext, f: &ArrayOfStringArraySetContext) {
}

pub fn func_array_of_string_map_clear(ctx: &ScFuncContext, f: &ArrayOfStringMapClearContext) {
}

pub fn func_array_of_string_map_set(ctx: &ScFuncContext, f: &ArrayOfStringMapSetContext) {
}

pub fn func_param_types(ctx: &ScFuncContext, f: &ParamTypesContext) {
}

pub fn func_random(ctx: &ScFuncContext, f: &RandomContext) {
}

pub fn func_string_map_of_string_array_append(ctx: &ScFuncContext, f: &StringMapOfStringArrayAppendContext) {
}

pub fn func_string_map_of_string_array_clear(ctx: &ScFuncContext, f: &StringMapOfStringArrayClearContext) {
}

pub fn func_string_map_of_string_array_set(ctx: &ScFuncContext, f: &StringMapOfStringArraySetContext) {
}

pub fn func_string_map_of_string_map_clear(ctx: &ScFuncContext, f: &StringMapOfStringMapClearContext) {
}

pub fn func_string_map_of_string_map_set(ctx: &ScFuncContext, f: &StringMapOfStringMapSetContext) {
}

pub fn func_take_allowance(ctx: &ScFuncContext, f: &TakeAllowanceContext) {
}

pub fn func_take_balance(ctx: &ScFuncContext, f: &TakeBalanceContext) {
}

pub fn func_trigger_event(ctx: &ScFuncContext, f: &TriggerEventContext) {
}

pub fn view_address_map_of_address_array_length(ctx: &ScViewContext, f: &AddressMapOfAddressArrayLengthContext) {
}

pub fn view_address_map_of_address_array_value(ctx: &ScViewContext, f: &AddressMapOfAddressArrayValueContext) {
}

pub fn view_address_map_of_address_map_value(ctx: &ScViewContext, f: &AddressMapOfAddressMapValueContext) {
}

pub fn view_array_of_address_array_length(ctx: &ScViewContext, f: &ArrayOfAddressArrayLengthContext) {
}

pub fn view_array_of_address_array_value(ctx: &ScViewContext, f: &ArrayOfAddressArrayValueContext) {
}

pub fn view_array_of_address_map_value(ctx: &ScViewContext, f: &ArrayOfAddressMapValueContext) {
}

pub fn view_array_of_string_array_length(ctx: &ScViewContext, f: &ArrayOfStringArrayLengthContext) {
}

pub fn view_array_of_string_array_value(ctx: &ScViewContext, f: &ArrayOfStringArrayValueContext) {
}

pub fn view_array_of_string_map_value(ctx: &ScViewContext, f: &ArrayOfStringMapValueContext) {
}

pub fn view_big_int_add(ctx: &ScViewContext, f: &BigIntAddContext) {
}

pub fn view_big_int_div(ctx: &ScViewContext, f: &BigIntDivContext) {
}

pub fn view_big_int_div_mod(ctx: &ScViewContext, f: &BigIntDivModContext) {
}

pub fn view_big_int_mod(ctx: &ScViewContext, f: &BigIntModContext) {
}

pub fn view_big_int_mul(ctx: &ScViewContext, f: &BigIntMulContext) {
}

pub fn view_big_int_shl(ctx: &ScViewContext, f: &BigIntShlContext) {
}

pub fn view_big_int_shr(ctx: &ScViewContext, f: &BigIntShrContext) {
}

pub fn view_big_int_sub(ctx: &ScViewContext, f: &BigIntSubContext) {
}

pub fn view_block_record(ctx: &ScViewContext, f: &BlockRecordContext) {
}

pub fn view_block_records(ctx: &ScViewContext, f: &BlockRecordsContext) {
}

pub fn view_check_address(ctx: &ScViewContext, f: &CheckAddressContext) {
}

pub fn view_check_agent_id(ctx: &ScViewContext, f: &CheckAgentIDContext) {
}

pub fn view_check_big_int(ctx: &ScViewContext, f: &CheckBigIntContext) {
}

pub fn view_check_bool(ctx: &ScViewContext, f: &CheckBoolContext) {
}

pub fn view_check_bytes(ctx: &ScViewContext, f: &CheckBytesContext) {
}

pub fn view_check_eth_address_and_agent_id(ctx: &ScViewContext, f: &CheckEthAddressAndAgentIDContext) {
}

pub fn view_check_hash(ctx: &ScViewContext, f: &CheckHashContext) {
}

pub fn view_check_hname(ctx: &ScViewContext, f: &CheckHnameContext) {
}

pub fn view_check_int_and_uint(ctx: &ScViewContext, f: &CheckIntAndUintContext) {
}

pub fn view_check_nft_id(ctx: &ScViewContext, f: &CheckNftIDContext) {
}

pub fn view_check_request_id(ctx: &ScViewContext, f: &CheckRequestIDContext) {
}

pub fn view_check_string(ctx: &ScViewContext, f: &CheckStringContext) {
}

pub fn view_check_token_id(ctx: &ScViewContext, f: &CheckTokenIDContext) {
}

pub fn view_get_random(ctx: &ScViewContext, f: &GetRandomContext) {
}

pub fn view_string_map_of_string_array_length(ctx: &ScViewContext, f: &StringMapOfStringArrayLengthContext) {
}

pub fn view_string_map_of_string_array_value(ctx: &ScViewContext, f: &StringMapOfStringArrayValueContext) {
}

pub fn view_string_map_of_string_map_value(ctx: &ScViewContext, f: &StringMapOfStringMapValueContext) {
}

pub fn view_token_balance(ctx: &ScViewContext, f: &TokenBalanceContext) {
}
