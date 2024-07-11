package main

import (
	"encoding/hex"
	"testing"

	v1 "github.com/streamingfast/substreams-foundational-modules/starket-common/pb/sf/substreams/starknet/type/v1"

	pbstarknet "github.com/streamingfast/substreams-foundational-modules/starket-common/pb/sf/starknet/type/v1"
	"github.com/stretchr/testify/require"
)

func Test_mapBlockEvents(t *testing.T) {
	block := &pbstarknet.Block{
		Transactions: []*pbstarknet.TransactionWithReceipt{
			{
				Transaction: &pbstarknet.TransactionWithReceipt_DeployTransactionV0{},
			},
		},
	}

	txs, err := AllTransaction(block)
	require.NoError(t, err)

	require.Equal(t, 1, len(txs.TransactionsWithReceipt))
}

func Test_FilteredTransactions(t *testing.T) {
	ch := []byte("class_hash")
	xch := "0x" + hex.EncodeToString(ch)

	tx := &pbstarknet.TransactionWithReceipt{
		Transaction: &pbstarknet.TransactionWithReceipt_DeployTransactionV0{
			DeployTransactionV0: &pbstarknet.DeployTransactionV0{
				ClassHash:           []byte("class_hash"),
				Version:             "0x3",
				ContractAddressSalt: []byte("contract_address_salt"),
			},
		},
		Receipt: &pbstarknet.TransactionReceipt{},
	}
	txs := &v1.Transactions{
		TransactionsWithReceipt: []*pbstarknet.TransactionWithReceipt{tx},
	}

	txs, err := FilteredTransactions("tx:class_hash:"+xch, txs)
	require.NoError(t, err)

	require.Equal(t, 1, len(txs.TransactionsWithReceipt))
}
