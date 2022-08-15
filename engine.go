package main

import (
	"fmt"

	"github.com/godbus/dbus"
	"github.com/haunt98/goibus/ibus"
)

type FcpEngine struct {
	ibus.Engine
	propList *ibus.PropList
	preedit  []rune
}

func (e *FcpEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	fmt.Println("Process Key Event > ", keyVal, keyCode, state)

	key := rune(keyVal)
	str := string(key)

	if state == 0 {
		// a-z
		if IBUS_a <= key && key <= IBUS_z {
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

			e.preedit = append(e.preedit, key)
			e.UpdatePreeditText(ibus.NewText(string(e.preedit)), uint32(1), true)

			return true, nil
		}

		// Remove a character from preedit
		if key == IBUS_BACKSPACE {
			if len(e.preedit) == 0 {
				e.HideLookupTable()
				return true, nil
			}

			e.preedit = e.preedit[:len(e.preedit)-1]
			e.UpdatePreeditText(ibus.NewText(string(e.preedit)), uint32(1), true)

			return true, nil
		}

		// Terminate typing
		if key == IBUS_ESC || key == IBUS_ENTER {
			e.preedit = e.preedit[:0]
			e.UpdatePreeditText(ibus.NewText(string(e.preedit)), uint32(1), true)
			e.HideLookupTable()
			return true, nil
		}
	}

	return false, nil
}

// Called when the user clicks a text area
func (e *FcpEngine) FocusIn() *dbus.Error {
	fmt.Println("FocusIn")
	e.RegisterProperties(e.propList)
	return nil
}

// Called when any of the UI props are called
func (e *FcpEngine) PropertyActivate(prop_name string, prop_state uint32) *dbus.Error {
	fmt.Println("PropertyActivate", prop_name)
	return nil
}
