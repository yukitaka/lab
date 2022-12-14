package router

import (
	"echo-example/entity"
	"net/http"

	"github.com/labstack/echo/v4"
)

func Build(app *echo.Echo) {
	app.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})
	app.GET("/users/:id", getUser)
}

func getUser(c echo.Context) error {
	u := &entity.User{
		Id:    c.Param("id"),
		Name:  "Example2",
		Email: "example@example.com",
	}
	return c.JSON(http.StatusOK, u)
}
