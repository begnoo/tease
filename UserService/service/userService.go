package service

import (
	"UserService/domain"
	"UserService/errors"
	"UserService/repo"
	"UserService/security"
	"fmt"
)

type UserService struct {
	userRepo *repo.UserRepo
	roleRepo *repo.RoleRepo
}

func ProvideUserService(userRepo repo.UserRepo, roleRepo repo.RoleRepo) UserService {
	return UserService{
		userRepo: &userRepo,
		roleRepo: &roleRepo,
	}
}

func (service *UserService) CreateUser(user domain.User, roleName string) (*domain.User, error) {

	role, err := service.roleRepo.ReadByName(roleName)

	if err != nil {
		return nil, &errors.MissingEntity{
			Message: fmt.Sprintf("Missing role entity with name '%s'", roleName),
		}
	}

	user.Roles = append(user.Roles, *role)

	hashPasswrod, err := security.GeneratehashPassword(user.Password)

	if err != nil {
		return &user, err
	}

	user.Password = hashPasswrod
	new_user, err := service.userRepo.Create(user)

	if new_user == nil {
		return nil, err
	}

	return new_user, errors.NilOrError(err, &errors.RepoError{Err: err})
}

func (service *UserService) ReadAll() (*[]domain.User, error) {
	users, err := service.userRepo.ReadAll()

	return users, errors.NilOrError(err, &errors.RepoError{Err: err})
}
