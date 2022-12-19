package usecase

import (
	"echo-example/entity"
	"echo-example/usecase/repository"
)

type UserInteractor struct {
	repository.Users
}

func NewUserInteractor(repository repository.Users) *UserInteractor {
	return &UserInteractor{
		Users: repository,
	}
}

func (itr *UserInteractor) FindById(id int) (*entity.User, error) {
	return itr.Users.Find(id)
}
