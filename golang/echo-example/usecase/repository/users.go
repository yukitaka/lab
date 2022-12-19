package repository

import "echo-example/entity"

type Users interface {
	Find(int) (*entity.User, error)
}
