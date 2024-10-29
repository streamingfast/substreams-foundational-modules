package main

import (
	"context"
	"fmt"

	v1 "github.com/streamingfast/substreams-foundational-modules/solana-accounts-common/pb/sf/solana/type/v1"
	indexv1 "github.com/streamingfast/substreams-foundational-modules/solana-accounts-common/pb/sf/substreams/index/v1"
	typev1 "github.com/streamingfast/substreams-foundational-modules/solana-accounts-common/pb/sf/substreams/solana/type/v1"
	"github.com/streamingfast/substreams-foundational-modules/solana-accounts-common/sqe"

	"github.com/mr-tron/base58"
)

func IndexAccounts(block *v1.AccountBlock) (*indexv1.Keys, error) {
	ix := &Index{
		Keys: &indexv1.Keys{},
	}

	accountMap := make(map[string]struct{})
	for _, account := range block.Accounts {
		for _, key := range indexForAccount(account).Keys {
			if _, exists := accountMap[key]; exists {
				continue
			}

			accountMap[key] = struct{}{}
			ix.AddKey(key)
		}
	}

	return ix.Keys, nil
}

func FilteredAccounts(query string, block *v1.AccountBlock) (*typev1.FilteredAccounts, error) {
	filteredAccounts := &typev1.FilteredAccounts{
		Accounts: []*v1.Account{},
	}

	for _, account := range block.Accounts {
		ix := indexForAccount(account)
		applies, err := applyQuery(query, ix)
		if err != nil {
			return nil, fmt.Errorf("applying query: %w", err)
		}

		if !applies {
			continue
		}

		filteredAccounts.Accounts = append(filteredAccounts.Accounts, account)
	}

	return filteredAccounts, nil
}

func applyQuery(query string, keys *indexv1.Keys) (bool, error) {
	keyQuerier := sqe.NewFromKeys(keys.Keys)
	q, err := sqe.Parse(context.Background(), query)
	if err != nil {
		return false, fmt.Errorf("parsing query %q: %w", query, err)
	}
	return sqe.KeysApply(q, keyQuerier), nil
}

func indexForAccount(account *v1.Account) *indexv1.Keys {
	keys := &indexv1.Keys{}

	accountKey := "account:" + base58Encode(account.Address)
	ownerKey := "owner:" + base58Encode(account.Owner)

	keys.Keys = append(keys.Keys, accountKey, ownerKey)
	return keys
}

func base58Encode(input []byte) string {
	encoded := base58.Encode(input)
	return encoded
}

func base58Decode(input string) []byte {
	decoded, err := base58.Decode(input)
	if err != nil {
		panic(err)
	}
	return decoded
}
