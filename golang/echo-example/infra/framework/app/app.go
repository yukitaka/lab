package app

import (
	"echo-example/infra/context"
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

func (app *App) Run(port string) {
	app.Use(func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(c echo.Context) error {
			cc := context.New(c)
			return next(cc)
		}
	})

	router.Build(app.Echo)

	app.Logger.Fatal(app.Start(":" + port))
}
