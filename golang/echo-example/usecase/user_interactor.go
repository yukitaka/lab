package usecase

import (
	"echo-example/entity"
	"echo-example/interface/context"
)

type UserInteractor struct{}

func NewUserInteractor(ctx context.Context) *UserInteractor {
	return &UserInteractor{}
}

func (itr *UserInteractor) FindById(id int) (*entity.User, error) {
	u := &entity.User{
		Id:    id,
		Name:  "Example",
		Email: "example@example.com",
	}

	return u, nil
}
