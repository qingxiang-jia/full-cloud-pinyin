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
	Preedit            []rune
	LookupTable        *ibus.LookupTable
	LookupTableVisible bool
	EnglishMode        bool
	Candidates         []string
	CandidateDepth     [8]int
	MatchedLength      []int
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

// Merge a state created by call to HandlePinyinInput with IBusEngineState atomically
func (e *FcpConcEngine) mergeStateAtomic(now State) {
}

// Not sure why the buil-in cursor moving functions don't work so I need to write my own.
// Same for the next three.
func (e *FcpConcEngine) moveCursorUp() bool {
	table := e.IBusEngineState.LookupTable
	if int(table.CursorPos) == len(table.Candidates) {
		return false
	}
	table.CursorPos++
	return true
}

func (e *FcpConcEngine) moveCursorDown() bool {
	table := e.IBusEngineState.LookupTable
	if table.CursorPos == 0 {
		return false
	}
	table.CursorPos--
	return true
}

// Workaround, because the IBus side doesn't work.
func (e *FcpConcEngine) movePageUp() {
	table := e.IBusEngineState.LookupTable
	sz := int(table.PageSize)
	total := len(table.Candidates)
	nextPos := int(table.CursorPos)
	nextPos += sz
	if nextPos >= total {
		nextPos = total - 1
	}
	if nextPos != int(table.CursorPos) {
		table.CursorPos = uint32(nextPos)
		e.UpdateLookupTable(table, true)
	}
}

// Workaround, because the IBus side doesn't work.
func (e *FcpConcEngine) movePageDown() {
	table := e.IBusEngineState.LookupTable
	sz := table.PageSize
	pos := table.CursorPos
	if pos < sz {
		return
	}
	pos -= sz
	table.CursorPos = pos
	e.UpdateLookupTable(table, true)
}

func (e *FcpConcEngine) atLastPage() bool {
	table := e.IBusEngineState.LookupTable
	sz := int(table.PageSize)
	total := len(table.Candidates)
	maxIdx := (total/sz+1)*sz - 1
	curIdx := int(table.CursorPos)
	if maxIdx-curIdx < sz {
		return true
	} else {
		return false
	}
}

func (e *FcpConcEngine) commitCandidate(i int) {
	table := e.IBusEngineState.LookupTable
	engineState := e.IBusEngineState
	if !engineState.LookupTableVisible {
		return
	}
	text := table.Candidates[i].Value().(ibus.Text)
	e.CommitText(&text)

	// TODO
	if engineState.MatchedLength != nil {
		matchedLen := engineState.MatchedLength[i]
		engineState.Preedit = engineState.Preedit[matchedLen:len(engineState.Preedit)]
	} else {
		engineState.Preedit = engineState.Preedit[:0]
	}
	e.UpdatePreeditText(ibus.NewText(string(engineState.Preedit)), uint32(1), true)
	if len(engineState.Preedit) == 0 {
		e.hideLookupTable()
		e.clearLookupTable()
	} else {
		// TODO: e.HandlePinyinInput('_', UnchangedRune, CandCntA)
	}
}

func (e *FcpConcEngine) hideLookupTable() {
	e.HideLookupTable()
	e.IBusEngineState.LookupTableVisible = false
}

func (e *FcpConcEngine) showLookupTable() {
	e.ShowLookupTable()
	e.IBusEngineState.LookupTableVisible = true
}

func (e *FcpConcEngine) clearLookupTable() {
	e.IBusEngineState.LookupTable.Candidates = e.IBusEngineState.LookupTable.Candidates[:0]
	e.IBusEngineState.LookupTable.Labels = e.IBusEngineState.LookupTable.Labels[:0]
}

// Called when the user clicks a text area
func (e *FcpConcEngine) FocusIn() *dbus.Error {
	e.RegisterProperties(e.IBusState.PropList)
	return nil
}

// Called when any of the UI props are called
func (e *FcpConcEngine) PropertyActivate(prop_name string, prop_state uint32) *dbus.Error {
	return nil
}
