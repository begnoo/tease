package service

import (
	"RepoService/domain"
	"RepoService/errors"
	"RepoService/repo"
	"time"
)

type SourceService struct {
	sourceRepo *repo.SourceRepo
}

func ProvideSourceService(sourceRepo repo.SourceRepo) SourceService {
	return SourceService{
		sourceRepo: &sourceRepo,
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

	err = service.sourceRepo.Delete(id)

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

	collab := domain.Collabarator{
		Name:            collab_email,
		ReactedToInvite: false,
		AcceptedInvite:  false,
		ExpiersAt:       time.Now().Add(time.Hour * 168),
		// jedna nedelja
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
