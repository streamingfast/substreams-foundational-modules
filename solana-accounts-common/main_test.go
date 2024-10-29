package main

import (
	"sort"
	"testing"

	v1 "github.com/streamingfast/substreams-foundational-modules/solana-accounts-common/pb/sf/solana/type/v1"

	"github.com/stretchr/testify/assert"
)

func TestIndexAccounts(t *testing.T) {
	testBlock := &v1.AccountBlock{
		Accounts: &v1.Accounts{
			Accounts: []*v1.Account{
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

	keys, err := IndexAccounts(testBlock)
	assert.NoError(t, err)

	expectedKeys := []string{
		"account:bQbp",
		"account:emR8",
		"owner:a3cM",
	}

	sort.Strings(keys.Keys)
	sort.Strings(expectedKeys)
	assert.Equal(t, expectedKeys, keys.Keys)

}

func TestFilteredAccounts_AccountQuery(t *testing.T) {
	testBlock := &v1.AccountBlock{
		Accounts: &v1.Accounts{
			Accounts: []*v1.Account{
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
		expected *v1.Accounts
	}
	for _, tt := range []test{
		{
			query: "account:bQbp",
			expected: &v1.Accounts{
				Accounts: []*v1.Account{
					{
						Address: base58Decode("bQbp"),
						Owner:   base58Decode("a3cM"),
					},
				},
			},
		},
		{
			query: "account:emR8",
			expected: &v1.Accounts{
				Accounts: []*v1.Account{
					{
						Address: base58Decode("emR8"),
						Owner:   base58Decode("a3cM"),
					},
				},
			},
		},
		{
			query: "owner:a3cM",
			expected: &v1.Accounts{
				Accounts: []*v1.Account{
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
		{
			query: "owner:popo || account:bQbp",
			expected: &v1.Accounts{
				Accounts: []*v1.Account{
					{
						Address: base58Decode("bQbp"),
						Owner:   base58Decode("a3cM"),
					},
				},
			},
		},
		{
			query: "owner:a3cM || account:popo",
			expected: &v1.Accounts{
				Accounts: []*v1.Account{
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
		{
			query: "owner:coco || account:popo",
			expected: &v1.Accounts{
				Accounts: []*v1.Account{},
			},
		},
		{
			query: "owner:a3cM && account:bQbp",
			expected: &v1.Accounts{
				Accounts: []*v1.Account{
					{
						Address: base58Decode("bQbp"),
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
