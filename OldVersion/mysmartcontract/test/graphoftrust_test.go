// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package test

import (
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"strings"
	"testing"

	"github.com/iotaledger/wasp/contracts/wasm/mysmartcontract/go/graphoftrust"
	"github.com/iotaledger/wasp/packages/wasmvm/wasmsolo"
	"github.com/stretchr/testify/require"
)

func TestDeploy(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, graphoftrust.ScName, graphoftrust.OnLoad)
	require.NoError(t, ctx.ContractExists(graphoftrust.ScName))
}

func TestGetInfo(t *testing.T) {
	ctx := wasmsolo.NewSoloContext(t, graphoftrust.ScName, graphoftrust.OnLoad)

	f := graphoftrust.ScFuncs.GetGraphOfTrustInfo(ctx)
	f.Func.Call()
	require.EqualValues(t, "Hello World!", f.Results.Info().String())

	require.NoError(t, ctx.Err)
}

func TestCreateIdentityClaim01(t *testing.T) {
	pubkeyCurve := elliptic.P256()
	privateKey, _ := ecdsa.GenerateKey(pubkeyCurve, rand.Reader)
	publicKeyASN1 := elliptic.Marshal(elliptic.P256(), privateKey.PublicKey.X, privateKey.PublicKey.Y)

	identifier := "Test"
	validationUrl := "https://test.io/.well-known"

	h := sha256.New()
	h.Write([]byte(identifier))
	h.Write([]byte(validationUrl))
	h.Write(publicKeyASN1)
	hash := h.Sum(nil)
	signature, _ := ecdsa.SignASN1(rand.Reader, privateKey, hash)
	ctx := wasmsolo.NewSoloContext(t, graphoftrust.ScName, graphoftrust.OnLoad)

	f := graphoftrust.ScFuncs.CreateIdentityClaim(ctx)
	f.Params.Identifier().SetValue(identifier)
	f.Params.ValidationUrl().SetValue(validationUrl)
	f.Params.PublicKey().SetValue(hex.EncodeToString(publicKeyASN1))
	f.Params.Signature().SetValue(hex.EncodeToString(signature))
	f.Func.TransferIotas(1).Post()

	require.NoError(t, ctx.Err)
}

// crate the same identity claim twice
// should throw an error
func TestCreateIdentityClaim02(t *testing.T) {

	pubkeyCurve := elliptic.P256()
	privateKey, _ := ecdsa.GenerateKey(pubkeyCurve, rand.Reader)
	publicKeyASN1 := elliptic.Marshal(elliptic.P256(), privateKey.PublicKey.X, privateKey.PublicKey.Y)

	identifier := "Test"
	validationUrl := "https://test.io/.well-known"

	h := sha256.New()
	h.Write([]byte(identifier))
	h.Write([]byte(validationUrl))
	h.Write(publicKeyASN1)
	hash := h.Sum(nil)
	signature, _ := ecdsa.SignASN1(rand.Reader, privateKey, hash)
	ctx := wasmsolo.NewSoloContext(t, graphoftrust.ScName, graphoftrust.OnLoad)

	f := graphoftrust.ScFuncs.CreateIdentityClaim(ctx)
	f.Params.Identifier().SetValue(identifier)
	f.Params.ValidationUrl().SetValue(validationUrl)
	f.Params.PublicKey().SetValue(hex.EncodeToString(publicKeyASN1))
	f.Params.Signature().SetValue(hex.EncodeToString(signature))
	f.Func.TransferIotas(1).Post()
	f.Func.TransferIotas(1).Post()

	require.Error(t, ctx.Err)
	require.True(t, strings.HasSuffix(ctx.Err.Error(), "identity claim already exists"))
}

func TestCreateIdentityClaim03(t *testing.T) {

	pubkeyCurve := elliptic.P256()
	privateKey, _ := ecdsa.GenerateKey(pubkeyCurve, rand.Reader)
	publicKeyASN1 := elliptic.Marshal(elliptic.P256(), privateKey.PublicKey.X, privateKey.PublicKey.Y)

	identifier := "Test"
	validationUrl := "https://test.io/.well-known"

	h := sha256.New()
	h.Write([]byte(identifier))
	h.Write([]byte(validationUrl))
	h.Write(publicKeyASN1)
	h.Write([]byte("let the signature fail"))
	hash := h.Sum(nil)
	signature, _ := ecdsa.SignASN1(rand.Reader, privateKey, hash)
	ctx := wasmsolo.NewSoloContext(t, graphoftrust.ScName, graphoftrust.OnLoad)

	f := graphoftrust.ScFuncs.CreateIdentityClaim(ctx)
	f.Params.Identifier().SetValue(identifier)
	f.Params.ValidationUrl().SetValue(validationUrl)
	f.Params.PublicKey().SetValue(hex.EncodeToString(publicKeyASN1))
	f.Params.Signature().SetValue(hex.EncodeToString(signature))
	f.Func.TransferIotas(1).Post()
	f.Func.TransferIotas(1).Post()

	require.Error(t, ctx.Err)
	require.True(t, strings.HasSuffix(ctx.Err.Error(), "signature is not valid!"))
}

func TestCreateDeclaration01(t *testing.T) {
	pubkeyCurve := elliptic.P256()
	privateKey, _ := ecdsa.GenerateKey(pubkeyCurve, rand.Reader)
	publicKeyASN1 := elliptic.Marshal(elliptic.P256(), privateKey.PublicKey.X, privateKey.PublicKey.Y)

	// add identity claim that sings the declaration
	identifier := "Test"
	validationUrl := "https://test.io/.well-known"
	h := sha256.New()
	h.Write([]byte(identifier))
	h.Write([]byte(validationUrl))
	h.Write(publicKeyASN1)
	hash := h.Sum(nil)
	signature, _ := ecdsa.SignASN1(rand.Reader, privateKey, hash)
	ctx := wasmsolo.NewSoloContext(t, graphoftrust.ScName, graphoftrust.OnLoad)

	f := graphoftrust.ScFuncs.CreateIdentityClaim(ctx)
	f.Params.Identifier().SetValue(identifier)
	f.Params.ValidationUrl().SetValue(validationUrl)
	f.Params.PublicKey().SetValue(hex.EncodeToString(publicKeyASN1))
	f.Params.Signature().SetValue(hex.EncodeToString(signature))
	f.Func.TransferIotas(1).Post()

	// add the declaration
	signorHash := hash
	objectHash, _ := hex.DecodeString("7843c827d39ecd133412fc76da8749badcd5425767f69383102dd7493fd22730")
	declarationType := "issues"
	h2 := sha256.New()
	h2.Write(signorHash)
	h2.Write(objectHash)
	h2.Write([]byte(declarationType))
	hash2 := h2.Sum(nil)
	signature2, _ := ecdsa.SignASN1(rand.Reader, privateKey, hash2)

	f2 := graphoftrust.ScFuncs.CreateDeclaration(ctx)
	f2.Params.SignorHash().SetValue(wasmlib.NewScHashFromBytes(signorHash))
	f2.Params.ObjectHash().SetValue(wasmlib.NewScHashFromBytes(objectHash))
	f2.Params.DeclarationType().SetValue(declarationType)
	f2.Params.Signature().SetValue(hex.EncodeToString(signature2))
	f2.Func.TransferIotas(1).Post()
	require.NoError(t, ctx.Err)
}

func TestCreateDeclaration02(t *testing.T) {
	pubkeyCurve := elliptic.P256()
	privateKey, _ := ecdsa.GenerateKey(pubkeyCurve, rand.Reader)
	publicKeyASN1 := elliptic.Marshal(elliptic.P256(), privateKey.PublicKey.X, privateKey.PublicKey.Y)

	// add identity claim that sings the declaration
	identifier := "Test"
	validationUrl := "https://test.io/.well-known"
	h := sha256.New()
	h.Write([]byte(identifier))
	h.Write([]byte(validationUrl))
	h.Write(publicKeyASN1)
	hash := h.Sum(nil)
	signature, _ := ecdsa.SignASN1(rand.Reader, privateKey, hash)
	ctx := wasmsolo.NewSoloContext(t, graphoftrust.ScName, graphoftrust.OnLoad)

	f := graphoftrust.ScFuncs.CreateIdentityClaim(ctx)
	f.Params.Identifier().SetValue(identifier)
	f.Params.ValidationUrl().SetValue(validationUrl)
	f.Params.PublicKey().SetValue(hex.EncodeToString(publicKeyASN1))
	f.Params.Signature().SetValue(hex.EncodeToString(signature))
	f.Func.TransferIotas(1).Post()

	// add the declaration
	signorHash, _ := hex.DecodeString("7843c827d39ecd133412fc76da8749badcd5425767f69383102dd7493fd22730")
	objectHash, _ := hex.DecodeString("7843c827d39ecd133412fc76da8749badcd5425767f69383102dd7493fd22730")
	declarationType := "issues"
	h2 := sha256.New()
	h2.Write(signorHash)
	h2.Write(objectHash)
	h2.Write([]byte(declarationType))
	hash2 := h2.Sum(nil)
	signature2, _ := ecdsa.SignASN1(rand.Reader, privateKey, hash2)

	f2 := graphoftrust.ScFuncs.CreateDeclaration(ctx)
	f2.Params.SignorHash().SetValue(wasmlib.NewScHashFromBytes(signorHash))
	f2.Params.ObjectHash().SetValue(wasmlib.NewScHashFromBytes(objectHash))
	f2.Params.DeclarationType().SetValue(declarationType)
	f2.Params.Signature().SetValue(hex.EncodeToString(signature2))
	f2.Func.TransferIotas(1).Post()

	require.Error(t, ctx.Err)
	require.True(t, strings.HasSuffix(ctx.Err.Error(), "signor does not exist!"))
}

func TestCreateDeclaration03(t *testing.T) {
	pubkeyCurve := elliptic.P256()
	privateKey, _ := ecdsa.GenerateKey(pubkeyCurve, rand.Reader)
	publicKeyASN1 := elliptic.Marshal(elliptic.P256(), privateKey.PublicKey.X, privateKey.PublicKey.Y)

	// add identity claim that sings the declaration
	identifier := "Test"
	validationUrl := "https://test.io/.well-known"
	h := sha256.New()
	h.Write([]byte(identifier))
	h.Write([]byte(validationUrl))
	h.Write(publicKeyASN1)
	hash := h.Sum(nil)
	signature, _ := ecdsa.SignASN1(rand.Reader, privateKey, hash)
	ctx := wasmsolo.NewSoloContext(t, graphoftrust.ScName, graphoftrust.OnLoad)

	f := graphoftrust.ScFuncs.CreateIdentityClaim(ctx)
	f.Params.Identifier().SetValue(identifier)
	f.Params.ValidationUrl().SetValue(validationUrl)
	f.Params.PublicKey().SetValue(hex.EncodeToString(publicKeyASN1))
	f.Params.Signature().SetValue(hex.EncodeToString(signature))
	f.Func.TransferIotas(1).Post()

	// add the declaration
	signorHash := hash
	objectHash, _ := hex.DecodeString("7843c827d39ecd133412fc76da8749badcd5425767f69383102dd7493fd22730")
	declarationType := "issues"
	h2 := sha256.New()
	h2.Write(signorHash)
	h2.Write(objectHash)
	h2.Write([]byte(declarationType))
	h2.Write([]byte("let the signature fail"))
	hash2 := h2.Sum(nil)
	signature2, _ := ecdsa.SignASN1(rand.Reader, privateKey, hash2)

	f2 := graphoftrust.ScFuncs.CreateDeclaration(ctx)
	f2.Params.SignorHash().SetValue(wasmlib.NewScHashFromBytes(signorHash))
	f2.Params.ObjectHash().SetValue(wasmlib.NewScHashFromBytes(objectHash))
	f2.Params.DeclarationType().SetValue(declarationType)
	f2.Params.Signature().SetValue(hex.EncodeToString(signature2))
	f2.Func.TransferIotas(1).Post()

	require.Error(t, ctx.Err)
	require.True(t, strings.HasSuffix(ctx.Err.Error(), "signature is not valid!"))
