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
}

func NewFcpEngine(conn *dbus.Conn, path *dbus.ObjectPath, prop *ibus.Property) *FcpEngine {
	return &FcpEngine{
		Engine:      ibus.BaseEngine(conn, *path),
		CloudPinyin: *cp.NewCloudPinyin(),
		PropList:    ibus.NewPropList(prop),
		Preedit:     []rune{},
		lt:          ibus.NewLookupTable(),
		ltVisible:   false,
	}
}

func (e *FcpEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	fmt.Println("Process Key Event > ", keyVal, keyCode, state)

	key := rune(keyVal)

	if state == 0 {
		// a-z
		if consts.IBusA <= key && key <= consts.IBusZ {
			e.Preedit = append(e.Preedit, key)
			cand, err := e.CloudPinyin.GetCandidates(string(e.Preedit), consts.CandCntA)
			if err != nil {
				fmt.Println(err)
				return true, nil
			}

			// Clear look up table before use
			e.lt.Candidates = e.lt.Candidates[:0]
			e.lt.Labels = e.lt.Labels[:0]

			for i, val := range cand {
				e.lt.AppendCandidate(val)
				e.lt.AppendLabel(fmt.Sprintf("%d:", i))
			}

			e.UpdateLookupTable(e.lt, true)
			e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
			e.ShowLt()
			// UpdateLookupTable and/or UpdatePreeditText seem to implicitly make lt visible
			// so call it here to keep in sync

			return true, nil
		}

		// Remove a character from preedit
		if key == consts.IBusBackspace {
			if len(e.Preedit) == 0 {
				e.HideLt()
				return true, nil
			}

			e.Preedit = e.Preedit[:len(e.Preedit)-1]
			e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)

			return true, nil
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
			e.MovePageUp()
			fmt.Println("cur pos:", e.lt.CursorPos)
			return true, nil
		}

		// - to go to previous page
		if key == consts.IBusMinus {
			e.MovePageDown()
			fmt.Println("cur pos:", e.lt.CursorPos)
			return true, nil
		}
	}

	return false, nil
}

// Not sure why the buil-in cursor moving functions don't work so I need to write my own.
// Same for the next three.
func (e *FcpEngine) MoveCursorUp() bool {
	if int(e.lt.CursorPos) == len(e.lt.Candidates) {
		return false
	}
	e.lt.CursorPos++
	fmt.Println("cur pos:", e.lt.CursorPos)
	return true
}

func (e *FcpEngine) MoveCursorDown() bool {
	if e.lt.CursorPos == 0 {
		return false
	}
	e.lt.CursorPos--
	fmt.Println("cur pos:", e.lt.CursorPos)
	return true
}

// Move the page up by moving the cursor to the lowest index of the next page.
// We use this because the page down signal isn't working.
func (e *FcpEngine) MovePageUp() bool {
	sz := int(e.lt.PageSize)
	max := len(e.lt.Candidates)
	curPageMin := int(e.lt.CursorPos) / sz * sz
	nextPos := curPageMin + sz
	if nextPos >= max {
		nextPos = max
	}
	if nextPos == int(e.lt.CursorPos) {
		return false
	} else {
		e.lt.CursorPos = uint32(nextPos)
		e.UpdateLookupTable(e.lt, true)
	}
	return true
}

// Move the page down by moving the cursor to the lowest index of the previous page.
// We use this because the page up signal isn't working.
func (e *FcpEngine) MovePageDown() bool {
	sz := e.lt.PageSize
	pos := e.lt.CursorPos
	fmt.Println("before: sz:", sz, "pos", pos)
	if pos < sz {
		return false
	}
	pos = (pos/sz - 1) * sz
	e.lt.CursorPos = pos
	e.UpdateLookupTable(e.lt, true)
	return true
}

func (e *FcpEngine) CommitCandidate(i int) {
	text := e.lt.Candidates[i].Value().(ibus.Text)
	e.CommitText(&text)
	e.Preedit = e.Preedit[:0]
	e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
	e.HideLt()
}

func (e *FcpEngine) HideLt() {
	e.HideLookupTable()
	e.ltVisible = false
}

func (e *FcpEngine) ShowLt() {
	e.ShowLookupTable()
	e.ltVisible = true
}

// Called when the user clicks a text area
func (e *FcpEngine) FocusIn() *dbus.Error {
	fmt.Println("FocusIn")
	e.RegisterProperties(e.PropList)
	return nil
}

// Called when any of the UI props are called
func (e *FcpEngine) PropertyActivate(prop_name string, prop_state uint32) *dbus.Error {
	fmt.Println("PropertyActivate", prop_name)
	return nil
}
