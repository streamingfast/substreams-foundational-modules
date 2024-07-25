package main

import pbindex "github.com/streamingfast/substreams-foundational-modules/starknet-common/pb/sf/substreams/index/v1"

type Index struct {
	Keys *pbindex.Keys
}

func (i *Index) AddKey(key string) {
	i.Keys.Keys = append(i.Keys.Keys, key)
}
