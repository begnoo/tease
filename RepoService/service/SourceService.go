package service

import (
	"RepoService/domain"
	"RepoService/errors"
	"RepoService/repo"
	"os"
	"strconv"
	"time"
)

type SourceService struct {
	sourceRepo *repo.SourceRepo
	collabRepo *repo.CollabRepo
}

func ProvideSourceService(sourceRepo repo.SourceRepo, collabRepo repo.CollabRepo) SourceService {
	return SourceService{
		sourceRepo: &sourceRepo,
		collabRepo: &collabRepo,
	}
}

func (service *SourceService) Create(source domain.Source, requestedBy string) (*domain.Source, error) {
	if requestedBy != source.Owner {
		return nil, &errors.OwnerMismatch{Message: "Mismatch in requested by and source owner."}
	}

	if !VerifyUserExists(requestedBy) {
		return nil, &errors.OwnerMismatch{Message: "User doesn't exist."}
	}

	res, err := service.sourceRepo.Create(source)

	return res, err
}

func (service *SourceService) Delete(id int, requestedBy string) (*domain.Source, error) {
	repo, err := service.sourceRepo.ReadById(id)
	if err != nil {
		return nil, err
	}

	if repo.Owner != requestedBy {
		return nil, &errors.OwnerMismatch{Message: "Mismatch in requested by and source owner."}
	}

	err = service.collabRepo.DeleteBySource(id)
	if err != nil {
		return nil, err
	}

	err = service.sourceRepo.Delete(id)
	if err != nil {
		return nil, err
	}

	return repo, err
}

func (service *SourceService) ReadAll() (*[]domain.Source, error) {
	res, err := service.sourceRepo.Read()

	return res, err
}

func (service *SourceService) ReadById(id int) (*domain.Source, error) {
	res, err := service.sourceRepo.ReadById(id)

	return res, err
}

func (service *SourceService) ReadByOwner(owner string) (*[]domain.Source, error) {
	res, err := service.sourceRepo.ReadByOwner(owner)

	return res, err
}

func (service *SourceService) CollabaratorHasAccess(collab, owner, name string) (bool, error) {
	res, err := service.sourceRepo.ReadByOwnerAndName(owner, name)

	if err != nil {
		return false, err
	}

	if res.Owner == owner {
		return true, nil
	}

	for _, collab := range res.Collabarators {
		if collab.Name == owner {
			return true, nil
		}
	}

	return false, err
}

func (service *SourceService) AddColabarator(sourceId int, collab_email, owner string) (*domain.Collabarator, error) {
	source, err := service.sourceRepo.ReadById(sourceId)
	if err != nil {
		return nil, err
	}

	if source.Owner != owner {
		return nil, &errors.OwnerMismatch{Message: "Posted source id doesn't match requester."}
	}

	if !VerifyUserExists(collab_email) {
		return nil, &errors.OwnerMismatch{Message: "Collabrarator doesn't exist."}
	}

	expires_mult, err := strconv.Atoi(os.Getenv("COLLAB_EXPIRES_IN"))
	collab := domain.Collabarator{
		Name:            collab_email,
		ReactedToInvite: false,
		AcceptedInvite:  false,
		ExpiersAt:       time.Now().Add(time.Hour * time.Duration(expires_mult)),
	}

	for _, v := range source.Collabarators {
		if v.Name == collab_email {
			return &v, &errors.AlreadyThere{Message: "Collabarator already invited."}
		}
	}
	source.Collabarators = append(source.Collabarators, collab)

	service.sourceRepo.Update(*source)

	//dodati slanje mail-a
	return &collab, err
}
