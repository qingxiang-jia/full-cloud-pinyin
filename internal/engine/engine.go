package engine

import (
	"fmt"

	"github.com/godbus/dbus"
	"github.com/haunt98/goibus/ibus"
	cp "github.com/qingxiang-jia/full-cloud-pinyin/internal/cloudPinyin"
	"github.com/qingxiang-jia/full-cloud-pinyin/internal/consts"
)

type FcpEngine struct {
	ibus.Engine
	CloudPinyin cp.CloudPinyin
	PropList    *ibus.PropList
	Preedit     []rune
	lt          *ibus.LookupTable
	ltVisible   bool
	enMode      bool
	cpDepth     [8]int
	cpCurDepth  int
}

func NewFcpEngine(conn *dbus.Conn, path *dbus.ObjectPath, prop *ibus.Property) *FcpEngine {
	return &FcpEngine{
		Engine:      ibus.BaseEngine(conn, *path),
		CloudPinyin: *cp.NewCloudPinyin(),
		PropList:    ibus.NewPropList(prop),
		Preedit:     []rune{},
		lt:          ibus.NewLookupTable(),
		ltVisible:   false,
		enMode:      false,
		cpDepth:     [8]int{consts.CandCntA, consts.CandCntB, consts.CandCntC, consts.CandCntD, consts.CandCntE, consts.CandCntF, consts.CandCntG, consts.CandCntH},
		cpCurDepth:  0,
	}
}

func (e *FcpEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	key := rune(keyVal)
	fmt.Println(key, string(key))

	// Decides whether need to switch to/out of English mode
	if state == consts.IBusButtonUp && (key == consts.IBusShiftL || key == consts.IBusShiftR) {
		e.enMode = !e.enMode
	}

	if state == consts.IBusButtonDown && !e.enMode {
		// a-z
		if consts.IBusA <= key && key <= consts.IBusZ {
			hasHandled := e.HandlePinyinInput(key, consts.AddRune, consts.CandCntA)

			return hasHandled, nil
		}

		if e.ltVisible {
			// Remove a character from preedit
			if key == consts.IBusBackspace {
				if len(e.Preedit) == 0 {
					e.HideLt()
					return true, nil
				}

				hasHandled := e.HandlePinyinInput('_', consts.RemoveRune, consts.CandCntA)
				return hasHandled, nil
			}

			// Terminate typing
			if key == consts.IBusEsc {
				e.Preedit = e.Preedit[:0]
				e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
				e.HideLt()
				return true, nil
			}

			// Commit preedit as latin
			if key == consts.IBusEnter {
				e.CommitText(ibus.NewText(string(e.Preedit)))
				e.Preedit = e.Preedit[:0]
				e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
				e.HideLt()
				return true, nil
			}

			// Commit preedit as Chinese
			if key == consts.IBusSpace {
				e.CommitCandidate(int(e.lt.CursorPos))
				return true, nil
			}

			// Cursor up lookup table
			if key == consts.IBusUp {
				if e.MoveCursorDown() {
					e.UpdateLookupTable(e.lt, true)
				}
				return true, nil
			}

			// Cursor down lookup table
			if key == consts.IBusDown {
				if e.MoveCursorUp() {
					e.UpdateLookupTable(e.lt, true)
				}
				return true, nil
			}

			// + to go to next page
			if key == consts.IBusEqual {
				if !e.MovePageUp() {
					fmt.Println("end reached, cur pos:", e.lt.CursorPos)
				}
				return true, nil
			}

			// - to go to previous page
			if key == consts.IBusMinus {
				e.MovePageDown()
				return true, nil
			}
		}
	}

	return false, nil
}

func (e *FcpEngine) HandlePinyinInput(key rune, op int, depth int) bool {
	switch op {
	case consts.AddRune:
		e.Preedit = append(e.Preedit, key)
	case consts.RemoveRune:
		e.Preedit = e.Preedit[0 : len(e.Preedit)-1]
	case consts.UnchangedRune:
	default:
		fmt.Println("Not a valid operation")
		return false
	}

	cand, err := e.CloudPinyin.GetCandidates(string(e.Preedit), depth)
	if err != nil {
		fmt.Println(err)
		return false
	}

	e.ClearLt()

	for _, val := range cand {
		e.lt.AppendCandidate(val)
	}

	e.UpdateLookupTable(e.lt, true)
	if op == consts.AddRune || op == consts.RemoveRune {
		e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
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
// Return value indicates whether we successfully moved a page.
func (e *FcpEngine) MovePageUp() bool {
	sz := int(e.lt.PageSize)
	max := len(e.lt.Candidates)
	nextPos := int(e.lt.CursorPos)
	nextPos += sz
	if nextPos >= max {
		nextPos = max - 1
	}
	if nextPos == int(e.lt.CursorPos) {
		return false
	} else {
		e.lt.CursorPos = uint32(nextPos)
		e.UpdateLookupTable(e.lt, true)
		return true
	}
}

// Workaround, because the IBus side doesn't work.
func (e *FcpEngine) MovePageDown() bool {
	sz := e.lt.PageSize
	pos := e.lt.CursorPos
	if pos < sz {
		return false
	}
	pos -= sz
	e.lt.CursorPos = pos
	e.UpdateLookupTable(e.lt, true)
	return true
}

func (e *FcpEngine) CommitCandidate(i int) {
	if !e.ltVisible {
		return
	}
	text := e.lt.Candidates[i].Value().(ibus.Text)
	e.CommitText(&text)
	e.Preedit = e.Preedit[:0]
	e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
	e.HideLt()
	e.ClearLt()
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
