package context

import (
	"echo-example/infra/db"
	idb "echo-example/interface/db"

	"github.com/labstack/echo/v4"
)

type CustomContext struct {
	db db.Db

	echo.Context
}

func New(ctx echo.Context) *CustomContext {
	return &CustomContext{
		db:      db.New(),
		Context: ctx,
	}
}

func (ctx *CustomContext) Db() idb.Db {
	return ctx.db
}
