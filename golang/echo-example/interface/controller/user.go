package controller

import (
	"echo-example/interface/context"
	"echo-example/interface/repository"
	"echo-example/usecase"
	"net/http"
	"strconv"
)

func GetUser(ctx context.Context) error {
	var id int
	id, _ = strconv.Atoi(ctx.Param("id"))
	rep := repository.NewUserRepository(ctx)
	itr := usecase.NewUserInteractor(rep)
	u, _ := itr.FindById(id)

	return ctx.JSON(http.StatusOK, u)
}
