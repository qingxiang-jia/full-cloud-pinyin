package main

import (
	"fmt"

	"github.com/godbus/dbus/v5"
	"github.com/haunt98/goibus/ibus"
	"github.com/qingxiang-jia/full-cloud-pinyin/internal/engine"
	fcp "github.com/qingxiang-jia/full-cloud-pinyin/internal/engine"
)

var eid = 0

func Init() {
	bus := ibus.NewBus()

	conn := bus.GetDbusConn()

	ibus.NewFactory(conn, setupEngine)

	bus.RegisterComponent(genEngineComp())

	bus.CallMethod("SetGlobalEngine", 0, "full-cloud-pinyin")

	c := make(chan *dbus.Signal, 10)
	conn.Signal(c)
	select {
	case <-c:
	}
}

func setupEngine(conn *dbus.Conn, engineName string) dbus.ObjectPath {
	// Get object path
	eid++
	objectPath := dbus.ObjectPath(fmt.Sprintf("/org/freedesktop/IBus/Engine/FcPinyin/%d", eid))

	// Generate a FcpEngine
	engine := genEngine(conn, &objectPath)

	// Publish the engine
	ibus.PublishEngine(conn, objectPath, engine)

	// Must return object path
	return objectPath
}

func genEngine(conn *dbus.Conn, path *dbus.ObjectPath) *engine.FcpEngine {
	// Create a menu item
	prop := ibus.NewProperty("setup", ibus.PROP_TYPE_NORMAL, "Preference - Full Cloud Pinyin", "gtk-preferences", "Configure Full Cloud Pinyin Engine", true, true, ibus.PROP_STATE_UNCHECKED)

	// Make an full cloud pinyin engine
	engine := fcp.NewFcpEngine(conn, path, prop)

	return engine
}

func genEngineComp() *ibus.Component {
	comp := ibus.NewComponent(
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

	comp.AddEngine(desc)

	return comp
}
