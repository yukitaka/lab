package router

import (
	"echo-example/infra/framework/context"
	"echo-example/interface/controller"

	"github.com/labstack/echo/v4"
)

func userRoute(app *echo.Group) {
	app.GET("/users/:id", func(ctx echo.Context) error {
		return controller.GetUser(ctx.(*context.CustomContext))
	})
}
