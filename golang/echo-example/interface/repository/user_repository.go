package repository

import (
	"echo-example/entity"
	"echo-example/interface/context"
	"echo-example/interface/model"
	"fmt"
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
	u := &model.User{
		User: entity.User{
			Id:    id,
			Name:  "Example",
			Email: "example@example.com",
		},
	}
	fmt.Printf("%v", u)

	return &u.User, nil
}
