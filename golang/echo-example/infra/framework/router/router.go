package router

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func Build(app *echo.Echo) {
	api := app.Group("/api")
	app.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})

	userRoute(api)
}
