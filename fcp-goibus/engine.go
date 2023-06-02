package main

import (
	"fmt"

	"github.com/godbus/dbus/v5"
	"github.com/haunt98/goibus/ibus"
)

type FcpEngine struct {
	ibus.Engine
	CloudPinyin *CloudPinyin
	PropList    *ibus.PropList
	Preedit     []rune
	lt          *ibus.LookupTable
	ltVisible   bool
	matchedLen  []int
	enMode      bool
	cpDepth     [8]int
	cpCurDepth  int
}

func NewFcpEngine(conn *dbus.Conn, path *dbus.ObjectPath, prop *ibus.Property) *FcpEngine {
	return &FcpEngine{
		Engine:      ibus.BaseEngine(conn, *path),
		CloudPinyin: NewCloudPinyin(),
		PropList:    ibus.NewPropList(prop),
		Preedit:     []rune{},
		lt:          ibus.NewLookupTable(),
		ltVisible:   false,
		matchedLen:  []int{},
		enMode:      false,
		cpDepth:     [8]int{CandCntA, CandCntB, CandCntC, CandCntD, CandCntE, CandCntF, CandCntG, CandCntH},
		cpCurDepth:  0,
	}
}

func (e *FcpEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	key := rune(keyVal)
	fmt.Println(key, string(key))

	// Decides whether need to switch to/out of English mode
	if state == IBusButtonUp && (key == IBusShiftL || key == IBusShiftR) {
		e.cpCurDepth = 0
		e.enMode = !e.enMode
	}

	if state == IBusButtonDown && !e.enMode {
		// a-z
		if IBusA <= key && key <= IBusZ {
			e.cpCurDepth = 0

			hasHandled := e.HandlePinyinInput(key, AddRune, CandCntA)

			return hasHandled, nil
		}

		if e.ltVisible {
			// Remove a character from preedit
			if key == IBusBackspace {
				e.cpCurDepth = 0

				if len(e.Preedit) == 0 {
					e.HideLt()
					return true, nil
				}

				hasHandled := e.HandlePinyinInput('_', RemoveRune, CandCntA)
				return hasHandled, nil
			}

			// Terminate typing
			if key == IBusEsc {
				e.cpCurDepth = 0

				e.Preedit = e.Preedit[:0]
				e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
				e.HideLt()
				return true, nil
			}

			// Commit preedit as latin
			if key == IBusEnter {
				e.cpCurDepth = 0

				e.CommitText(ibus.NewText(string(e.Preedit)))
				e.Preedit = e.Preedit[:0]
				e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
				e.HideLt()
				return true, nil
			}

			// Commit preedit as Chinese
			if key == IBusSpace {
				e.cpCurDepth = 0

				e.CommitCandidate(int(e.lt.CursorPos))
				return true, nil
			}

			// Commit candidate by keying in candidate index
			if IBus0 <= key && key <= IBus9 {
				idx := int(key) - 48 - 1
				base := int(e.lt.CursorPos / e.lt.PageSize * e.lt.PageSize)
				idx += base
				if 0 <= idx && idx < len(e.lt.Candidates) {
					e.cpCurDepth = 0

					e.CommitCandidate(idx)
				}
				return true, nil
			}

			// Cursor up lookup table
			if key == IBusUp {
				if e.MoveCursorDown() {
					e.UpdateLookupTable(e.lt, true)
				}
				return true, nil
			}

			// Cursor down lookup table
			if key == IBusDown {
				if e.MoveCursorUp() {
					e.UpdateLookupTable(e.lt, true)
				}
				return true, nil
			}

			if key == IBusLeft || key == IBusRight {
				// Currently I don't plan to support moving preedit cursor
				return true, nil
			}

			// + to go to next page
			if key == IBusEqual {
				e.MovePageUp()
				if e.AtLastPage() {
					// We may want to load more candidates
					if e.cpCurDepth < len(e.cpDepth) {
						e.cpCurDepth++
					}
					e.HandlePinyinInput('_', UnchangedRune, e.cpDepth[e.cpCurDepth])
				}
				return true, nil
			}

			// - to go to previous page
			if key == IBusMinus {
				e.MovePageDown()
				return true, nil
			}
		}
	}

	return false, nil
}

func (e *FcpEngine) HandlePinyinInput(key rune, op int, depth int) bool {
	switch op {
	case AddRune:
		e.Preedit = append(e.Preedit, key)
	case RemoveRune:
		e.Preedit = e.Preedit[0 : len(e.Preedit)-1]
	case UnchangedRune:
	default:
		fmt.Println("Not a valid operation")
		return false
	}

	cand, matchedLen, err := e.CloudPinyin.GetCandidates(string(e.Preedit), depth)
	if err != nil {
		fmt.Println(err)
		return false
	}

	e.ClearLt()

	for _, val := range cand {
		e.lt.AppendCandidate(val)
	}
	e.matchedLen = matchedLen

	e.UpdateLookupTable(e.lt, true)
	if op == AddRune || op == RemoveRune {
		e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(len(e.Preedit)), true)
	}
	e.ShowLt()
	// UpdateLookupTable and/or UpdatePreeditText seem to implicitly make lt visible
	// so call it here to keep in sync

	return true
}

// Not sure why the buil-in cursor moving functions don't work so I need to write my own.
// Same for the next three.
func (e *FcpEngine) MoveCursorUp() bool {
	if int(e.lt.CursorPos) == len(e.lt.Candidates) {
		return false
	}
	e.lt.CursorPos++
	return true
}

func (e *FcpEngine) MoveCursorDown() bool {
	if e.lt.CursorPos == 0 {
		return false
	}
	e.lt.CursorPos--
	return true
}

// Workaround, because the IBus side doesn't work.
func (e *FcpEngine) MovePageUp() {
	sz := int(e.lt.PageSize)
	total := len(e.lt.Candidates)
	nextPos := int(e.lt.CursorPos)
	nextPos += sz
	if nextPos >= total {
		nextPos = total - 1
	}
	if nextPos != int(e.lt.CursorPos) {
		e.lt.CursorPos = uint32(nextPos)
		e.UpdateLookupTable(e.lt, true)
	}
}

// Workaround, because the IBus side doesn't work.
func (e *FcpEngine) MovePageDown() {
	sz := e.lt.PageSize
	pos := e.lt.CursorPos
	if pos < sz {
		return
	}
	pos -= sz
	e.lt.CursorPos = pos
	e.UpdateLookupTable(e.lt, true)
}

func (e *FcpEngine) AtLastPage() bool {
	sz := int(e.lt.PageSize)
	total := len(e.lt.Candidates)
	maxIdx := (total/sz+1)*sz - 1
	curIdx := int(e.lt.CursorPos)
	if maxIdx-curIdx < sz {
		return true
	} else {
		return false
	}
}

func (e *FcpEngine) CommitCandidate(i int) {
	if !e.ltVisible {
		return
	}
	text := e.lt.Candidates[i].Value().(ibus.Text)
	e.CommitText(&text)

	if e.matchedLen != nil {
		matchedLen := e.matchedLen[i]
		e.Preedit = e.Preedit[matchedLen:len(e.Preedit)]
	} else {
		e.Preedit = e.Preedit[:0]
	}
	e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
	if len(e.Preedit) == 0 {
		e.HideLt()
		e.ClearLt()
	} else {
		e.HandlePinyinInput('_', UnchangedRune, CandCntA)
	}
}

func (e *FcpEngine) HideLt() {
	e.HideLookupTable()
	e.ltVisible = false
}

func (e *FcpEngine) ShowLt() {
	e.ShowLookupTable()
	e.ltVisible = true
}

func (e *FcpEngine) ClearLt() {
	e.lt.Candidates = e.lt.Candidates[:0]
	e.lt.Labels = e.lt.Labels[:0]
}

// Called when the user clicks a text area
func (e *FcpEngine) FocusIn() *dbus.Error {
	e.RegisterProperties(e.PropList)
	return nil
}

// Called when any of the UI props are called
func (e *FcpEngine) PropertyActivate(prop_name string, prop_state uint32) *dbus.Error {
	return nil
}