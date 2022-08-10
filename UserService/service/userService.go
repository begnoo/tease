package service

import (
	"UserService/domain"
	"UserService/repo"
	"log"
)

type ServiceLogger log.Logger

type UserService struct {
	logger *ServiceLogger
	repo   *repo.UserRepo
}

func ProvideUserService(userRepo repo.UserRepo, logger *ServiceLogger) UserService {
	return UserService{
		logger: logger,
		repo:   &userRepo,
	}
}

func (service *UserService) CreateUser(user domain.User) (domain.User, error) {
	return service.repo.Create(user)
}

func (service *UserService) ReadAll() ([]domain.User, error) {
	return service.repo.ReadAll()
}
