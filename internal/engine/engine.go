package engine

import (
	"fmt"

	"github.com/godbus/dbus/v5"
	"github.com/haunt98/goibus/ibus"
	cp "github.com/qingxiang-jia/full-cloud-pinyin/internal/cloudPinyin"
	"github.com/qingxiang-jia/full-cloud-pinyin/internal/consts"
)

type FcpEngine struct {
	*ibus.Engine
	CloudPinyin cp.CloudPinyin
	PropList    *ibus.PropList
	Preedit     []rune
	matchedLen  []int
	enMode      bool
	cpDepth     [8]int
	cpCurDepth  int
	table       *CandidateTable
}

func NewFcpEngine(conn *dbus.Conn, path *dbus.ObjectPath, prop *ibus.Property) *FcpEngine {
	engine := ibus.BaseEngine(conn, *path)
	lt := ibus.NewLookupTable()
	return &FcpEngine{
		Engine:      &engine,
		CloudPinyin: *cp.NewCloudPinyin(),
		PropList:    ibus.NewPropList(prop),
		Preedit:     []rune{},
		matchedLen:  []int{},
		enMode:      false,
		cpDepth:     [8]int{consts.CandCntA, consts.CandCntB, consts.CandCntC, consts.CandCntD, consts.CandCntE, consts.CandCntF, consts.CandCntG, consts.CandCntH},
		cpCurDepth:  0,
		table:       NewCandidateTbale(&engine, lt),
	}
}

func (e *FcpEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	key := rune(keyVal)
	fmt.Println(key, string(key))

	// Decides whether need to switch to/out of English mode
	if state == consts.IBusButtonUp && (key == consts.IBusShiftL || key == consts.IBusShiftR) {
		e.cpCurDepth = 0
		e.enMode = !e.enMode
	}

	if state == consts.IBusButtonDown && !e.enMode {
		// a-z
		if consts.IBusA <= key && key <= consts.IBusZ {
			e.cpCurDepth = 0

			go e.HandlePinyinInput(key, consts.AddRune, consts.CandCntA)

			return true, nil
		}

		if e.table.Visible {
			// Remove a character from preedit
			if key == consts.IBusBackspace {
				e.cpCurDepth = 0

				if len(e.Preedit) == 0 {
					e.table.Hide()
					return true, nil
				}

				go e.HandlePinyinInput('_', consts.RemoveRune, consts.CandCntA)
				return true, nil
			}

			// Terminate typing
			if key == consts.IBusEsc {
				e.cpCurDepth = 0

				e.Preedit = e.Preedit[:0]
				e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
				e.table.Hide()
				return true, nil
			}

			// Commit preedit as latin
			if key == consts.IBusEnter {
				e.cpCurDepth = 0

				e.CommitText(ibus.NewText(string(e.Preedit)))
				e.Preedit = e.Preedit[:0]
				e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
				e.table.Hide()
				return true, nil
			}

			// Commit preedit as Chinese
			if key == consts.IBusSpace {
				e.cpCurDepth = 0

				e.CommitCandidate(e.table.Cursor())
				return true, nil
			}

			// Commit candidate by keying in candidate index
			if consts.IBus0 <= key && key <= consts.IBus9 {
				idx := int(key) - 48 - 1
				base := e.table.Cursor() / e.table.PageSize() * e.table.PageSize()
				idx += base
				if 0 <= idx && idx < len(e.table.lt.Candidates) {
					e.cpCurDepth = 0

					e.CommitCandidate(idx)
				}
				return true, nil
			}

			// Cursor up lookup table
			if key == consts.IBusUp {
				e.table.CursorDown()
				return true, nil
			}

			// Cursor down lookup table
			if key == consts.IBusDown {
				e.table.CursorUp()
				return true, nil
			}

			if key == consts.IBusLeft || key == consts.IBusRight {
				// Currently I don't plan to support moving preedit cursor
				return true, nil
			}

			// + to go to next page
			if key == consts.IBusEqual {
				e.table.PageUp()
				if e.table.AtLastPage() {
					// We may want to load more candidates
					if e.cpCurDepth < len(e.cpDepth) {
						e.cpCurDepth++
					}
					go e.HandlePinyinInput('_', consts.UnchangedRune, e.cpDepth[e.cpCurDepth])
				}
				return true, nil
			}

			// - to go to previous page
			if key == consts.IBusMinus {
				e.table.PageDown()
				return true, nil
			}
		}
	}

	return false, nil
}

func (e *FcpEngine) HandlePinyinInput(key rune, op int, depth int) {
	switch op {
	case consts.AddRune:
		e.Preedit = append(e.Preedit, key)
	case consts.RemoveRune:
		e.Preedit = e.Preedit[0 : len(e.Preedit)-1]
	case consts.UnchangedRune:
	default:
		fmt.Println("Not a valid operation")
	}

	cand, matchedLen, err := e.CloudPinyin.GetCandidates(string(e.Preedit), depth)
	if err != nil {
		fmt.Println(err)
	}

	e.table.Clear()

	e.table.Add(cand)
	e.matchedLen = matchedLen

	if op == consts.AddRune || op == consts.RemoveRune {
		e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(len(e.Preedit)), true)
	}
	e.table.Show()
	// UpdateLookupTable and/or UpdatePreeditText seem to implicitly make lt visible
	// so call it here to keep in sync
}

func (e *FcpEngine) CommitCandidate(i int) {
	if !e.table.Visible {
		return
	}
	text := e.table.lt.Candidates[i].Value().(ibus.Text)
	e.CommitText(&text)

	if e.matchedLen != nil {
		matchedLen := e.matchedLen[i]
		e.Preedit = e.Preedit[matchedLen:len(e.Preedit)]
	} else {
		e.Preedit = e.Preedit[:0]
	}
	e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
	if len(e.Preedit) == 0 {
		e.table.Hide()
		e.table.Clear()
	} else {
		go e.HandlePinyinInput('_', consts.UnchangedRune, consts.CandCntA)
	}
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

func (e *FcpEngine) Enable() *dbus.Error {
	return nil
}

func (e *FcpEngine) Disable() *dbus.Error {
	return nil
}
