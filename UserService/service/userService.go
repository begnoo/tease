package service

import (
	"UserService/domain"
	"UserService/errors"
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

func (service *UserService) CreateUser(user domain.User) (*domain.User, error) {
	new_user, err := service.repo.Create(user)

	if new_user == nil {
		return nil, err
	}

	return new_user, errors.NilOrError(err, &errors.RepoError{Err: err})
}

func (service *UserService) ReadAll() (*[]domain.User, error) {
	users, err := service.repo.ReadAll()

	return users, errors.NilOrError(err, &errors.RepoError{Err: err})
}
