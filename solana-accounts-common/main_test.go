package main

import (
	"testing"

	"github.com/stretchr/testify/assert"

	typev1 "github.com/streamingfast/substreams-foundational-modules/solana-accounts-common/pb/sf/solana/type/v1"
)

func TestIndexAccounts(t *testing.T) {
	testBlock := &typev1.AccountBlock{
		Accounts: &typev1.Accounts{
			Accounts: []*typev1.Account{
				{
					Address: []byte("bQbp"),
					Owner:   []byte("a3cM"),
				},
				{
					Address: []byte("emR8"),
					Owner:   []byte("a3cM"),
				},
			},
		},
	}

	keys, err := IndexAccounts(testBlock)
	assert.NoError(t, err)
	assert.NotNil(t, keys)
	assert.Len(t, keys.Keys, 3)
}

func TestFilteredAccounts_AccountQuery(t *testing.T) {
	testBlock := &typev1.AccountBlock{
		Accounts: &typev1.Accounts{
			Accounts: []*typev1.Account{
				{
					Address: base58Decode("bQbp"),
					Owner:   base58Decode("a3cM"),
				},
				{
					Address: base58Decode("emR8"),
					Owner:   base58Decode("a3cM"),
				},
			},
		},
	}

	type test struct {
		query    string
		expected *typev1.Accounts
	}
	for _, tt := range []test{
		{
			query: "account:bQbp",
			expected: &typev1.Accounts{
				Accounts: []*typev1.Account{
					{
						Address: base58Decode("bQbp"),
						Owner:   base58Decode("a3cM"),
					},
				},
			},
		},
		{
			query: "account:emR8",
			expected: &typev1.Accounts{
				Accounts: []*typev1.Account{
					{
						Address: base58Decode("emR8"),
						Owner:   base58Decode("a3cM"),
					},
				},
			},
		},
		{
			query: "owner:a3cM",
			expected: &typev1.Accounts{
				Accounts: []*typev1.Account{
					{
						Address: base58Decode("bQbp"),
						Owner:   base58Decode("a3cM"),
					},
					{
						Address: base58Decode("emR8"),
						Owner:   base58Decode("a3cM"),
					},
				},
			},
		},
	} {
		tt := tt
		t.Run(tt.query, func(t *testing.T) {
			filteredAccounts, err := FilteredAccounts(tt.query, testBlock)
			assert.NoError(t, err)
			assert.NotNil(t, filteredAccounts)
			assert.Equal(t, tt.expected, filteredAccounts)
		})
	}
}
