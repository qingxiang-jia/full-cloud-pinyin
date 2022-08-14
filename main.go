package main

import (
	"fmt"

	"github.com/godbus/dbus"
	"github.com/haunt98/goibus/ibus"
)

func main() {
	bus := ibus.NewBus()
	fmt.Println("Got Bus, Running Standalone")

	conn := bus.GetDbusConn()

	ibus.NewFactory(conn, FcpEngineBuilder)

	bus.RegisterComponent(buildComponent())

	bus.CallMethod("SetGlobalEngine", 0, "full-cloud-pinyin")
	fmt.Println("Setting Global Engine to me")

	c := make(chan *dbus.Signal, 10)
	conn.Signal(c)
	select {
	case <-c:
	}
}

var eid = 0

func FcpEngineBuilder(conn *dbus.Conn, engineName string) dbus.ObjectPath {
	eid++
	objectPath := dbus.ObjectPath(fmt.Sprintf("/org/freedesktop/IBus/Engine/FcPinyin/%d", eid))

	// key, ptype, label, icon, tooltip, sensitive, visible, state
	prop := ibus.NewProperty("setup", ibus.PROP_TYPE_NORMAL, "Preference - Full Cloud Pinyin", "gtk-preferences", "Configure Full Cloud Pinyin Engine", true, true, ibus.PROP_STATE_UNCHECKED)

	engine := &FcpEngine{ibus.BaseEngine(conn, objectPath), ibus.NewPropList(prop)}
	ibus.PublishEngine(conn, objectPath, engine)
	return objectPath
}

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

func (e *FcpEngine) FocusIn() *dbus.Error {
	fmt.Println("FocusIn")
	e.RegisterProperties(e.propList)
	return nil
}

func (e *FcpEngine) PropertyActivate(prop_name string, prop_state uint32) *dbus.Error {
	fmt.Println("PropertyActivate", prop_name)
	return nil
}

func buildComponent() *ibus.Component {
	component := ibus.NewComponent(
		"org.freedesktop.IBus.FcPinyin",
		"Full Cloud Pinyin",
		"0.1",
		"MIT",
		"Qingxiang Jia <ybjqx3340@gmail.com>",
		"https://github.com/qingxiang-jia/full-cloud-pinyin",
		"/use/bin/fcpengine",
		"full-cloud-pinyin",
	)

	desc := ibus.SmallEngineDesc(
		"full-cloud-pinyin",
		"Full Cloud Pinyin",
		"Full Cloud Pinyin Engine",
		"en",
		"MIT",
		"Qingxiang Jia <ybjqx3340@gmail.com>",
		"/usr/share/icons/octopi.png",
		"en",
		"/usr/bin/gittupref",
		"2.0",
	)

	component.AddEngine(desc)

	return component
}
