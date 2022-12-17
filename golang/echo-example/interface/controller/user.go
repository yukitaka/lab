package controller

import (
	"echo-example/interface/context"
	"echo-example/usecase"
	"net/http"
	"strconv"
)

func GetUser(ctx context.Context) error {
	var id int
	id, _ = strconv.Atoi(ctx.Param("id"))
	itr := usecase.NewUserInteractor(ctx)
	u, _ := itr.FindById(id)

	return ctx.JSON(http.StatusOK, u)
}
