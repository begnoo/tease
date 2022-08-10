//go:build wireinject
// +build wireinject

package di

import (
	"UserService/repo"
	"UserService/service"
	"context"
	"log"
	"os"

	"github.com/google/wire"
)

var repoSet = wire.NewSet(ProvideRepoLogger, repo.ProvideConnection, repo.ProvideUserRepo)

func ProvideServiceLogger() *service.ServiceLogger {
	return (*service.ServiceLogger)(log.New(os.Stdout, "USER_SERVICE", 1))
}

func ProvideRepoLogger() *log.Logger {
	return log.New(os.Stdout, "USER_REPO", 1)
}

func InitializeUserRepo(ctx context.Context) (repo.UserRepo, error) {
	wire.Build(repoSet)
	return repo.UserRepo{}, nil
}

func InitializeUserService() service.UserService {
	wire.Build(ProvideServiceLogger, repoSet, service.ProvideUserService)
	return service.UserService{}
}
