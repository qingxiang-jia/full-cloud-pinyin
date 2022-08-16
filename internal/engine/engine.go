package engine

import (
	"fmt"

	"github.com/godbus/dbus"
	"github.com/haunt98/goibus/ibus"
	"github.com/qingxiang-jia/full-cloud-pinyin/internal/consts"
)

type FcpEngine struct {
	ibus.Engine
	PropList *ibus.PropList
	Preedit  []rune
}

func (e *FcpEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	fmt.Println("Process Key Event > ", keyVal, keyCode, state)

	key := rune(keyVal)
	str := string(key)

	if state == 0 {
		// a-z
		if consts.IBUS_a <= key && key <= consts.IBUS_z {
			lt := ibus.NewLookupTable()
			lt.AppendCandidate(str)
			lt.AppendCandidate(str)
			lt.AppendCandidate(str)
			lt.AppendCandidate(str)
			lt.AppendCandidate(str)
			lt.AppendLabel("1:")
			lt.AppendLabel("2:")
			lt.AppendLabel("3:")
			lt.AppendLabel("4:")
			lt.AppendLabel("5:")

			e.UpdateLookupTable(lt, true)

			e.Preedit = append(e.Preedit, key)
			e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)

			return true, nil
		}

		// Remove a character from preedit
		if key == consts.IBUS_BACKSPACE {
			if len(e.Preedit) == 0 {
				e.HideLookupTable()
				return true, nil
			}

			e.Preedit = e.Preedit[:len(e.Preedit)-1]
			e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)

			return true, nil
		}

		// Terminate typing
		if key == consts.IBUS_ESC || key == consts.IBUS_ENTER {
			e.Preedit = e.Preedit[:0]
			e.UpdatePreeditText(ibus.NewText(string(e.Preedit)), uint32(1), true)
			e.HideLookupTable()
			return true, nil
		}
	}

	return false, nil
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
