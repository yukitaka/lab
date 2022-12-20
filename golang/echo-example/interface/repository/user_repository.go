package repository

import (
	"echo-example/entity"
	"echo-example/interface/context"
)

type UserRepository struct {
	context.Context
}

func NewUserRepository(ctx context.Context) *UserRepository {
	return &UserRepository{
		Context: ctx,
	}
}

func (r *UserRepository) Find(id int) (*entity.User, error) {
	u := &entity.User{
		Id:    id,
		Name:  "Example",
		Email: "example@example.com",
	}

	return u, nil
}
