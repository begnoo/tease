//go:build wireinject
// +build wireinject

package di

import (
	"RepoService/repo"
	"RepoService/service"
	"context"

	"github.com/google/wire"
)

var userRepoSet = wire.NewSet(repo.ProvideConnection, repo.ProvideSourceRepo)

func InitializeSourceRepo(ctx context.Context) (repo.SourceRepo, error) {
	wire.Build(userRepoSet)
	return repo.SourceRepo{}, nil
}
func InitializeSourceService() service.SourceService {
	wire.Build(repo.ProvideConnection, repo.ProvideSourceRepo, service.ProvideSourceService)
	return service.SourceService{}
}
