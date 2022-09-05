package service

import (
	"StatsService/domain"
	"StatsService/repo"

	"go.mongodb.org/mongo-driver/mongo"
)

type CloneService struct {
	cloneRepo *repo.CloneRepo
}

func ProvideCloneService(cloneRepo repo.CloneRepo) CloneService {
	return CloneService{
		cloneRepo: &cloneRepo,
	}
}

func (service *CloneService) Create(clone domain.Clone) (*mongo.InsertOneResult, error) {
	res, err := service.cloneRepo.Create(clone)

	return res, err
}

func (service *CloneService) ReadBySource(owner, name string) (*[]domain.Clone, error) {
	res, err := service.cloneRepo.ReadBySource(owner, name)

	return res, err
}
