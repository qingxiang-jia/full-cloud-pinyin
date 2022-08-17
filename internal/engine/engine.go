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
}

func NewFcpEngine(conn *dbus.Conn, path *dbus.ObjectPath, prop *ibus.Property) *FcpEngine {
	ibusEngine := ibus.BaseEngine(conn, *path)
	cloudpinyin := cp.NewCloudPinyin()
	propList := ibus.NewPropList(prop)
	preedit := []rune{}
	lt := ibus.NewLookupTable()
	return &FcpEngine{Engine: ibusEngine, CloudPinyin: *cloudpinyin, PropList: propList, Preedit: preedit, lt: lt}
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

			return true, nil
		}

		// Remove a character from preedit
		if key == consts.IBusBackspace {
			if len(e.Preedit) == 0 {
				e.HideLookupTable()
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
			e.HideLookupTable()
			return true, nil
		}

		// Commit preedit as latin
		if key == consts.IBusEnter {
			e.CommitText(ibus.NewText(string(e.Preedit)))
			e.Preedit = e.Preedit[:0]
			e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
			e.HideLookupTable()
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
			e.PageDownLookupTable()
			return true, nil
		}
	}

	return false, nil
}

// Not sure why the buil-in cursor moving functions don't work so I need to write my own. Same for the next three.
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

func (e *FcpEngine) MovePageDown() bool {
	return false
}

func (e *FcpEngine) CommitCandidate(i int) {
	text := e.lt.Candidates[i].Value().(ibus.Text)
	e.CommitText(&text)
	e.Preedit = e.Preedit[:0]
	e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
	e.HideLookupTable()
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
