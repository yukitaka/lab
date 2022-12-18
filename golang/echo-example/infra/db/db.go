package db

import (
	"os"

	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

type Db struct {
	Con *gorm.DB
}

func New() Db {
	dsn := "host=" + os.Getenv("DB_HOST") + " user=" + os.Getenv("DB_USER") +
		" password=" + os.Getenv("DB_PASSWORD") + " database=" + os.Getenv("DB_NAME") +
		" port=" + os.Getenv("DB_PORT") + " sslmode=disable TimeZone=Asia/Tokyo"

	con, _ := gorm.Open(postgres.Open(dsn), &gorm.Config{})

	db := Db{
		Con: con,
	}

	return db
}
