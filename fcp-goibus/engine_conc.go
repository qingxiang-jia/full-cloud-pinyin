package main

import (
	"fmt"

	"github.com/godbus/dbus/v5"
	"github.com/haunt98/goibus/ibus"
)

type FcpConcEngine struct {
	ibus.Engine
	cloud       *CloudPinyin
	propList    *ibus.PropList
	preedit     []rune
	lt          *ibus.LookupTable
	ltVisible   bool
	matchedLen  []int
	englishMode bool
	level       [8]int
	depth       int
}

func NewFcpConcEngine(conn *dbus.Conn, path *dbus.ObjectPath, prop *ibus.Property) *FcpConcEngine {
	return &FcpConcEngine{
		Engine:      ibus.BaseEngine(conn, *path),
		cloud:       NewCloudPinyin(),
		propList:    ibus.NewPropList(prop),
		preedit:     []rune{},
		lt:          ibus.NewLookupTable(),
		ltVisible:   false,
		matchedLen:  []int{},
		englishMode: false,
		level:       [8]int{CandCntA, CandCntB, CandCntC, CandCntD, CandCntE, CandCntF, CandCntG, CandCntH},
		depth:       0,
	}
}

func (e *FcpConcEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	key := rune(keyVal)
	fmt.Println(key, string(key))

	// Decides whether need to switch to/out of English mode
	if state == IBusButtonUp && (key == IBusShiftL || key == IBusShiftR) {
		e.depth = 0
		e.englishMode = !e.englishMode
	}

	if state == IBusButtonDown && !e.englishMode {
		// a-z
		if IBusA <= key && key <= IBusZ {
			e.depth = 0

			hasHandled := e.handlePinyinInput(key, AddRune, CandCntA)

			return hasHandled, nil
		}

		if e.ltVisible {
			// Remove a character from preedit
			if key == IBusBackspace {
				e.depth = 0

				if len(e.preedit) == 0 {
					e.hideLt()
					return true, nil
				}

				hasHandled := e.handlePinyinInput('_', RemoveRune, CandCntA)
				return hasHandled, nil
			}

			// Terminate typing
			if key == IBusEsc {
				e.depth = 0

				e.preedit = e.preedit[:0]
				e.UpdatePreeditText(ibus.NewText(string(e.preedit)), uint32(1), true)
				e.hideLt()
				return true, nil
			}

			// Commit preedit as latin
			if key == IBusEnter {
				e.depth = 0

				e.CommitText(ibus.NewText(string(e.preedit)))
				e.preedit = e.preedit[:0]
				e.UpdatePreeditText(ibus.NewText(string(e.preedit)), uint32(1), true)
				e.hideLt()
				return true, nil
			}

			// Commit preedit as Chinese
			if key == IBusSpace {
				e.depth = 0

				e.commitCandidate(int(e.lt.CursorPos))
				return true, nil
			}

			// Commit candidate by keying in candidate index
			if IBus0 <= key && key <= IBus9 {
				idx := int(key) - 48 - 1
				base := int(e.lt.CursorPos / e.lt.PageSize * e.lt.PageSize)
				idx += base
				if 0 <= idx && idx < len(e.lt.Candidates) {
					e.depth = 0

					e.commitCandidate(idx)
				}
				return true, nil
			}

			// Cursor up lookup table
			if key == IBusUp {
				if e.moveCursorDown() {
					e.UpdateLookupTable(e.lt, true)
				}
				return true, nil
			}

			// Cursor down lookup table
			if key == IBusDown {
				if e.moveCursorUp() {
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
				e.movePageUp()
				if e.atLastPage() {
					// We may want to load more candidates
					if e.depth < len(e.level) {
						e.depth++
					}
					e.handlePinyinInput('_', UnchangedRune, e.level[e.depth])
				}
				return true, nil
			}

			// - to go to previous page
			if key == IBusMinus {
				e.movePageDown()
				return true, nil
			}
		}
	}

	return false, nil
}

func (e *FcpConcEngine) handlePinyinInput(key rune, op int, depth int) bool {
	switch op {
	case AddRune:
		e.preedit = append(e.preedit, key)
	case RemoveRune:
		e.preedit = e.preedit[0 : len(e.preedit)-1]
	case UnchangedRune:
	default:
		fmt.Println("Not a valid operation")
		return false
	}

	cand, matchedLen, err := e.cloud.GetCandidates(string(e.preedit), depth)
	if err != nil {
		fmt.Println(err)
		return false
	}

	e.clearLt()

	for _, val := range cand {
		e.lt.AppendCandidate(val)
	}
	e.matchedLen = matchedLen

	e.UpdateLookupTable(e.lt, true)
	if op == AddRune || op == RemoveRune {
		e.UpdatePreeditText(ibus.NewText(string(e.preedit)), uint32(len(e.preedit)), true)
	}
	e.showLt()
	// UpdateLookupTable and/or UpdatePreeditText seem to implicitly make lt visible
	// so call it here to keep in sync

	return true
}

// Not sure why the buil-in cursor moving functions don't work so I need to write my own.
// Same for the next three.
func (e *FcpConcEngine) moveCursorUp() bool {
	if int(e.lt.CursorPos) == len(e.lt.Candidates) {
		return false
	}
	e.lt.CursorPos++
	return true
}

func (e *FcpConcEngine) moveCursorDown() bool {
	if e.lt.CursorPos == 0 {
		return false
	}
	e.lt.CursorPos--
	return true
}

// Workaround, because the IBus side doesn't work.
func (e *FcpConcEngine) movePageUp() {
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
func (e *FcpConcEngine) movePageDown() {
	sz := e.lt.PageSize
	pos := e.lt.CursorPos
	if pos < sz {
		return
	}
	pos -= sz
	e.lt.CursorPos = pos
	e.UpdateLookupTable(e.lt, true)
}

func (e *FcpConcEngine) atLastPage() bool {
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

func (e *FcpConcEngine) commitCandidate(i int) {
	if !e.ltVisible {
		return
	}
	text := e.lt.Candidates[i].Value().(ibus.Text)
	e.CommitText(&text)

	if e.matchedLen != nil {
		matchedLen := e.matchedLen[i]
		e.preedit = e.preedit[matchedLen:len(e.preedit)]
	} else {
		e.preedit = e.preedit[:0]
	}
	e.UpdatePreeditText(ibus.NewText(string(e.preedit)), uint32(1), true)
	if len(e.preedit) == 0 {
		e.hideLt()
		e.clearLt()
	} else {
		e.handlePinyinInput('_', UnchangedRune, CandCntA)
	}
}

func (e *FcpConcEngine) hideLt() {
	e.HideLookupTable()
	e.ltVisible = false
}

func (e *FcpConcEngine) showLt() {
	e.ShowLookupTable()
	e.ltVisible = true
}

func (e *FcpConcEngine) clearLt() {
	e.lt.Candidates = e.lt.Candidates[:0]
	e.lt.Labels = e.lt.Labels[:0]
}

// Called when the user clicks a text area
func (e *FcpConcEngine) FocusIn() *dbus.Error {
	e.RegisterProperties(e.propList)
	return nil
}

// Called when any of the UI props are called
func (e *FcpConcEngine) PropertyActivate(prop_name string, prop_state uint32) *dbus.Error {
	return nil
}
