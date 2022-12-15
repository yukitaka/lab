package context

import (
	"github.com/labstack/echo/v4"
)

type CustomContext struct {
	echo.Context
}

func New(ctx echo.Context) *CustomContext {
	return &CustomContext{ctx}
}
