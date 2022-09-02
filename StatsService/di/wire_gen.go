// Code generated by Wire. DO NOT EDIT.

//go:generate go run github.com/google/wire/cmd/wire
//go:build !wireinject
// +build !wireinject

package di

import (
	"StatsService/repo"
	"StatsService/service"
)

// Injectors from wire.go:

func InitializeCommitService() service.CommitService {
	client := repo.ProvideConnection()
	commitRepo := repo.ProvideCommitRepo(client)
	commitService := service.ProvideCommitService(commitRepo)
	return commitService
}
