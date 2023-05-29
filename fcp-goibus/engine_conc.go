package main

import (
	"fmt"
	"sync"

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
	mu        sync.Mutex
	cp        *CloudPinyin
	dbusState DBusState
	ibusState IBusState
	now       *State
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
		now: &State{
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
		next := State{
			preedit:     []rune{},
			candidates:  []string{},
			matchedLen:  []int{},
			ltVisible:   false,
			englishMode: !e.now.englishMode,
			depth:       0,
		}
		e.applyStateAtomic(&next)
	}

	return true, nil
}

func (e *FcpConcEngine) applyStateAtomic(next *State) {
	e.mu.Lock()
	// Has ltVisible changed? If so, update everything
	if next.ltVisible != e.now.ltVisible {
		e.updatePreedit(&next.preedit, next.ltVisible)
		e.updateLt(&next.candidates, next.ltVisible)
		// IBus doesn't care matchedLen, ltVisible, englishMode, depth, so skip
		e.mu.Unlock()
		return
	}

	// Has depth changed? If so, update candidates, matchedLen
	if next.depth != e.now.depth {
		// IBus
	}

	// Has englishMode changed? If so, update everything

	// Has preedit changed? If so, update IBus with changes on preedit, candidates, matchedLen

}

func (e *FcpConcEngine) updatePreedit(preedit *[]rune, visible bool) {
	e.UpdatePreeditText(ibus.NewText(string(*preedit)), uint32(1), visible)
}

func (e *FcpConcEngine) updateLt(new *[]string, visible bool) {
	e.clearLt()
	for _, candidate := range *new {
		e.lt.AppendCandidate(candidate)
	}
	e.UpdateLookupTable(e.lt, visible)
}

func (e *FcpConcEngine) clearLt() {
	e.lt.Candidates = e.lt.Candidates[:0]
	e.lt.Labels = e.lt.Labels[:0]
}
