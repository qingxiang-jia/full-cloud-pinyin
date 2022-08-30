package cloudpinyin

import "time"

type Cache struct {
	store map[string]*Words
}

type Words struct {
	Time time.Time
	Data []string
}

func (c *Cache) NewCache() *Cache {
	m := make(map[string]*Words)
	cache := Cache{
		store: m,
	}
	return &cache
}

func (c *Cache) Save(key string, val []string) bool {
	if old, ok := c.store[key]; ok {
		if time.Since(old.Time) > time.Hour*720 || len(val) > len(old.Data) { // 30 days
			words := c.store[key]
			words.Data = val
			words.Time = time.Now()
			return true
		}
		return false
	} else {
		c.store[key] = &Words{
			Time: time.Now(),
			Data: val,
		}
		return true
	}
}

func (c *Cache) Get(key string) ([]string, bool) {
	old, ok := c.store[key]
	if ok {
		return old.Data, true
	} else {
		return nil, false
	}
}
