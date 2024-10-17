//go:build !tinygo && !wasip1

package main

import (
	"encoding/hex"
	"testing"

	pbstarknet "github.com/streamingfast/substreams-foundational-modules/starknet-common/pb/sf/starknet/type/v1"
	v1 "github.com/streamingfast/substreams-foundational-modules/starknet-common/pb/sf/substreams/starknet/type/v1"
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

	txs, err := AllTransactions(block)
	require.NoError(t, err)

	require.Equal(t, 1, len(txs.TransactionsWithReceipt))
}

func Test_FilteredTransactions(t *testing.T) {
	ch := []byte("class_hash")
	xch := padFeltString("0x" + hex.EncodeToString(ch))

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

func TestPadFeltString(t *testing.T) {
	cases := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "sunny_path",
			input:    "0x29598b407c8716b17f6d2795eca1b471413fa03fb145a5e337",
			expected: "0x0000000000000029598b407c8716b17f6d2795eca1b471413fa03fb145a5e337",
		},
		{
			name:     "ekubo_contract",
			input:    "0x2e0af29598b407c8716b17f6d2795eca1b471413fa03fb145a5e33722184067",
			expected: "0x02e0af29598b407c8716b17f6d2795eca1b471413fa03fb145a5e33722184067",
		},
		{
			name:     "address without lead",
			input:    "2e0af29598b407c8716b17f6d2795eca1b471413fa03fb145a5e33722184067",
			expected: "02e0af29598b407c8716b17f6d2795eca1b471413fa03fb145a5e33722184067",
		},
	}

	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			result := padFeltString(c.input)
			require.Equal(t, c.expected, result)
		})
	}
}
