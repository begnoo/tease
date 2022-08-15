package service

import (
	"RepoService/domain"
	"RepoService/errors"
	"RepoService/repo"
)

type CollabService struct {
	collabRepo *repo.CollabRepo
}

func ProvideCollabService(collabRepo repo.CollabRepo) CollabService {
	return CollabService{
		collabRepo: &collabRepo,
	}
}

func (service *CollabService) AcceptInvite(id int) (*domain.Collabarator, error) {
	res, err := service.collabRepo.ReadById(id)
	if err != nil {
		return nil, err
	}

	if res.ReactedToInvite {
		return res, &errors.AlreadyThere{Message: "Collab invite already responded to."}
	}

	res.ReactedToInvite = true
	res.AcceptedInvite = true
	service.collabRepo.Update(*res)

	return res, err
}

func (service *CollabService) RejectInvite(id int) (*domain.Collabarator, error) {
	res, err := service.collabRepo.ReadById(id)
	if err != nil {
		return nil, err
	}

	if res.ReactedToInvite {
		return res, &errors.AlreadyThere{Message: "Collab invite already responded to."}
	}

	res.ReactedToInvite = true
	res.AcceptedInvite = false
	service.collabRepo.Update(*res)

	return res, err
}

func (service *CollabService) DeleteExpieredInvitations() bool {
	err := service.collabRepo.DeleteExpiered()

	return err == nil
}

func (service *CollabService) DeleteCollabarator(id int) (*domain.Collabarator, error) {
	res, err := service.collabRepo.ReadById(id)
	if err != nil {
		return nil, err
	}

	err = service.collabRepo.Delete(id)

	return res, err
}
