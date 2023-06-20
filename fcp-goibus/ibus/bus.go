package ibus

import (
	"log"

	"github.com/godbus/dbus/v5"
)

// Copied from github.com/haunt98/goibus
type Bus struct {
	dbusConn   *dbus.Conn
	dbusObject dbus.BusObject
	ibusObject dbus.BusObject
}

func NewBus() *Bus {
	doPanic := func(err error) {
		if err != nil {
			panic(err)
		}
	}
	addr := GetAddress()
	conn, err := dbus.Dial(addr)
	doPanic(err)

	err = conn.Auth(GetUserAuth())
	doPanic(err)

	err = conn.Hello()
	doPanic(err)

	dbusObject := conn.Object(BUS_DAEMON_NAME, dbus.ObjectPath(BUS_DAEMON_PATH))
	ibusObject := conn.Object(IBUS_SERVICE_IBUS, dbus.ObjectPath(IBUS_PATH_IBUS))
	log.Printf("Create DBus Object: destination=%s path=%s\n", BUS_DAEMON_NAME, BUS_DAEMON_PATH)
	log.Printf("Create DBus Object: destination=%s path=%s\n", IBUS_SERVICE_IBUS, IBUS_PATH_IBUS)

	return &Bus{conn, dbusObject, ibusObject}
}

func (bus *Bus) CallMethod(name string, flags dbus.Flags, args ...interface{}) *dbus.Call {
	log.Printf("CallMethod: name=%s at dest=%s\n", name, bus.ibusObject.Destination())
	return bus.ibusObject.Call(bus.ibusObject.Destination()+"."+name, flags, args...)
}

func (bus *Bus) RequestName(name string, flags dbus.RequestNameFlags) (dbus.RequestNameReply, error) {
	log.Printf("RequestName: name=%s\n", name)
	return bus.dbusConn.RequestName(name, flags)
}

func (bus *Bus) RegisterComponent(component *Component) {
	bus.CallMethod("RegisterComponent", 0, dbus.MakeVariant(component))
}

func (bus *Bus) GetDbusConn() *dbus.Conn {
	return bus.dbusConn
}
