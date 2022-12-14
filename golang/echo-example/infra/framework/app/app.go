package app

import (
	"echo-example/infra/framework/router"

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

func (app *App) Run() {
	router.Build(app.Echo)

	app.Logger.Fatal(app.Start(":1323"))
}
