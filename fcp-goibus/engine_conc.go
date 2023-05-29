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

type EngineState struct {
	NowState       State
	LookupTable    *ibus.LookupTable
	CandidateDepth [8]int
}

type FcpConcEngine struct {
	ibus.Engine
	CloudPinyin     *CloudPinyin
	IBusState       IBusState
	IBusEngineState EngineState
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
		IBusEngineState: EngineState{
			LookupTable:    ibus.NewLookupTable(),
			CandidateDepth: [8]int{CandCntA, CandCntB, CandCntC, CandCntD, CandCntE, CandCntF, CandCntG, CandCntH},
		},
	}
}

// Merge a state created by call to HandlePinyinInput with IBusEngineState atomically
func (e *FcpConcEngine) mergeStateAtomic(now State) {
}
