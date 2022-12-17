package main

import (
	"echo-example/infra/framework/app"
	"os"

	"github.com/joho/godotenv"
)

func main() {
	loadEnv()

	app := app.New()
	app.Run(port())
}

func loadEnv() {
	env := os.Getenv("echo-example")
	if env == "" {
		env = "development"
	}
	godotenv.Load(".env." + env + ".local")
	if env != "test" {
		godotenv.Load(".env.local")
	}
	godotenv.Load(".env." + env)
	godotenv.Load()
}

func port() string {
	if port, ok := os.LookupEnv("APP_PORT"); ok {
		return port
	} else {
		return "1323"
	}
}
