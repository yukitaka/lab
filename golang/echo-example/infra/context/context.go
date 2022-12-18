package context

import (
	"echo-example/infra/db"

	"github.com/labstack/echo/v4"
)

type CustomContext struct {
	Db db.Db

	echo.Context
}

func New(ctx echo.Context) *CustomContext {
	return &CustomContext{
		Db:      db.New(),
		Context: ctx,
	}
}
