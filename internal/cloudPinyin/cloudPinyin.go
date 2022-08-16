package cloudpinyin

import (
	"errors"
	"fmt"
	"io"
	"net/http"
	"regexp"
	"time"

	"golang.org/x/net/http2"
)

var re, _ = regexp.Compile("[^\"\\[\\],{}}]+")

const maxPinyinLen = 50

type CloudPinyin struct {
	conditioned string
	http2       http.Client
}

// https://posener.github.io/http2/

func NewCloudPinyin() *CloudPinyin {
	client := http.Client{Timeout: 5 * time.Second}
	client.Transport = &http2.Transport{}

	cp := CloudPinyin{"", client}
	return &cp
}

func (c *CloudPinyin) GetCandidates(pinyin string, candCnt int) ([]string, error) {
	if len(pinyin) > maxPinyinLen {
		pinyin = pinyin[:50]
	}

	url := fmt.Sprintf("https://inputtools.google.com/request?text=%s&itc=zh-t-i0-pinyin&num=%d&cp=0&cs=1&ie=utf-8&oe=utf-8&app=demopage", pinyin, candCnt)

	// ["SUCCESS",[["ceshi",["测试","策士","侧视","测","侧","册","策","厕","CE","恻","測"],[],{"annotation":["ce shi","ce shi","ce shi","ce","ce","ce","ce","ce","c e","ce","ce"],"candidate_type":[0,0,0,0,0,0,0,0,0,0,0],"lc":["16 16","16 16","16 16","16","16","16","16","16","0 0","16","16"],"matched_length":[5,5,5,2,2,2,2,2,2,2,2]}]]]
	fmt.Println(url)

	resp, err := c.http2.Get(url)
	if err != nil {
		fmt.Println(err)
		return nil, err
	}
	defer resp.Body.Close()

	data, err := io.ReadAll(resp.Body)
	if err != nil {
		fmt.Println(err)
		return nil, err
	}
	str := string(data)

	cand, err := jstrToCand(str)

	return cand, err
}

func jstrToCand(jstr string) ([]string, error) {
	words := re.FindAllString(jstr, -1)

	fmt.Println(words)

	cand := []string{}
	for i, val := range words {
		if i == 0 {
			if val != "SUCCESS" {
				return nil, errors.New("network request failed")
			}
		}
		if val == "annotation" {
			break
		} else {
			cand = append(cand, val)
		}
	}
	return cand, nil
}
