package main

import (
	"fmt"

	"github.com/godbus/dbus/v5"
	"github.com/haunt98/goibus/ibus"
)

type State struct {
	preedit     []rune
	candidates  []string
	matchedLen  []int
	ltVisible   bool
	englishMode bool
	depth       int
}

type DBusState struct {
	conn    *dbus.Conn
	objPath *dbus.ObjectPath
}

type IBusState struct {
	prop     *ibus.Property
	propList *ibus.PropList
}

type FcpConcEngine struct {
	ibus.Engine
	cp        *CloudPinyin
	dbusState DBusState
	ibusState IBusState
	now       State
	level     [8]int
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
			preedit:     []rune{},
			candidates:  []string{},
			matchedLen:  []int{},
			ltVisible:   false,
			englishMode: false,
			depth:       0,
		},
		level: [8]int{CandCntA, CandCntB, CandCntC, CandCntD, CandCntE, CandCntF, CandCntG, CandCntH},
		lt:    ibus.NewLookupTable(),
	}
}

func (e *FcpConcEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	key := rune(keyVal)
	fmt.Println(key, string(key))

	// Decides if we need to switch to or out of English mode
	if state == IBusButtonUp && (key == IBusShiftL || key == IBusShiftR) {

	}

	return true, nil
}

// Merge a state created by call to HandlePinyinInput with IBusEngineState atomically
func (e *FcpConcEngine) mergeStateAtomic(now State) {
}
