// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package test

import (
	"testing"

	"github.com/iotaledger/wasp/contracts/wasm/gotnew/go/gotnew"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmsolo"
	"github.com/stretchr/testify/require"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, gotnew.ScName, gotnew.OnLoad)
	require.NoError(t, ctx.ContractExists(gotnew.ScName))
}
