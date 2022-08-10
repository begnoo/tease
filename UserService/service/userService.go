package service

import (
	"UserService/domain"
	"UserService/repo"
	"log"
	"os"
)

type UserService struct {
	logger *log.Logger
	repo   repo.UserRepo
}

func NewUserService() UserService {
	return UserService{
		logger: log.New(os.Stdout, "USER_SERVICE", 1),
		repo:   repo.NewUserRepo(),
	}
}

func (service *UserService) CreateUser(user domain.User) (domain.User, error) {
	return service.repo.Create(user)
}
