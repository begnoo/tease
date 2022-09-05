package service

import (
	"RepoService/domain"
	"RepoService/errors"
	"RepoService/repo"
	"RepoService/utils"
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
	err = service.collabRepo.Update(*res)
	if err != nil {
		return nil, err
	}

	defer utils.SendAcceptMail(res.From, res.SourceName, res.Name)
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
	err = service.collabRepo.Update(*res)
	if err != nil {
		return nil, err
	}

	defer utils.SendRejectMail(res.From, res.SourceName, res.Name)
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

func (service *CollabService) ReadByName(name string) (*[]domain.Collabarator, error) {
	res, err := service.collabRepo.ReadByName(name)

	return res, err
}
