package repository

import (
	"echo-example/entity"
	"echo-example/interface/context"
)

type UserRepository struct {
	context context.Context
}

func NewUserRepository(ctx context.Context) *UserRepository {
	return &UserRepository{
		context: ctx,
	}
}

func (r *UserRepository) Find(id int) (*entity.User, error) {
	var user = entity.User{}
	r.context.Db().First(&user, id)

	return &user, nil
}
