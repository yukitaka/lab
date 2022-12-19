package repository

import "echo-example/entity"

type UserRepository struct{}

func NewUserRepository() *UserRepository {
	return &UserRepository{}
}

func (r *UserRepository) Find(id int) (*entity.User, error) {
	u := &entity.User{
		Id:    id,
		Name:  "Example",
		Email: "example@example.com",
	}

	return u, nil
}
