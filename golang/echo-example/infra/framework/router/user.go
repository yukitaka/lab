package router

import (
	"echo-example/entity"
	"net/http"

	"github.com/labstack/echo/v4"
)

func userRoute(app *echo.Group) {
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
