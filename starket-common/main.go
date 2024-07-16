package main

import (
	"context"
	"fmt"
	"time"

	"github.com/streamingfast/substreams-foundational-modules/starket-common/sqe"

	"github.com/NethermindEth/juno/core/felt"
	pbstarknet "github.com/streamingfast/substreams-foundational-modules/starket-common/pb/sf/starknet/type/v1"
	pbindex "github.com/streamingfast/substreams-foundational-modules/starket-common/pb/sf/substreams/index/v1"
	v1 "github.com/streamingfast/substreams-foundational-modules/starket-common/pb/sf/substreams/starknet/type/v1"
	pbsubstreams "github.com/streamingfast/substreams-foundational-modules/starket-common/pb/sf/substreams/v1"
	"google.golang.org/protobuf/types/known/timestamppb"
)

func AllTransactions(block *pbstarknet.Block) (*v1.Transactions, error) {
	clock := &pbsubstreams.Clock{
		Id:        feltToString(block.BlockHash),
		Number:    block.BlockNumber,
		Timestamp: timestamppb.New(time.Unix(int64(block.Timestamp), 0)),
	}

	transactions := &v1.Transactions{
		Clock: clock,
	}
	for _, tx := range block.Transactions {
		transactions.TransactionsWithReceipt = append(transactions.TransactionsWithReceipt, tx)
	}

	return transactions, nil
}

func IndexTransaction(transactions *v1.Transactions) (*pbindex.Keys, error) {
	keys := &pbindex.Keys{}
	for _, tx := range transactions.TransactionsWithReceipt {
		idx, err := indexForTransactionsWithReceipt(tx)
		if err != nil {
			return nil, fmt.Errorf("indexing transaction: %w", err)
		}
		keys.Keys = append(keys.Keys, idx.Keys.Keys...)
	}
	return keys, nil
}

func indexForTransactionsWithReceipt(transaction *pbstarknet.TransactionWithReceipt) (*Index, error) {
	index := &Index{
		Keys: &pbindex.Keys{},
	}

	switch tt := transaction.Transaction.(type) {
	case *pbstarknet.TransactionWithReceipt_DeployTransactionV0:
		t := tt.DeployTransactionV0
		index.AddKey(feltToIndexKey("tx:class_hash", t.ClassHash))
		index.AddKey(feltToIndexKey("tx:contract_address_salt", t.ContractAddressSalt))
		index.AddKey(stringToIndexKey("tx:version", t.Version))

	case *pbstarknet.TransactionWithReceipt_DeployAccountTransactionV1:
		t := tt.DeployAccountTransactionV1
		index.AddKey(feltToIndexKey("tx:class_hash", t.ClassHash))
		index.AddKey(feltToIndexKey("tx:contract_address_salt", t.ContractAddressSalt))
		index.AddKey(stringToIndexKey("tx:version", t.Version))

	case *pbstarknet.TransactionWithReceipt_DeployAccountTransactionV3:
		t := tt.DeployAccountTransactionV3
		index.AddKey(feltToIndexKey("tx:class_hash", t.ClassHash))
		index.AddKey(feltToIndexKey("tx:contract_address_salt", t.ContractAddressSalt))
		index.AddKey(stringToIndexKey("tx:version", t.Version))

	case *pbstarknet.TransactionWithReceipt_DeclareTransactionV0:
		t := tt.DeclareTransactionV0
		index.AddKey(feltToIndexKey("tx:sender_address", t.SenderAddress))
		index.AddKey(feltToIndexKey("tx:class_hash", t.ClassHash))

	case *pbstarknet.TransactionWithReceipt_DeclareTransactionV1:
		t := tt.DeclareTransactionV1
		index.AddKey(feltToIndexKey("tx:sender_address", t.SenderAddress))
		index.AddKey(feltToIndexKey("tx:class_hash", t.ClassHash))

	case *pbstarknet.TransactionWithReceipt_DeclareTransactionV2:
		t := tt.DeclareTransactionV2
		index.AddKey(feltToIndexKey("tx:sender_address", t.SenderAddress))
		index.AddKey(feltToIndexKey("tx:class_hash", t.ClassHash))
		index.AddKey(feltToIndexKey("tx:compile_class_hash", t.CompiledClassHash))

	case *pbstarknet.TransactionWithReceipt_DeclareTransactionV3:
		t := tt.DeclareTransactionV3
		index.AddKey(feltToIndexKey("tx:sender_address", t.SenderAddress))
		index.AddKey(feltToIndexKey("tx:compile_class_hash", t.CompiledClassHash))
		index.AddKey(stringToIndexKey("tx:version", t.Version))

	case *pbstarknet.TransactionWithReceipt_InvokeTransactionV0:
		t := tt.InvokeTransactionV0
		index.AddKey(feltToIndexKey("tx:contract_address", t.ContractAddress))
		index.AddKey(stringToIndexKey("tx:version", t.Version))
		index.AddKey(feltToIndexKey("tx:entry_point_selector", t.EntryPointSelector))

	case *pbstarknet.TransactionWithReceipt_InvokeTransactionV1:
		t := tt.InvokeTransactionV1
		index.AddKey(feltToIndexKey("tx:sender_address", t.SenderAddress))
		index.AddKey(stringToIndexKey("tx:version", t.Version))

	case *pbstarknet.TransactionWithReceipt_InvokeTransactionV3:
		t := tt.InvokeTransactionV3
		index.AddKey(feltToIndexKey("tx:sender_address", t.SenderAddress))
		index.AddKey(stringToIndexKey("tx:version", t.Version))

	case *pbstarknet.TransactionWithReceipt_L1HandlerTransaction:
		t := tt.L1HandlerTransaction
		index.AddKey(stringToIndexKey("tx:version", t.Version))
		index.AddKey(feltToIndexKey("tx:contract_address", t.ContractAddress))
		index.AddKey(feltToIndexKey("tx:entry_point_selector", t.EntryPointSelector))

	default:
		return nil, fmt.Errorf("unknown transaction type %T", transaction)
	}
	receipt := transaction.Receipt
	index.AddKey(fmt.Sprintf("rc:type:%d", receipt.Type))
	index.AddKey(fmt.Sprintf("rc:execution_status:%d", receipt.ExecutionStatus))

	for _, e := range receipt.Events {
		index.AddKey(feltToIndexKey("ev:from_address", e.FromAddress))
		for _, key := range e.Keys {
			index.AddKey(feltToIndexKey("ev:key", key))
		}
	}

	return index, nil
}

func FilteredTransactions(query string, transactions *v1.Transactions) (*v1.Transactions, error) {
	filtered := &v1.Transactions{
		Clock: transactions.Clock,
	}

	for _, tx := range transactions.TransactionsWithReceipt {
		idx, err := indexForTransactionsWithReceipt(tx)
		if err != nil {
			return nil, fmt.Errorf("indexing transaction: %w", err)
		}

		match, err := applyQuery(idx.Keys, query)
		if err != nil {
			return nil, fmt.Errorf("applying query: %w", err)
		}
		if match {
			filtered.TransactionsWithReceipt = append(filtered.TransactionsWithReceipt, tx)
		}

	}

	if len(filtered.TransactionsWithReceipt) == 0 {
		return &v1.Transactions{}, nil
	}

	return filtered, nil
}

func applyQuery(keys *pbindex.Keys, query string) (bool, error) {
	keyQuerier := sqe.NewFromKeys(keys.Keys)
	q, err := sqe.Parse(context.Background(), query)
	if err != nil {
		return false, fmt.Errorf("parsing query %q: %w", query, err)
	}

	return sqe.KeysApply(q, keyQuerier), nil
}

type Index struct {
	Keys *pbindex.Keys
}

func (i *Index) AddKey(key string) {
	i.Keys.Keys = append(i.Keys.Keys, key)
}

func stringToIndexKey(prefix, str string) string {
	return fmt.Sprintf("%s:%s", prefix, str)
}

func feltToIndexKey(prefix string, bytes []byte) string {
	return fmt.Sprintf("%s:%s", prefix, feltToString(bytes))
}

func feltToString(bytes []byte) string {
	f := &felt.Felt{}
	f.SetBytes(bytes)
	return f.String()
}
