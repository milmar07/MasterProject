// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package test

import (
	"testing"

	"github.com/iotaledger/wasp/mysmartcontract/go/mysmartcontract"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmsolo"
	"github.com/stretchr/testify/require"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, mysmartcontract.ScName, mysmartcontract.OnLoad)
	require.NoError(t, ctx.ContractExists(mysmartcontract.ScName))
}
