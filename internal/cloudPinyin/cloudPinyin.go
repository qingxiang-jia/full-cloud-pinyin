package cloudpinyin

import (
	"fmt"
	"io"
	"net/http"
	"regexp"
	"strconv"
	"time"

	"golang.org/x/net/http2"
)

var re, _ = regexp.Compile("[^\"\\[\\],{}}]+")
var reMl, _ = regexp.Compile("matched_length")

const maxPinyinLen = 50

type CloudPinyin struct {
	conditioned string
	http2       http.Client
}

func NewCloudPinyin() *CloudPinyin {
	client := http.Client{Timeout: 5 * time.Second}
	client.Transport = &http2.Transport{}

	cp := CloudPinyin{"", client}
	return &cp
}

func (c *CloudPinyin) GetCandidates(pinyin string, candCnt int) ([]string, []int, error) {
	if len(pinyin) == 0 {
		return []string{}, []int{}, nil
	}
	if len(pinyin) > maxPinyinLen {
		pinyin = pinyin[:50]
	}

	url := fmt.Sprintf("https://inputtools.google.com/request?text=%s&itc=zh-t-i0-pinyin&num=%d&cp=0&cs=1&ie=utf-8&oe=utf-8&app=demopage", pinyin, candCnt)

	resp, err := c.http2.Get(url)
	if err != nil {
		fmt.Println(err)
		return nil, nil, err
	}
	defer resp.Body.Close()

	data, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Println(err)
		return nil, nil, err
	}
	str := string(data)

	cand, matchedLen := jstrToCand(str)

	return cand, matchedLen, err
}

// ["SUCCESS",[["ceshi",["测试","策士","侧视","测","侧","册","策","厕","CE","恻","測"],[],{"annotation":["ce shi","ce shi","ce shi","ce","ce","ce","ce","ce","c e","ce","ce"],"candidate_type":[0,0,0,0,0,0,0,0,0,0,0],"lc":["16 16","16 16","16 16","16","16","16","16","16","0 0","16","16"],"matched_length":[5,5,5,2,2,2,2,2,2,2,2]}]]]
func jstrToCand(jstr string) ([]string, []int) {
	words := re.FindAllString(jstr, -1)
	words = words[2:]

	hasMatchedLen := len(reMl.FindString(jstr)) != 0

	// Grab candidates
	cand := []string{}
	for _, val := range words {
		if val == "annotation" {
			break
		} else {
			cand = append(cand, val)
		}
	}

	// Grab matched length
	matchedLen := []int{}
	if hasMatchedLen {
		startIdx := len(words) - len(cand)
		for i := startIdx; i < len(words); i++ {
			len, _ := strconv.Atoi(words[i])
			matchedLen = append(matchedLen, len)
		}
	}

	return cand, matchedLen
}
