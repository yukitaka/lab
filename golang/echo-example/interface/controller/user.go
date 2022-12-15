package controller

import (
	"echo-example/entity"
	"echo-example/interface/context"
	"net/http"
)

func GetUser(ctx context.Context) error {
	u := &entity.User{
		Id:    ctx.Param("id"),
		Name:  "Example",
		Email: "example@example.com",
	}
	return ctx.JSON(http.StatusOK, u)
}
