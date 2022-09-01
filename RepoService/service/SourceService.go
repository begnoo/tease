package service

import (
	"RepoService/domain"
	"RepoService/errors"
	"RepoService/repo"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"

	"gorm.io/gorm"
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
	println(source.Owner)
	println(requestedBy)
	println(source.Owner == requestedBy)

	if requestedBy != source.Owner {
		return nil, &errors.OwnerMismatch{Message: "Mismatch in requested by and source owner."}
	}

	if !VerifyUserExists(requestedBy) {
		return nil, &errors.OwnerMismatch{Message: "User doesn't exist."}
	}

	same_name, err := service.sourceRepo.ReadByOwnerAndName(source.Owner, source.Name)
	if err != nil && err != gorm.ErrRecordNotFound {
		return nil, err
	}

	if same_name.Name == source.Name {
		return nil, &errors.AlreadyThere{Message: fmt.Sprintf("Source with name %s already initialized", source.Name)}
	}

	res, err := service.sourceRepo.Create(source)
	if err != nil {
		return nil, err
	}

	if !CreateSourceInStorage(requestedBy, source.Name) {
		service.Delete(int(res.ID), requestedBy)
	}

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

func (service *SourceService) Search(keyword string) (*[]domain.Source, error) {
	trimmed := strings.Trim(keyword, " ")
	if trimmed == "" {
		return service.ReadAll()
	}
	res, err := service.sourceRepo.Search(trimmed)

	return res, err
}

func (service *SourceService) GetCollabarators(owner, name string) (*[]domain.Collabarator, error) {
	res, err := service.sourceRepo.ReadByOwnerAndName(owner, name)

	if err != nil {
		return nil, err
	}

	collabs := res.Collabarators

	return &collabs, err
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

func (service *SourceService) AddColabarator(collab_email, owner, name, sent_by string) (*domain.Collabarator, error) {
	source, err := service.sourceRepo.ReadByOwnerAndName(owner, name)
	if err != nil {
		return nil, err
	}

	if collab_email == owner {
		return nil, &errors.OwnerMismatch{Message: "Can't add self as collabalator."}
	}

	if source.Owner != sent_by {
		return nil, &errors.OwnerMismatch{Message: "Posted source id doesn't match requester."}
	}

	if !VerifyUserExists(collab_email) {
		return nil, &errors.OwnerMismatch{Message: "Collabrarator doesn't exist."}
	}

	expires_mult, err := strconv.Atoi(os.Getenv("COLLAB_EXPIRES_IN"))
	collab := domain.Collabarator{
		Name:            collab_email,
		SourceName:      source.Name,
		From:            source.Owner,
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
