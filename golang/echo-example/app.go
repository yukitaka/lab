package main

import (
	"echo-example/entity"
	"net/http"

	"github.com/labstack/echo/v4"
)

func main() {
	e := echo.New()
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})
	e.GET("/users/:id", getUser)
	e.Logger.Fatal(e.Start(":1323"))
}

func getUser(c echo.Context) error {
	u := &entity.User{
		Id:    c.Param("id"),
		Name:  "Example2",
		Email: "example@example.com",
	}
	return c.JSON(http.StatusOK, u)
}
