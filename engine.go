package main

import (
	"fmt"

	"github.com/godbus/dbus"
	"github.com/haunt98/goibus/ibus"
)

type FcpEngine struct {
	ibus.Engine
	propList *ibus.PropList
}

func (e *FcpEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	fmt.Println("Process Key Event > ", keyVal, keyCode, state)
	if state == 0 && keyVal == 115 {
		e.UpdateAuxiliaryText(ibus.NewText("s"), true)

		lt := ibus.NewLookupTable()
		lt.AppendCandidate("sss")
		lt.AppendCandidate("s")
		lt.AppendCandidate("gittu")
		lt.AppendLabel("1:")
		lt.AppendLabel("2:")
		lt.AppendLabel("3:")

		e.UpdateLookupTable(lt, true)

		e.UpdatePreeditText(ibus.NewText("s"), uint32(1), true)
		return true, nil
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
