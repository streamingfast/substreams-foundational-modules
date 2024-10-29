package main

import indexv1 "github.com/streamingfast/substreams-foundational-modules/solana-accounts-common/pb/sf/substreams/index/v1"

type Index struct {
	Keys *indexv1.Keys
}

func (i *Index) AddKey(key string) {
	i.Keys.Keys = append(i.Keys.Keys, key)
}
