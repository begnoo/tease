//go:build wireinject
// +build wireinject

package di

import (
	"StatsService/repo"
	"StatsService/service"

	"github.com/google/wire"
)

func InitializeCommitService() service.CommitService {
	wire.Build(repo.ProvideConnection, repo.ProvideCommitRepo, service.ProvideCommitService)
	return service.CommitService{}
}
