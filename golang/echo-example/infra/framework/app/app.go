package app

import (
	"github.com/labstack/echo/v4"
)

type App struct {
	*echo.Echo
}

func New() *App {
	return &App{
		Echo: echo.New(),
	}
}
