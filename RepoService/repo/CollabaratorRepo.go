package repo

import (
	"RepoService/domain"
	"fmt"
	"time"

	"gorm.io/gorm"
)

type CollabRepo struct {
	db *gorm.DB
}

func ProvideCollabRepo(db *gorm.DB) CollabRepo {
	return CollabRepo{
		db: db,
	}
}

func (r *CollabRepo) ReadById(id int) (*domain.Collabarator, error) {
	var collabarator domain.Collabarator
	res := r.db.First(&collabarator, id)

	return &collabarator, res.Error
}

func (repo *CollabRepo) ReadBySource(sourceId int) (*[]domain.Collabarator, error) {
	var collabarators []domain.Collabarator
	res := repo.db.Where(&domain.Collabarator{SourceID: sourceId}).Find(&collabarators)

	return &collabarators, res.Error
}

func (repo *CollabRepo) ReadByName(name string) (*[]domain.Collabarator, error) {
	var collabarators []domain.Collabarator
	res := repo.db.Order("created_at desc").Where(&domain.Collabarator{Name: name}).Find(&collabarators)
	println("rows affected: %d", res.RowsAffected)

	return &collabarators, res.Error
}

func (r *CollabRepo) Delete(id int) error {
	res := r.db.Delete(&domain.Collabarator{}, id)

	return res.Error
}

func (r *CollabRepo) Update(collabarator domain.Collabarator) error {
	res := r.db.Save(&collabarator)

	return res.Error
}

func (r *CollabRepo) DeleteExpiered() error {
	res := r.db.Where(&domain.Collabarator{AcceptedInvite: false}).Where("expiers_at < ?", time.Now()).Delete(&domain.Collabarator{})

	return res.Error
}

func (r *CollabRepo) DeleteBySource(id int) error {
	res := r.db.Where(&domain.Collabarator{SourceID: id}).Delete(&domain.Collabarator{})

	return res.Error
}

func (r *CollabRepo) HandleError(res *gorm.DB) error {
	if res.Error != nil && res.Error != gorm.ErrRecordNotFound {
		err := fmt.Errorf("error: %w", res.Error)
		return err
	}

	return nil
}
