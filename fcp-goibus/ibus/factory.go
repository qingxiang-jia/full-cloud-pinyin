package ibus

import (
	"log"

	"github.com/godbus/dbus/v5"
)

// Copied from github.com/haunt98/goibus

type Factory struct {
	conn          *dbus.Conn
	EngineCreator func(conn *dbus.Conn, engineName string) dbus.ObjectPath
}

func NewFactory(conn *dbus.Conn, EngineCreator func(conn *dbus.Conn, engineName string) dbus.ObjectPath) *Factory {
	log.Printf("NewFactory: path=%s interface=%s\n", "/org/freedesktop/IBus/Factory", IBUS_IFACE_ENGINE_FACTORY)

	f := &Factory{conn, EngineCreator}
	conn.Export(f, "/org/freedesktop/IBus/Factory", IBUS_IFACE_ENGINE_FACTORY)
	return f
}

// # Return a array. [name, default_language, icon_path, authors, credits]
// @method(out_signature="as")
// def GetInfo(self): pass

// # Factory should allocate all resources in this method
// @method()
// def Initialize(self): pass

// # Factory should free all allocated resources in this method
// @method()
// def Uninitialize(self): pass

// # Create an input context and return the id of the context.
// # If failed, it will return "" or None.
// @method(in_signature="s", out_signature="o")
func (factory *Factory) CreateEngine(engineName string) (dbus.ObjectPath, *dbus.Error) {
	log.Printf("Factory::CreateEngine: engineName=%s\n", engineName)

	return factory.EngineCreator(factory.conn, engineName), nil
}

// # Destroy the engine
// @method()
func (factory *Factory) Destroy() *dbus.Error {
	log.Printf("Factory::Destroy\n")

	factory.conn.Export(nil, "/org/freedesktop/IBus/Factory", IBUS_IFACE_ENGINE_FACTORY)
	return nil
}