package model

import (
	"echo-example/entity"

	"gorm.io/gorm"
)

type User struct {
	gorm.Model
	User entity.User `gorm:"emgedded"`
}
