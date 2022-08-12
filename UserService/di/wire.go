//go:build wireinject
// +build wireinject

package di

import (
	"UserService/repo"
	"UserService/service"
	"context"

	"github.com/google/wire"
)

var userRepoSet = wire.NewSet(repo.ProvideConnection, repo.ProvideUserRepo)
var roleRepoSet = wire.NewSet(repo.ProvideConnection, repo.ProvideRoleRepo)

func InitializeUserRepo(ctx context.Context) (repo.UserRepo, error) {
	wire.Build(userRepoSet)
	return repo.UserRepo{}, nil
}

func InitializeRoleRepo(ctx context.Context) (repo.RoleRepo, error) {
	wire.Build(roleRepoSet)
	return repo.RoleRepo{}, nil
}

func InitializeUserService() service.UserService {
	wire.Build(repo.ProvideConnection, repo.ProvideRoleRepo, repo.ProvideUserRepo, service.ProvideUserService)
	return service.UserService{}
}
