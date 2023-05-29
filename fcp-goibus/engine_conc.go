package main

import (
	"github.com/godbus/dbus/v5"
	"github.com/haunt98/goibus/ibus"
)

type State struct {
	preedit        []rune
	candidates     []string
	matchedLen     []int
	ltVisible      bool
	englishMode    bool
	candidateDepth [8]int
}

type DBusState struct {
	conn    *dbus.Conn
	objPath *dbus.ObjectPath
}

type IBusState struct {
	prop     *ibus.Property
	propList *ibus.PropList
}

type EngineState struct {
	now            State
	lt             *ibus.LookupTable
	candidateDepth [8]int
}

type FcpConcEngine struct {
	ibus.Engine
	cp        *CloudPinyin
	dbusState DBusState
	ibusState IBusState
	now       State
	lt        *ibus.LookupTable
}

func NewFcpConcEngine(conn *dbus.Conn, path *dbus.ObjectPath, prop *ibus.Property) *FcpConcEngine {
	return &FcpConcEngine{
		Engine: ibus.BaseEngine(conn, *path),
		cp:     NewCloudPinyin(),
		dbusState: DBusState{
			conn:    conn,
			objPath: path,
		},
		ibusState: IBusState{
			prop:     prop,
			propList: ibus.NewPropList(prop),
		},
		now: State{
			preedit:        []rune{},
			candidates:     []string{},
			matchedLen:     []int{},
			ltVisible:      false,
			englishMode:    false,
			candidateDepth: [8]int{CandCntA, CandCntB, CandCntC, CandCntD, CandCntE, CandCntF, CandCntG, CandCntH},
		},
		lt: ibus.NewLookupTable(),
	}
}

// Merge a state created by call to HandlePinyinInput with IBusEngineState atomically
func (e *FcpConcEngine) mergeStateAtomic(now State) {
}
