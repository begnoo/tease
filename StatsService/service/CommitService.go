package service

import (
	"StatsService/domain"
	"StatsService/repo"
	"fmt"

	"go.mongodb.org/mongo-driver/mongo"
)

type CommitService struct {
	commitRepo *repo.CommitRepo
}

func ProvideCommitService(commitRepo repo.CommitRepo) CommitService {
	return CommitService{
		commitRepo: &commitRepo,
	}
}

func (service *CommitService) Create(commit domain.Commit) (*mongo.InsertOneResult, error) {
	res, err := service.commitRepo.Create(commit)

	return res, err
}

func (service *CommitService) CreateCommits(commits []domain.Commit) (*mongo.InsertManyResult, error) {
	newValue := make([]interface{}, len(commits))
	for i, v := range commits {
		newValue[i] = v
	}
	fmt.Printf("\n%+v\n", newValue)
	res, err := service.commitRepo.CreateCommits(newValue)

	return res, err
}

func (service *CommitService) ReadBySource(owner, name string) (*[]domain.CommitCountByDay, error) {
	res, err := service.commitRepo.ReadBySourceGroupByDay(owner, name)

	return res, err
}

func (service *CommitService) ReadBySourceGroupByUser(owner, name string) (*[]domain.CommitCountByUser, error) {
	res, err := service.commitRepo.ReadBySourceGroupByUser(owner, name)

	return res, err
}

func (service *CommitService) ReadByUser(user string) (*[]domain.Commit, error) {
	res, err := service.commitRepo.ReadByUser(user)

	return res, err
}
