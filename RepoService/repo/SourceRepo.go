package repo

import (
	"RepoService/domain"
	"fmt"

	"gorm.io/gorm"
)

type SourceRepo struct {
	db *gorm.DB
}

func ProvideSourceRepo(db *gorm.DB) SourceRepo {
	return SourceRepo{
		db: db,
	}
}

func (r *SourceRepo) ReadByName(name string) (*domain.Source, error) {
	var source domain.Source
	res := r.db.Where(&domain.Source{Name: name}).First(&source)

	return &source, res.Error
}

func (r *SourceRepo) ReadById(id int) (*domain.Source, error) {
	var source domain.Source
	res := r.db.Preload("Collabarators").First(&source, id)

	return &source, res.Error
}

func (repo *SourceRepo) ReadByOwner(owner string) (*[]domain.Source, error) {
	var sources []domain.Source
	res := repo.db.Where(&domain.Source{Owner: owner}).Find(&sources)

	return &sources, res.Error
}

func (r *SourceRepo) Read() (*[]domain.Source, error) {
	var sources []domain.Source
	res := r.db.Find(&sources)

	return &sources, res.Error
}

func (r *SourceRepo) Create(repo domain.Source) (*domain.Source, error) {
	res := r.db.Create(&repo)

	return &repo, r.HandleError(res)
}

func (r *SourceRepo) Delete(id int) error {
	res := r.db.Delete(&domain.Source{}, id)

	return res.Error
}

func (r *SourceRepo) Update(source domain.Source) error {
	res := r.db.Save(&source)

	return res.Error
}

func (repo *SourceRepo) ReadByOwnerAndName(owner, name string) (*domain.Source, error) {
	var source domain.Source
	res := repo.db.Preload("Collabarators").Where(&domain.Source{Owner: owner, Name: name}).First(&source)

	return &source, res.Error
}

func (repo *SourceRepo) Search(keyword string) (*[]domain.Source, error) {
	var sources []domain.Source
	res := repo.db.Where("name like ?", "%"+keyword+"%").Find(&sources)

	return &sources, res.Error
}

func (r *SourceRepo) HandleError(res *gorm.DB) error {
	if res.Error != nil && res.Error != gorm.ErrRecordNotFound {
		err := fmt.Errorf("error: %w", res.Error)
		return err
	}

	return nil
}
