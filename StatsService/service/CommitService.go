package service

import (
	"StatsService/domain"
	"StatsService/repo"
	"fmt"
	"sort"

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

	savedCommits, err := service.commitRepo.ReadBySource(commits[0].Owner, commits[0].Source)
	if err != nil {
		return nil, err
	}
	var toUpdate []domain.Commit
	var toCreate []domain.Commit
	for _, commit := range commits {
		found := false
		for _, savedCommit := range *savedCommits {
			if commit.Sha == savedCommit.Sha {
				toUpdate = append(toUpdate, commit)
				found = true
				break
			}
		}
		if !found {
			toCreate = append(toCreate, commit)
		}
	}
	fmt.Printf("create %+v\n", toCreate)
	fmt.Printf("update %+v\n", toUpdate)

	res, err := service.commitRepo.CreateCommits(toCreate)
	if err != nil && err != mongo.ErrEmptySlice {
		return nil, err
	}

	if commits[0].Branch == "master" {
		_, err = service.commitRepo.UpdateCommits(toUpdate)
		if err != nil && err != mongo.ErrEmptySlice {
			return nil, err
		}
	}

	return res, err
}

func (service *CommitService) ReadBySource(owner, name string) (*[]domain.CommitCountByDay, error) {
	res, err := service.commitRepo.ReadBySourceGroupByDay(owner, name)

	return res, err
}

func (service *CommitService) ReadBySourceGroupByCollabAndDay(owner, name string) (*[]domain.CommitsByUserAndDate, error) {
	res, err := service.commitRepo.ReadBySourceGroupByCollabAndDay(owner, name)
	if err != nil {
		return nil, err
	}
	m := make(map[string][]domain.CommitDateAndCount)

	for _, value := range *res {

		temp_value := domain.CommitDateAndCount{
			Date:    value.ID.Date,
			Count:   value.Count,
			Added:   value.Added,
			Deleted: value.Deleted,
		}

		if array, found := m[value.ID.User]; found {
			m[value.ID.User] = append(array, temp_value)
		} else {
			m[value.ID.User] = []domain.CommitDateAndCount{temp_value}
		}

	}

	var finArr []domain.CommitsByUserAndDate
	for key, element := range m {
		a := domain.CommitsByUserAndDate{
			User:    key,
			Items:   element,
			Added:   getAdded(element),
			Deleted: getDeleted(element),
		}
		finArr = append(finArr, a)
	}

	sort.Slice(finArr, func(i, j int) bool {
		return finArr[i].Added > finArr[j].Added
	})

	return &finArr, err
}

func getAdded(items []domain.CommitDateAndCount) int {
	total := 0
	for _, item := range items {
		total = total + item.Added
	}
	return total
}

func getDeleted(items []domain.CommitDateAndCount) int {
	total := 0
	for _, item := range items {
		total = total + item.Deleted
	}
	return total
}
func (service *CommitService) ReadBySourceGroupByUser(owner, name string) (*[]domain.CommitCountByUser, error) {
	res, err := service.commitRepo.ReadBySourceGroupByUser(owner, name)

	return res, err
}

func (service *CommitService) ReadByUser(user string) (*[]domain.Commit, error) {
	res, err := service.commitRepo.ReadByUser(user)

	return res, err
}
