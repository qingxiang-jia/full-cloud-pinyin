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
}

func NewFcpEngine(conn *dbus.Conn, path *dbus.ObjectPath, prop *ibus.Property) *FcpEngine {
	ibusEngine := ibus.BaseEngine(conn, *path)
	cloudpinyin := cp.NewCloudPinyin()
	propList := ibus.NewPropList(prop)
	preedit := []rune{}
	return &FcpEngine{Engine: ibusEngine, CloudPinyin: *cloudpinyin, PropList: propList, Preedit: preedit}
}

func (e *FcpEngine) ProcessKeyEvent(keyVal uint32, keyCode uint32, state uint32) (bool, *dbus.Error) {
	fmt.Println("Process Key Event > ", keyVal, keyCode, state)

	key := rune(keyVal)

	if state == 0 {
		// a-z
		if consts.IBUS_a <= key && key <= consts.IBUS_z {
			e.Preedit = append(e.Preedit, key)
			cand, err := e.CloudPinyin.GetCandidates(string(e.Preedit), consts.CandCntA)
			if err != nil {
				fmt.Println(err)
				return true, nil
			}

			lt := ibus.NewLookupTable()
			for i, val := range cand {
				lt.AppendCandidate(val)
				lt.AppendLabel(fmt.Sprintf("%d:", i))
			}

			e.UpdateLookupTable(lt, true)
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
