package main

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

type User struct {
	Id    string `json:"id"`
	Name  string `json:"name"`
	Email string `json:"email"`
}

func main() {
	e := echo.New()
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})
	e.GET("/users/:id", getUser)
	e.Logger.Fatal(e.Start(":1323"))
}

func getUser(c echo.Context) error {
	u := &User{
		Id:    c.Param("id"),
		Name:  "Example",
		Email: "example@example.com",
	}
	return c.JSON(http.StatusOK, u)
}
