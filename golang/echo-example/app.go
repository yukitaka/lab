package main

import (
	"echo-example/infra/framework/app"
)

func main() {
	app := app.New()
	app.Run()
}
