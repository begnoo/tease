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
var collabRepoSet = wire.NewSet(repo.ProvideConnection, repo.ProvideCollabRepo)

func InitializeSourceRepo(ctx context.Context) (repo.SourceRepo, error) {
	wire.Build(userRepoSet)
	return repo.SourceRepo{}, nil
}

func InitializeSourceService() service.SourceService {
	wire.Build(repo.ProvideConnection, repo.ProvideSourceRepo, repo.ProvideCollabRepo, service.ProvideSourceService)
	return service.SourceService{}
}

func InitializeCollabRepo(ctx context.Context) (repo.CollabRepo, error) {
	wire.Build(collabRepoSet)
	return repo.CollabRepo{}, nil
}

func InitializeCollabService() service.CollabService {
	wire.Build(repo.ProvideConnection, repo.ProvideCollabRepo, service.ProvideCollabService)
	return service.CollabService{}
}
