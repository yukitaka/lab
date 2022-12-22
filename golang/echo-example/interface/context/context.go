package context

import "echo-example/interface/db"

type Context interface {
	Db() db.Db
	JSON(int, interface{}) error
	Param(string) string
}
