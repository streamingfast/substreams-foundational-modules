package main

import (
	"context"
	"fmt"

	"github.com/mr-tron/base58"
	typev1 "github.com/streamingfast/substreams-foundational-modules/solana-accounts-common/pb/sf/solana/type/v1"
	indexv1 "github.com/streamingfast/substreams-foundational-modules/solana-accounts-common/pb/sf/substreams/index/v1"
	"github.com/streamingfast/substreams-foundational-modules/solana-accounts-common/sqe"
)

func IndexAccounts(block *typev1.AccountBlock) (*indexv1.Keys, error) {
	ix := &Index{
		Keys: &indexv1.Keys{},
	}

	accountMap := make(map[string]struct{})
	for _, account := range block.Accounts.Accounts {
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

func FilteredAccounts(query string, block *typev1.AccountBlock) (*typev1.Accounts, error) {
	filteredAccounts := &typev1.Accounts{
		Accounts: []*typev1.Account{},
	}

	for _, account := range block.Accounts.Accounts {
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

func indexForAccount(account *typev1.Account) *indexv1.Keys {
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
