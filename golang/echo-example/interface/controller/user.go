package controller

import (
	"echo-example/interface/context"
	"echo-example/usecase"
	"net/http"
)

func GetUser(ctx context.Context) error {
	itr := usecase.NewUserInteractor(ctx)
	u, _ := itr.FindById(ctx.Param("id"))

	return ctx.JSON(http.StatusOK, u)
}
