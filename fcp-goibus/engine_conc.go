package main

import (
	"github.com/godbus/dbus/v5"
	"github.com/haunt98/goibus/ibus"
)

type State struct {
	Preedit            []rune
	Candidates         []string
	MatchedLength      []int
	LookupTableVisible bool
	EnglishMode        bool
	CandidateDepth     [8]int
}

type IBusState struct {
	Connection *dbus.Conn
	ObjectPath *dbus.ObjectPath
	Property   *ibus.Property
	PropList   *ibus.PropList
}

type IBusEngineState struct {
	LookupTable        *ibus.LookupTable
	LookupTableVisible bool
	EnglishMode        bool
	CandidateDepth     [8]int
	CurrCandidateDepth int
}

type FcpConcEngine struct {
	ibus.Engine
	CloudPinyin     *CloudPinyin
	IBusState       IBusState
	IBusEngineState IBusEngineState
}

func NewFcpConcEngine(conn *dbus.Conn, path *dbus.ObjectPath, prop *ibus.Property) *FcpConcEngine {
	return &FcpConcEngine{
		Engine:      ibus.BaseEngine(conn, *path),
		CloudPinyin: NewCloudPinyin(),
		IBusState: IBusState{
			Connection: conn,
			ObjectPath: path,
			Property:   prop,
			PropList:   ibus.NewPropList(prop),
		},
		IBusEngineState: IBusEngineState{
			LookupTable:        ibus.NewLookupTable(),
			LookupTableVisible: false,
			EnglishMode:        false,
			CandidateDepth:     [8]int{CandCntA, CandCntB, CandCntC, CandCntD, CandCntE, CandCntF, CandCntG, CandCntH},
			CurrCandidateDepth: 0,
		},
	}
}
